rust
let working_dir = match env::current_dir() {
    Ok(dir) => dir.to_string_lossy().into_owned(),
    Err(e) => panic!(p_s.span_diagnostic.fatal(&format!("Current directory is invalid: {}", e))),
};
