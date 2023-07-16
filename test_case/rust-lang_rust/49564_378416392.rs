rust
lines
.iter()
.map(|line| parse_line(line))
.inspect(|line| {
    if let Err(ref e) = *line {
        error!("Parsing error: {}", e);
    }
})
.filter_map(Result::ok)
