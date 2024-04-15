use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    pub address: String,

    #[clap(short, long, default_value = "info")]
    pub log_level: String,
}
