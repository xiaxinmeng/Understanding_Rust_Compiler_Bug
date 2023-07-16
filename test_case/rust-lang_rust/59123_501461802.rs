rust
let mut x = get_future();
dbg!(std::mem::size_of_val(&x));
x.await
