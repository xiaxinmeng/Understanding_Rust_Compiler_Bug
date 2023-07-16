rust
async fn foo() {
}

fn size_of<T>(_t: T) -> usize {
    std::mem::size_of::<T>()
}

fn main() {
    println!("{}", size_of(foo()));
}
