rust
fn type_of<T:'static>(t: T) -> std::any::TypeId {
    std::any::TypeId::of::<T>()
}

fn main() {
    println!("{:?}", type_of(main));
}
