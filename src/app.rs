use crate::command::InputMode;
use crate::{handlers, ui};
use crossterm::event::{self};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: sysinfo::Pid,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub status: String,
}

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    pub sys: System,
    pub cpu_history: VecDeque<f64>,
    pub memory_history: VecDeque<f64>,
    pub all_processes: Vec<ProcessInfo>,
    pub processes: Vec<ProcessInfo>,
    pub table_state: TableState,
    pub last_process_refresh: Instant,
    pub command: String,
    pub mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            sys: System::new_all(),
            cpu_history: VecDeque::new(),
            memory_history: VecDeque::new(),
            all_processes: Vec::new(),
            processes: Vec::new(),
            table_state: TableState::new(),
            last_process_refresh: Instant::now(),
            command: String::new(),
            mode: InputMode::Normal,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            if self.last_process_refresh.elapsed() >= Duration::from_secs(1) {
                self.update_processes();
                self.last_process_refresh = Instant::now();
            }

            self.update_cpu();
            self.update_memory();

            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(200))? {
                handlers::handle_action(self)?;
            }
        }

        Ok(())
    }

    pub fn update_processes(&mut self) {
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        self.all_processes = self
            .sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: *pid,
                name: process.name().to_string_lossy().into_owned(),
                cpu: process.cpu_usage(),
                memory: process.memory(),
                status: format!("{:?}", process.status()),
            })
            .collect();

        self.all_processes
            .sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal));

        self.apply_filter();

        if self.table_state.selected().is_none() && !self.processes.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    fn update_cpu(&mut self) {
        self.sys.refresh_cpu_usage();
        let cpu = self.sys.global_cpu_usage() as f64;
        self.cpu_history.push_back(cpu);
        if self.cpu_history.len() > 100 {
            self.cpu_history.pop_front();
        }
    }

    fn update_memory(&mut self) {
        self.sys.refresh_memory();

        let memory = self.sys.used_memory() as f64 / self.sys.total_memory() as f64 * 100.0;

        self.memory_history.push_back(memory);

        if self.memory_history.len() > 100 {
            self.memory_history.pop_front();
        }
    }

    pub fn apply_filter(&mut self) {
        if matches!(self.mode, InputMode::Command) {
            self.processes = self.all_processes.clone();
            return;
        }

        let query = self.command.to_lowercase();

        self.processes = self
            .all_processes
            .iter()
            .filter(|p| query.is_empty() || p.name.to_lowercase().contains(&query))
            .cloned()
            .collect();

        if self.processes.is_empty() {
            self.table_state.select(None);
        } else {
            let selected = self.table_state.selected().unwrap_or(0);

            if selected >= self.processes.len() {
                self.table_state.select(Some(0));
            }
        }
    }
    fn draw(&mut self, frame: &mut Frame) {
        ui::draw(frame, self)
    }
}
