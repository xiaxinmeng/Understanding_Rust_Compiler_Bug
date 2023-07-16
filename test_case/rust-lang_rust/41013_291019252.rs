rust
fn test5<'b, 'a: 'b, 'c>(f: &'c Fn() -> &'a ()) -> &'c Fn() -> &'b () {
    f  // ok
}

fn test6<'b, 'a: 'b>(f: Box<Fn() -> &'a ()>) -> Box<Fn() -> &'b ()> {
    f  // ok
}
