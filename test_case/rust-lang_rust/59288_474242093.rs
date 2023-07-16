rust
match thing {
    PAT(y) if let Some(x) = y {
        print(x); // Nope
    }
}
