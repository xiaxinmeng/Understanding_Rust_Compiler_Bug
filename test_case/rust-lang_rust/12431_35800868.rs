 rust
let mut iter = s.chars();
for c in iter { if c == ' ' { break } }
iter.remaining().split(',', |tok| {println!("{}", tok)});
