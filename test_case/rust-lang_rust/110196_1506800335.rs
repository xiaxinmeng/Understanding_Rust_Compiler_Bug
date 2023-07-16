rust
let mut iter = 0u16..;
(&mut iter).take(2usize.pow(16)).count();
dbg!(iter);
