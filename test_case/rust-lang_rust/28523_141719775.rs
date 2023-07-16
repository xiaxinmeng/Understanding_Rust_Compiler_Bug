 rust
fn foo<T>() {
    println!("The size of T is {}", std::mem::size_of::<T>());
}
fn main() { foo() }
