 rust
let mut count = 0;
some_iter.filter_map(|x| { if count == 0 { count = n - 1; Some(x) } else { count -= 1; None })
