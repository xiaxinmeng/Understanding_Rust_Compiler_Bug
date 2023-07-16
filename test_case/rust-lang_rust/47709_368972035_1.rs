rust
fn test() -> Option<Foo> {
    let r = vec![Foo{bar:0}].into_iter()
        .fold(None, |acc, o| {
            if let Some(acc) = acc {
                if o.bar == acc.bar {
                    return Some(acc)
                }
            }
            Some(o)
        });
    r
}
