use clap::Parser;

#[derive(Parser)]
#[command(version, about, author)]
pub struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub address: String,

    #[arg(short, long, default_value_t = 9000)]
    pub port: u16,
}
