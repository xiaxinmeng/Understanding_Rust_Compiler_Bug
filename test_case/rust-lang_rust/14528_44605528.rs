 rust
let mut char_iter = data.chars().filter_scan(None, |st, x| st.take().map(|a| (a,x)).or_else(|| { *st = Some(x); None }));
