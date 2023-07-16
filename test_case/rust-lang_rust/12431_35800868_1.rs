 rust
s.chars().each(|c| { c != ' ' }).remaining().split(',', |tok| {println!("{}", tok)});
