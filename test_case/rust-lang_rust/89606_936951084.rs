rust
pub struct S<'a> {
    o: Option<&'a i32>,
}

fn foo(s: &mut S) {
    || {
        let S { o } = s;
        s.o = None;
    };
}
