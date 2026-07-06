use crate::command::InputMode;
use crate::{handlers, ui};
use crossterm::event::{self};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Debug)]
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
    pub processes: Vec<ProcessInfo>,
    pub filtered_processes: Vec<usize>,
    pub table_state: TableState,
    pub last_process_refresh: Instant,
    pub search_query: String,
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
            processes: Vec::new(),
            filtered_processes: Vec::new(),
            table_state: TableState::new(),
            last_process_refresh: Instant::now(),
            search_query: String::new(),
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

        self.processes = self
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

        self.processes
            .sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal));

        self.update_process_table();

        self.update_filtered_processes();
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

    pub fn update_filtered_processes(&mut self) {
        let query = self.search_query.to_lowercase();

        self.filtered_processes = self
            .processes
            .iter()
            .enumerate()
            .filter(|(_, p)| query.is_empty() || p.name.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();

        let selected = self.table_state.selected().unwrap_or(0);

        if self.filtered_processes.is_empty() {
            self.table_state.select(None);
        } else if selected >= self.filtered_processes.len() {
            self.table_state.select(Some(0));
        }
    }

    fn update_process_table(&mut self) {
        if self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        }
    }
    fn draw(&mut self, frame: &mut Frame) {
        ui::draw(frame, self)
    }
}
