rust
fn pass<'a: 'a>(x: Foo<'a>) -> Foo<'a> {
    x
}

fn duplicate<'a>(x: Foo<'a>) -> [Foo<'a>; 100] {
    [pass::<'a>(x); 100]
}
