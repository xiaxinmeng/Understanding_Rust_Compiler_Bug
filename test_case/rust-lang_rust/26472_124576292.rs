 rust
#![feature(core, unboxed_closures)]
mod inner {
    pub struct Dog {
        pub x: usize,
        pub c: Barking,
        d: Barking,
    }

    impl Dog {
        pub fn new() -> Dog {
            Dog {
                x: 5,
                c: Barking,
                d: Barking,
            }
        }

        pub fn d() -> usize {
            3
        }
    }

    pub struct Barking;
    impl Barking {
        pub fn woof(&self) -> usize {
            3
        }
    }

    impl Fn<()> for Barking {
        extern "rust-call" fn call(&self, _: ()) -> usize {
            self.woof()
        }
    }

    impl FnMut<()> for Barking {   
        extern "rust-call" fn call_mut(&mut self, _: ()) -> usize {
            self.woof()
        }
    }

    impl FnOnce<()> for Barking {
        type Output = usize;
        extern "rust-call" fn call_once(self, _: ()) -> usize {
            self.woof()
        }
    }
}

fn main() {
    let dog = inner::Dog::new();
    let _ = dog.x();
    let _ = dog.c();
    let _ = (dog.c)();
    let _ = dog.d();
    let _ = (dog.d)();
}
