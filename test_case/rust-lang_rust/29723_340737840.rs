Rust
use std::cell::RefCell;

fn assign<'a, 'b>(x: &RefCell<Option<&'a &'b mut u32>>, y: &'a &'b mut u32) {
    *x.borrow_mut() = Some(y);
}

fn main() {
    let (mut t, mut ten);
    ten = 10;
    t = Some(&mut ten);
    unsound(&mut t);
}

fn unsound<'a>(opt: &'a mut Option<&'a mut u32>) -> Option<&'a mut u32> {
    let store: RefCell<Option<&&mut u32>> = RefCell::new(None);
    match *opt {
        #[cfg(segfault)]
        Some(ref mut x) if {
            // this (making `x` escape from the arm) should be disallowed
            // - `x` shouldn't be `&'a mut &'a mut u32` here
            assign(&store, x);
            false
        } => {
           None
        }
        #[cfg(not(segfault))]
        Some(ref mut x) if { false } => {
            // but just using `x` should be fine: `x` has the type `&'a mut &'a mut u32` here
            Some(x)
        }
        ref mut x => {
            *x = None;
            println!("{:?}", store.borrow());
            None
        }
    }
}
