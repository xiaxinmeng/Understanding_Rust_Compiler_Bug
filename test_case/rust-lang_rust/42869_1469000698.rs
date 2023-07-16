rust
println!("{}", path.starts_with(r"\\?\"));
println!("{}", path.to_string_lossy().starts_with(r"\\?\"));
print!("{}", path.display());
std::io::stdout().flush().unwrap();
