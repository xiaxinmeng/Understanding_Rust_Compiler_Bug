rust
match (3,42) {
    (a, _) | (a, _) if a > 10 => {println!("{}", a)}
    _ => ()
}
