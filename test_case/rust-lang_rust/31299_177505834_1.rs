 Rust
trait Front { type Back; }
impl<T> Front for T { type Back = *mut T; /* note *mut is here */ }
struct PtrBack<T: Front>(T::Back);
struct M(PtrBack<M>);

fn main() {
    println!("{}", std::mem::size_of::<M>());
}
