use crate::{app::App, command::InputMode};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let help = match app.mode {
        InputMode::Normal => " ↑/↓ Navigate   •   Type Search   •   : Command Mode   •   q Quit ",
        InputMode::Command => {
            " ↑/↓ Navigate   •   Enter Execute   •   Tab Insert PID   •   Esc Cancel   •   ⌫ Delete "
        }
    };

    let footer = Paragraph::new(help)
        .style(
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::bordered()
                .title(" Help ")
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    frame.render_widget(footer, area);
}
