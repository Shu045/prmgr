use ratatui::{
    Frame,
    layout::Rect,
    style::{
        Color::{self, Yellow},
        Style,
    },
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!("1)Up 2)Down 3) Reset");

    let footer = Paragraph::new(text).block(
        Block::bordered()
            .title("Footer")
            .title_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Yellow)),
    );

    frame.render_widget(footer, area);
}
