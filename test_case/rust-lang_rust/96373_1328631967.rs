rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    return Err("string message"); // compile error, wrong type
    return Err("string message".into()); // works
    Err("string message")?; // works
    do yeet "string message"; // works
}
