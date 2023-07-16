 rust
#![feature(box_syntax)]
struct F(Box<()>);

impl Drop for F {
    fn drop(&mut self) {
        println!("drop");
        panic!()
    }
}

fn main() {
    let _v = vec!(F(box ()), F(box ()), F(box ()), F(box ()));
}
