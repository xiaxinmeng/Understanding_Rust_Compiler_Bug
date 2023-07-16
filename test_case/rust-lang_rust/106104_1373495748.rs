
use toml::Value;

fn main() {
    let input = std::fs::read_to_string("triagebot.toml").unwrap();
    let value = input.parse::<Value>().unwrap();
    println!("{value}");
}
