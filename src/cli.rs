use clap::Parser;

#[derive(Parser)]
#[command(version, about, author)]
pub struct Args {
    #[arg(short, long)]
    pub address: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>,
}
