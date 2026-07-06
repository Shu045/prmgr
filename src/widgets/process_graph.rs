use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let graph = build_graph(&app.process_history);

    frame.render_widget(
        Paragraph::new(graph).block(
            Block::bordered()
                .title(" Process Count ")
                .border_style(Style::default().fg(Color::Rgb(255, 182, 193))),
        ),
        area,
    );
}

fn build_graph(data: &Vec<usize>) -> String {
    if data.is_empty() {
        return String::new();
    }

    let min = *data.iter().min().unwrap() as f64;
    let max = *data.iter().max().unwrap() as f64;

    let range = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        max - min
    };

    data.iter()
        .map(|v| {
            let n = (*v as f64 - min) / range;

            match n {
                x if x < 0.125 => '▁',
                x if x < 0.25 => '▂',
                x if x < 0.375 => '▃',
                x if x < 0.5 => '▄',
                x if x < 0.625 => '▅',
                x if x < 0.75 => '▆',
                x if x < 0.875 => '▇',
                _ => '█',
            }
        })
        .collect()
}
