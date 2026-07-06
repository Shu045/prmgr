use crate::{
    app::App,
    command::{Command, InputMode},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

pub fn handle_action(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Up => {
                move_up(app);
                return Ok(());
            }
            KeyCode::Down => {
                move_down(app);
                return Ok(());
            }
            _ => {}
        }

        match app.mode {
            InputMode::Normal => handle_normal_input(app, key),
            InputMode::Command => handle_command_input(app, key),
        }
    }

    Ok(())
}

fn handle_normal_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') if app.command.is_empty() => app.exit = true,

        KeyCode::Char(':') if app.command.is_empty() => {
            app.mode = InputMode::Command;
            app.command.push(':');
            app.apply_filter();
        }

        KeyCode::Char(c) => {
            app.command.push(c);
            app.apply_filter();
        }

        KeyCode::Backspace => {
            if !app.command.is_empty() {
                app.command.pop();
                app.apply_filter();
            }
        }

        _ => {}
    }
}

fn handle_command_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => app.command.push(c),

        KeyCode::Backspace => {
            app.command.pop();

            if app.command.is_empty() {
                leave_command_mode(app);
            }
        }

        KeyCode::Enter => {
            app.command = app.command.trim_start_matches(':').to_string();
            execute_command(app);
            leave_command_mode(app);
        }

        KeyCode::Tab => {
            if let Some(selected) = app.table_state.selected() {
                if let Some(process) = app.processes.get(selected) {
                    app.command.push(' ');
                    app.command.push_str(&process.pid.to_string());
                }
            }
        }

        KeyCode::Esc => leave_command_mode(app),

        _ => {}
    }
}

fn leave_command_mode(app: &mut App) {
    app.command.clear();
    app.mode = InputMode::Normal;
    app.apply_filter();
}

fn execute_command(app: &mut App) {
    let command_line = app.command.clone();

    let mut parts = command_line.split_whitespace();

    let command: &str = parts.next().unwrap_or("");
    let argument: &str = parts.next().unwrap_or("");

    match Command::from_str(command) {
        Command::Kill => kill_process(app, argument),
        Command::Search | Command::Unknown => {}
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
