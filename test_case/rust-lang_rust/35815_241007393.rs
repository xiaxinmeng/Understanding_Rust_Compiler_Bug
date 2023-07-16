 rust
struct Foo<T: ?Sized> {
    a: i64,
    b: bool,
    c: T,
}

fn main() {
    let a: &Foo<i32> = &Foo { a: 1, b: false, c: 2i32 };
    println!("{}", std::mem::size_of_val(a));

    let a: &Foo<Send> = a;
    println!("{}", std::mem::size_of_val(a));
}
