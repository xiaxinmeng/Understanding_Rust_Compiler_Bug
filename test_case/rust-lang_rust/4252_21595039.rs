
trait X {
    fn call(&self);
}

struct Y;
struct Z<T> {
    x: T
}

impl X for Y {
    fn call(&self) {
    }
}

#[unsafe_destructor]
impl<T: X> Drop for Z<T> {
    fn drop(&self) {
        self.x.call(); // Adding this statement causes an ICE.
    }
}

fn main() {
    let y = Y;
    let _z = Z{x: y};
}
