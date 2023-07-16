 rust
    let s: String = format!("{:.10}", v).trim_right_matches('0').into();
    if s.ends_with('.') { s + "0" } else { s }
