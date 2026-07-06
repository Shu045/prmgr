mod app;
mod command;
mod handlers;
mod ui;
mod widgets;

use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}
