rust
struct A<'a> {
    backref: Option<&'a mut A<'a>>,
}

fn child_with_value<'a>(this: &'a mut A<'a>) -> A<'a> {
    A {
        backref: Some(this),
    }
}

fn errors(a: &mut A) {
    let mut b = child_with_value(a);
}

