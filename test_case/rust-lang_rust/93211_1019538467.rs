rust
fn snippet() {
    let var = "CI";
    match env::var(var) {
        Ok(val) => if val == "true" {assert_eq!(check!(fs::read_link(r"C:\Documents and settings\")), Path::new(r"C:\Users"));},
        Err(e) => println!("Test case can be skipped"),
    }
}
