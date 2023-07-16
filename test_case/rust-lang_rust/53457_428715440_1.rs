rust
existential type X: Iterator<Item = i32>;

fn temp() -> X {
    vec![1].into_iter().filter(|x| *x > 1)
}
