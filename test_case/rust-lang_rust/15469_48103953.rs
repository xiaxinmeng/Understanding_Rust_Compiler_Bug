 rust
#![crate_type = "dylib"]

struct Boxee<'a> {
    cb: ||: 'a,
}

pub struct Boxer<'a> {
    data: Box<Boxee<'a>>,
}

impl<'a> Drop for Boxer<'a> {
    fn drop(&mut self) {
        println!("dropping boxer");
    }
}

pub fn build_it<'a>(cb: ||: 'a) -> Boxer<'a> {
    let data = box Boxee{cb: cb};
    Boxer{data: data}
}
