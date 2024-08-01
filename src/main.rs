mod app;
mod cli;
mod listener;
mod log;
mod tui;

use std::io;

use app::App;
use clap::Parser;
use cli::Args;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut terminal = tui::init()?;
    let app_result = App::new(args).run(&mut terminal);
    tui::restore()?;
    app_result
}
