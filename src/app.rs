use crate::{action::Action, handlers, ui};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, Signal, System};

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
    pub selected: usize,
    pub cpu_history: VecDeque<f64>,
    pub memory_history: VecDeque<f64>,
    pub processes: Vec<ProcessInfo>,
    pub table_state: TableState,
    pub last_process_refresh: Instant,
    pub search_query: String,
    pub search_mode: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            sys: System::new_all(),
            selected: 0,
            cpu_history: VecDeque::new(),
            memory_history: VecDeque::new(),
            processes: vec![],
            table_state: TableState::new(),
            last_process_refresh: Instant::now(),
            search_query: String::new(),
            search_mode: false,
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
                self.handle_action()?;
            }
        }

        Ok(())
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,

            Action::MoveUp => self.move_up(),

            Action::MoveDown => self.move_down(),

            Action::KillProcess => {
                self.kill_selected_process();
            }

            Action::Refresh => {
                self.sys.refresh_all();
            }

            Action::StartSearch => {
                self.search_mode = true;
                self.search_query.clear(); // Optional
            }

            Action::StopSearch => {
                self.search_mode = false;
            }

            Action::None => {}
        }
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

    fn update_process_table(&mut self) {
        if self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        }
    }

    fn kill_selected_process(&mut self) {
        let Some(selected) = self.table_state.selected() else {
            return;
        };

        let Some(info) = self.processes.get(selected) else {
            return;
        };

        if let Some(process) = self.sys.process(info.pid) {
            process.kill_with(Signal::Term);

            // Refresh the process list afterwards
            self.update_processes();
        }
    }

    pub fn move_down(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);

        if selected + 1 < self.processes.len() {
            self.table_state.select(Some(selected + 1));
        }
    }

    pub fn move_up(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);

        if selected > 0 {
            self.table_state.select(Some(selected - 1));
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::draw(frame, self)
    }

    fn handle_action(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if self.search_mode {
                self.handle_search_input(key);
                return Ok(());
            }

            let action = handlers::key_to_action(key);
            self.update(action);
        }

        Ok(())
    }
    fn handle_search_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.search_mode = false;
            }

            KeyCode::Enter => {
                self.search_mode = false;
            }

            KeyCode::Backspace => {
                self.search_query.pop();
            }

            KeyCode::Char(c) => {
                self.search_query.push(c);
            }

            _ => {}
        }
    }
}
