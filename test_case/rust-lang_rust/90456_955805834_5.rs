rust
enum Single {
    A(())
}

fn test() -> Single {
    match true {
        _ => Single::A
    }(())
}
