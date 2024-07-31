use crate::cli::Args;

pub struct App {
    args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self { args }
    }
}
