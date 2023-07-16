rust
struct Foo;

fn print_type<T>(val: T) { println!("{}", std::any::type_name::<T>()) }

fn main() {
    let a = &Foo;
    let b = a.to_owned();
    print_type(b);
}
