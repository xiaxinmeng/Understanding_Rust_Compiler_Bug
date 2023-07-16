rust
fn snippet() {
    let var = "CI";
    match env::var(var) {
        Ok(val) => assert_eq!(check!(fs::read_link(r"C:\Documents and settings\")), Path::new(r"C:\Users")),
        Err(e) => if Path::new(r"C:\Documents and settings\").exists() {
            assert_eq!(check!(fs::read_link(r"C:\Documents and settings\")), Path::new(r"C:\Users"))
        } else { println!("Path does not exist => Skip the test.") },
    }
}
