
fn main() {
    println!("Hello, world!");
    let regex = regex::Regex::new("^Hello").unwrap();
    println!("{}", regex);
}
