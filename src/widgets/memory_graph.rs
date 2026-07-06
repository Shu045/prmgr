use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let graph: String = app
        .memory_history
        .iter()
        .map(|v| memory_to_char(*v))
        .collect();

    frame.render_widget(Paragraph::new(graph), area);
}

fn memory_to_char(mem: f64) -> char {
    match mem {
        x if x < 12.5 => '▁',
        x if x < 25.0 => '▂',
        x if x < 37.5 => '▃',
        x if x < 50.0 => '▄',
        x if x < 62.5 => '▅',
        x if x < 75.0 => '▆',
        x if x < 87.5 => '▇',
        _ => '█',
    }
}
