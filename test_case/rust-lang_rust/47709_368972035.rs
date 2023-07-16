rust
#[derive(PartialEq)]
struct Foo {
    bar: u64
}

fn test() -> Option<Foo> {
    vec![Foo{bar:0}].into_iter()
        .fold(None, |acc: Option<Foo>, o| {
            if let Some(acc) = acc {
                if o.bar == acc.bar {
                    return Some(acc)
                }
            }
            Some(o)
        }).map(|o| {
            o
        })
}
