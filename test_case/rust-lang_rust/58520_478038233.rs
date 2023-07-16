rust
let display_chain: String = error
    .iter_sources()
    .fold(format!("Error: {}", error), |mut s, e| {
        s.push_str(&format!("\nCaused by: {}", e));
        s
    });
