 rust
s.chars(|c| if c == ' ' { break }).remaining().split(',', |tok| {println!("{}", tok)});
