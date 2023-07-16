rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    str1: String,
    str2: String,
}

fn main() {
    let c = Cli::parse();
    println!("{}", [&c.str1, &c.str2].join(","));
}
