Rust
use std::mem;

struct Foo {
    flag: bool,
    data: str,
}

struct FooConcrete {
    flag: bool,
    data: [u8; 10],
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("splat! {:?} {}", self.flag, &self.data)
    }
}


fn main() {
    let _foo: Box<Foo> = unsafe { mem::transmute(
        (Box::new(FooConcrete {
            flag: false,
            data: [0; 10]
        }), 10)
    )};
}
