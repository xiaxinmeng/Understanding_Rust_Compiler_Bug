rust
if output.status.success() {
    Ok(output)
} else {
    eprintln!("rustc error: {}", std::str::from_utf8(&output.stderr).unwrap());
    Err(())
}
