rust
use std::cell::RefCell;
use std::rc::Rc;

trait Swap: Sized {
    fn swap(self, other: Self);
}

impl<T> Swap for Rc<RefCell<T>> {
    fn swap(self, other: Self) {
        <RefCell<T>>::swap(&self, &other);
    }
}

fn hide<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
    x
}

fn dangle() -> &'static [i32; 3] {
    let long = Rc::new(RefCell::new(&[4, 5, 6]));
    let x = [1, 2, 3];
    let short = Rc::new(RefCell::new(&x));
    hide(long.clone()).swap(hide(short));
    let res: &'static [i32; 3] = *long.borrow();
    res
}

fn main() {
    println!("{:?}", dangle()); // prints nonsense values
}
