rust
fn write_db(key: String, val: String) -> Result<(), Error> {
    std::fs::write(
        "hello.txt", 
        format!("{} {}", key, val));
}
