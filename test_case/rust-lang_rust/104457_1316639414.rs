rust
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_enum, default_value_t=Level::Debug)]
    level: Level,
}

#[derive(ValueEnum, Clone, Debug)]
enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

fn main() {
    println!("{:?}", Args::parse());
}
