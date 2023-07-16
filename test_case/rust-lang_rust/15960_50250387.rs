
// Rust will fail to infer from the type of `f` that `random()` should be `random<int>();`
let f: int = std::rand::random() % 100i;
