rust
println!("{:?}", "  a  b  c  ".split_whitespace().collect::<Vec<&str>>());
println!("{:?}", "  a  b  c  ".split_terminator(char::is_whitespace).collect::<Vec<&str>>());
