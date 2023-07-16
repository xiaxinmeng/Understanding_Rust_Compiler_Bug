 rust
// Rust will fail to infer from the type of `f` that `random()` should be `random<int>();`
let f: int = std::rand::random() % 100i;

// But it will infer it when we do this: (although this is incorrect logically)
let f: int = 100i % std::rand::random();
