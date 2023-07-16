rust
fn main() -> std::io::Result<()> {
    std::process::Command::new(r"hello").output()?;
    println!("Success!");
    Ok(())
}
