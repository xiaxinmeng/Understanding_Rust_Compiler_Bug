 rust
#![feature(macro_rules)]

struct Element;

macro_rules! foo {
    ($tag: expr, $string: expr) => {
        if $tag == $string {
            let element = box Element;
            unsafe {
                return std::mem::transmute::<_, uint>(element);
            }
        }
    }
}

fn bar() -> uint {
    foo!("a", "b");
    0
}

fn main() {
    bar();
}
