use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Paragraph, Row, Table},
};

use crate::{
    app::{self},
    command::InputMode,
};

pub fn draw(frame: &mut Frame, area: Rect, app: &mut app::App) {
    let block = Block::bordered()
        .title("Processes")
        .title_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);

    let [header_area, table_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(inner);

    let header = Row::new(["PID", "NAME", "CPU%", "MEM", "STATUS"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows = app.filtered_processes.iter().map(|&i| {
        let p = &app.processes[i];

        Row::new([
            p.pid.to_string(),
            p.name.clone(),
            format!("{:.1}", p.cpu),
            format!("{:.1} MB", p.memory as f64 / 1024.0),
            p.status.clone(),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Percentage(50),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(Block::bordered().border_type(BorderType::Rounded))
    .row_highlight_style(Style::default().bg(Color::Green).fg(Color::White))
    .highlight_symbol("▶ ");

    let text = if matches!(app.mode, InputMode::Command) {
        app.command.as_str()
    } else {
        app.search_query.as_str()
    };

    let search = Paragraph::new(text).block(
        Block::bordered()
            .title("Command")
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(search, header_area);
    frame.render_widget(block, area);
    frame.render_stateful_widget(table, table_area, &mut app.table_state);
}
