rust
lines
.iter()
.map(|line| parse_line(line))
.filter_map(|line| match line {
    Ok(x) => Some(x),
    Err(e) => { error!("Parsing error: {}", e); None },
})
// etc
