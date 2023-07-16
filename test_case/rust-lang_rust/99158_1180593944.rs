rust
fn helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T { v }

pub fn make_static<'a, T>(input: &'a T) -> &'static T {
    let f: fn(_, &'a T) -> &'static T = helper;
    f(&&(), input)
}
