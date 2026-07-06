use crate::{
    app::App,
    widgets::{cpu_graph, memory_graph, process_graph},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title("Resources")
        .title_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(255, 182, 193)));

    let inner = block.inner(area);

    let [cpu_graph_area, memory_graph_area, process_graph_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .areas(inner);

    cpu_graph::draw(frame, cpu_graph_area, app);
    memory_graph::draw(frame, memory_graph_area, app);
    process_graph::draw(frame, process_graph_area, app);

    frame.render_widget(block, area);
}
