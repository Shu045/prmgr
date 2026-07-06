use crate::{
    app::App,
    command::{Command, InputMode},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

pub fn handle_action(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match app.mode {
            InputMode::Normal => handle_normal_input(app, key),
            InputMode::Command => handle_command_input(app, key),
        }
    }
    Ok(())
}

fn handle_normal_input(app: &mut App, key: KeyEvent) {
    match key.code {
        event::KeyCode::Char('q') => app.exit = true,
        event::KeyCode::Up => move_up(app),
        event::KeyCode::Down => move_down(app),
        event::KeyCode::Char(':') => {
            app.mode = InputMode::Command;
            app.command.clear();
        }

        _ => {}
    }
}
fn handle_command_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.command.push(c);
        }
        KeyCode::Backspace => {
            app.command.pop();
        }

        KeyCode::Enter => {
            execute_command(app);
            app.mode = InputMode::Command;
            app.command.clear();
        }

        KeyCode::Esc => {
            app.command.clear();
            app.mode = InputMode::Normal;
        }

        _ => {}
    }
}

fn execute_command(app: &mut App) {
    let command_line = app.command.clone();

    let mut parts = command_line.splitn(2, ' ');

    let command: &str = parts.next().unwrap_or("");
    let argument: &str = parts.next().unwrap_or("");

    match Command::from_str(command) {
        Command::Search => {
            app.search_query = argument.to_string();
            app.update_filtered_processes();
            app.filtered_processes.clear();
        }

        Command::Kill => kill_process(app, argument),

        Command::Unknown => {}
    }
}

fn move_up(app: &mut App) {
    let selected = app.table_state.selected().unwrap_or(0);

    if selected > 0 {
        app.table_state.select(Some(selected - 1));
    }
}

fn move_down(app: &mut App) {
    let selected = app.table_state.selected().unwrap_or(0);

    if selected + 1 < app.processes.len() {
        app.table_state.select(Some(selected + 1));
    }
}
fn kill_process(app: &mut App, pid_str: &str) {
    let Ok(pid_num) = pid_str.parse::<usize>() else {
        return;
    };

    let pid = sysinfo::Pid::from(pid_num);

    if let Some(process) = app.sys.process(pid) {
        process.kill();
    }

    app.update_processes();
}
