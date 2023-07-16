 rust
let mut n: $T = 0;
let mut x: $T = *self;
loop {
  if x < base { break }
  x /= base;
  n += 1;
}
Some(n)
