 rust
struct Dummy<T>(T);

fn foo(mut vec: Vec<isize>) {
    let mut iter = vec.iter_mut();
    let mut cur = Dummy(iter.next().unwrap());
    let mut next = iter.next().unwrap();
    loop {
        *next = 22;
        cur = Dummy(next);
        next = iter.next().unwrap();
    }
}
fn main() {}
