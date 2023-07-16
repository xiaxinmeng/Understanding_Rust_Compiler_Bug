rust
struct A(());

fn test() -> A {
    match true {
        _ => A
    }(())
}
