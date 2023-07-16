 rust
fn foo(mut vec: Vec<int>) {
    let mut iter = vec.iter_mut();
    let mut cur = iter.next().unwrap();
    let mut next = iter.next().unwrap();
    loop {
        *next = 22; // ERROR cannot assign to `next` because it is borrowed
        cur = next; // NOTE borrowed here
        next = iter.next().unwrap();
    }
}
fn main() {}
