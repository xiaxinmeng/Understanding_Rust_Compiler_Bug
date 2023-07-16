rust
#![feature(nll)]

fn foo<T: AsRef<[u8]> + ?Sized>(slice: &T) -> &[u8] {
    slice.as_ref()
}

fn main() {
    let v = [1, 2, 3, 4];
    let x = foo(&foo(&v));
    println!("{:?}", x);
}
