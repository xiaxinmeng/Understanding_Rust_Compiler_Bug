rust
let mut x = get_future();
dbg!(std::mem::size_of_val(&x));
{
  let mut pinned = x;  // <--- This move
  loop {
    // use `&mut pinned`
    yield;
  }
}
