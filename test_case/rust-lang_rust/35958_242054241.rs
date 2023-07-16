 rust
fn main() {
    let mut to = String::with_capacity(20);
    let from = "hello"; 
    to = from.chars().fold("x".to_string(), |s, item| s+item.to_string().as_str() );
    println!("{:?}",to);
}
