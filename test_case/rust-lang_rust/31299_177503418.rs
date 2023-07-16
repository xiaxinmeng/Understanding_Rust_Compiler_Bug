 Rust
trait Front { type Back; }
impl<T> Front for T { type Back = T; }
struct PtrBack<T: Front>(*mut T::Back);
struct M(PtrBack<M>);

fn main() {
    println!("{}", std::mem::size_of::<M>());
}
