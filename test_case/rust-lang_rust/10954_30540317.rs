
#[allow(dead_code)];

extern "C" fn a<T>(_: *A<T>) { }

struct A<'a, T> {
    a: 'a |&T|,
}

fn main() { }
