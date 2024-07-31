mod app;
mod cli;

use app::App;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    let app = App::new(args);
}
