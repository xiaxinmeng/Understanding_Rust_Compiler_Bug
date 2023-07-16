rust
struct N<K: Hash>(PhantomData<K>);
type ActuallyNever = N<f32>;

fn main() {
    let foo: Option<!> = None; // Ok
    let foo: Option<ActuallyNever> = None; // ERROR!
}
