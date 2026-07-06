use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use sysinfo::System;

use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, _app: &App) {
    // ---------- System Info ----------
    let uptime = System::uptime();
    let days = uptime / 86_400;
    let hours = (uptime % 86_400) / 3_600;
    let minutes = (uptime % 3_600) / 60;

    let formatted_uptime = format!("{days}d {hours}h {minutes}m");

    let os = System::name().unwrap_or_else(|| "Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    let formatted_stats = format!(
        "Load: {:.2}\nUptime: {}",
        System::load_average().one,
        formatted_uptime
    );

    let formatted_os_info = format!("OS: {}\nKernel: {}\nHost: {}", os, kernel, hostname);

    // ---------- Layout ----------
    let [title_area, stats_area, os_area] = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Length(30),
        Constraint::Min(0),
    ])
    .areas(area);

    let border_style = Style::default().fg(Color::Rgb(255, 182, 193));

    // ---------- App Title ----------
    frame.render_widget(
        Paragraph::new("PRMGR")
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(border_style),
            ),
        title_area,
    );

    // ---------- Load + Uptime ----------
    frame.render_widget(
        Paragraph::new(formatted_stats)
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("System")
                    .border_type(BorderType::Rounded)
                    .border_style(border_style),
            ),
        stats_area,
    );

    // ---------- OS Info ----------
    frame.render_widget(
        Paragraph::new(formatted_os_info).block(
            Block::bordered()
                .title("OS")
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        ),
        os_area,
    );
}
