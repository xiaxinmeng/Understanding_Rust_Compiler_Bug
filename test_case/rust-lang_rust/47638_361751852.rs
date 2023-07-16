rust
struct Newtype<T>(T);

fn id<'c, 'b>(f: &'c Newtype<&'b Fn(&i32)>) -> &'c Newtype<&'b Fn(&'static i32)> {
    f
}

fn main() {
    let f: &Fn(&i32) = &|x| {};
    id(&Newtype(f));
}
