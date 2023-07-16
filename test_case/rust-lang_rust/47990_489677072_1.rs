rust
trait Trait {}

struct Receiver;
impl Receiver {
    fn f<T>(&self) {}
}

fn main() {
    let receiver = Receiver;
    type Param = dyn Trait;
    receiver.f::<Param>();
}
