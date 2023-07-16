rust
fn test() -> impl Iterator<Item = i32> {
    foo(
        panic!()
    )
}

fn foo(_: !) -> impl Iterator<Item = i32> { std::iter::empty() }
