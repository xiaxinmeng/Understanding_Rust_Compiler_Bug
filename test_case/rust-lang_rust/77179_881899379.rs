rust
type Pointer<T> = impl std::ops::Deref<Target=T>;

fn main() {
    let p: Pointer<_> = Box::new(1);
}
