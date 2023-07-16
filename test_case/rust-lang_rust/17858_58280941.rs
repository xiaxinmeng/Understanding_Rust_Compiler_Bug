 rust
#![feature(unsafe_destructor)]

struct S<'a>(&'a T);

struct T;

impl<'a> S<'a> {
    fn foo(&self) {}
}

#[unsafe_destructor]
impl<'a> Drop for S<'a> {
    fn drop(&mut self) {
        println!("dropping S");
    }
}

impl Drop for T {
    fn drop(&mut self) {
        println!("dropping T");
    }
}

fn main() {
    {
        println!("before!");
        let t = T;
        S(&t).foo()
    };
    println!("after!");

}
