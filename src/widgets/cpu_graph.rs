use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let history = &app.cpu_history;

    let graph = if history.is_empty() {
        String::new()
    } else {
        let min = history.iter().copied().fold(f64::INFINITY, f64::min);

        let max = history.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        history
            .iter()
            .map(|&v| cpu_to_char(v, min, max))
            .collect::<String>()
    };

    frame.render_widget(
        Paragraph::new(graph).block(
            Block::bordered()
                .title(" CPU ")
                .border_style(Style::default().fg(Color::Rgb(255, 182, 193))),
        ),
        area,
    );
}

fn cpu_to_char(value: f64, min: f64, max: f64) -> char {
    if (max - min).abs() < f64::EPSILON {
        return '▄';
    }

    let normalized = (value - min) / (max - min);

    match normalized {
        x if x < 1.0 / 8.0 => '▁',
        x if x < 2.0 / 8.0 => '▂',
        x if x < 3.0 / 8.0 => '▃',
        x if x < 4.0 / 8.0 => '▄',
        x if x < 5.0 / 8.0 => '▅',
        x if x < 6.0 / 8.0 => '▆',
        x if x < 7.0 / 8.0 => '▇',
        _ => '█',
    }
}
