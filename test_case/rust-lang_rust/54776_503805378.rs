rust
use cell::*;
use std::slice::Iter;

struct MyRef(RefCell<Vec<usize>>);

impl MyRef {
    fn iter<'t, 's: 't>(&'s self) -> RefVal<'t, Iter<usize>> {
        Ref::map_val(self.0.borrow(), |x| x.iter())
    }
}

fn main() {
    let v = MyRef(RefCell::new(vec![1,2,3,4,5]));
    println!("capacity before: {:?}", v.0.borrow().capacity());
    
    let it = v.iter().clone();

    v.0.borrow_mut().push(0);

    println!("capacity after: {:?}", v.0.borrow().capacity());

    for &a in it {
        print!("{} ", a);
    }
}
