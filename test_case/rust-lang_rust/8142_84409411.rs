 rust
#![feature(unsafe_destructor)]

use std::cell::Cell;

struct InvalidateOnDrop {
    orig_value: &'static str,
    value: &'static str
}

// Note: Definition has no constraint relating 'b and 'a ...
struct P<'c, 'b:'c, 'a> {
    x: &'c Cell<&'b InvalidateOnDrop>,
    y: &'a InvalidateOnDrop
}

// ... but the Drop impl says 'a outlives 'b, and thus is
// able to copy `self.y` into `self.x`.  Havoc ensues below.
#[unsafe_destructor]
impl<'c, 'b:'c, 'a:'b> Drop for P<'c, 'b, 'a> {
    fn drop(&mut self) {
        self.x.set(self.y);
    }
}

#[allow(non_snake_case)]
fn InvalidateOnDrop(s: &'static str) -> InvalidateOnDrop {
    InvalidateOnDrop {
        orig_value: s,
        value: s,
    }
}

impl Drop for InvalidateOnDrop {
    fn drop(&mut self) {
        self.value = "invalidated";
    }
}

struct Loud<'l1, 'l2:'l1> {
    name: &'static str,
    value: &'l1 Cell<&'l2 InvalidateOnDrop>
}

#[unsafe_destructor]
impl<'l1, 'l2> Drop for Loud<'l1, 'l2> {
    fn drop(&mut self) {
        let orig = self.value.get().orig_value;
        let val = self.value.get().value;
        if orig == val {
            println!("dropping Loud {} pointing to {}", self.name, val);
        } else {
            println!("dropping Loud {} pointing to {} (orig: {})", self.name, val, orig);

        }
    }
}



fn main() {
    let b = InvalidateOnDrop("b");
    let c = Cell::new(&b);
    let _l1 = Loud { name: "l1", value: &c };
    let a = InvalidateOnDrop("a");
    let _p = P { x: &c, y: &a };

    let _l2 = Loud { name: "l2", value: &c };
    println!("Hello World");
}
