rust
struct FoobarContainer<T> {
    values: Vec<T>,
}

#[repr(transparent)]
struct Foobar<T> {
    value: T,
}

let input: Vec<Foobar<T>> = unimplemented!();
let output: Vec<T> = mem::transmute(input);
