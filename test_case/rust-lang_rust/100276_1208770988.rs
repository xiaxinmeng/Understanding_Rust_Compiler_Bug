rust
fn let_chains(entry: std::io::Result<std::fs::DirEntry>) {
    if let Ok(entry) = entry
        && let file_name = entry.file_name()
        && let Some(s) = file_name.to_str()
        && s.contains("")
    {
        println!("Ok");
    }
}
