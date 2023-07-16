rust
#[derive(Clone(not(T)))]
struct Foo<T> { 
    data: Rc<T>
}
