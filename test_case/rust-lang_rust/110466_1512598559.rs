
// I "inlined" `Foo` and replaced `Vec` with `Box`. 
struct Bar<T> {
    f: Box<Bar<T>>,
}
