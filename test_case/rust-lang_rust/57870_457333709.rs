rust
trait Swap: Sized {
    fn swap(self, other: Self);
}

impl<T> Swap for &mut T {
    fn swap(self, other: Self) {
        std::mem::swap(self, other);
    }
}

fn hide<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
    x
}

fn dangle() -> &'static [i32; 3] {
    let mut res = &[4, 5, 6];
    let x = [1, 2, 3];
    hide(&mut res).swap(hide(&mut &x));
    res
}

fn main() {
    println!("{:?}", dangle()); // prints nonsense values
}
