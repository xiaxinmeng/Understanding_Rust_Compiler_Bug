 rust
#![feature(unboxed_closures)]

struct Dropper {
    msg: &'static str
}

impl Dropper {
    fn new(msg: &'static str) -> Dropper {
        Dropper {
            msg: msg
        }
    }
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("drop: {}", self.msg);
    }
}

pub fn main() {
    let c = {
        let one = Dropper::new("one");
        let two = Dropper::new("two");
        let three = Dropper::new("three");
        |:| { let _ = (&one, &two, &three); }
    };
    println!("Dropping unboxed closure");
    drop(c);
}
