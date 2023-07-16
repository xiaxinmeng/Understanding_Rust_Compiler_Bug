rust
let blerg = if template.target_path.exists() {
    std::fs::read_to_string(&template.target_path).unwrap().trim() == contents.trim()
} else {
    false
};
