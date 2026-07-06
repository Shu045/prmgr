use crate::app::{self};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::widgets::{footer, header, process_table, sidebar};

pub fn draw(frame: &mut Frame, app: &mut app::App) {
    let [header_area, body_area, footer_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    let [sidebar_area, table_area] =
        Layout::horizontal([Constraint::Length(30), Constraint::Min(0)]).areas(body_area);

    header::draw(frame, header_area, app);
    sidebar::draw(frame, sidebar_area, app);
    process_table::draw(frame, table_area, app);
    footer::draw(frame, footer_area, app);
}
