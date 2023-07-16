rust
enum Never {}

fn test() -> Never {
    match true {
        _ => Never
    }()
}
