rust
// No error if a coercion would otherwise occur.
mem::transmute::<fn(), usize>(main);
