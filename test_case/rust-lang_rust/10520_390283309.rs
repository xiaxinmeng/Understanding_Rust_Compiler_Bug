rust
fn foo(mut vec: Vec<i32>) {
    let mut iter = vec.iter_mut();
    let mut cur = iter.next().unwrap();
    let mut next = iter.next().unwrap();
    loop {
        *next = 22;
        cur = next;
        next = iter.next().unwrap();
    }
}
fn main() {}
