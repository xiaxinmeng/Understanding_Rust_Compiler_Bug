rust
fn f(vec: &mut Vec<u8>) -> &u8 {
    if let Some(n) = vec.iter_mut().find(|n| **n == 1) {
        *n = 10;
        n
    } else {
        vec.push(10);
        vec.last().unwrap()
    }
}

fn main() {
    let mut vec = vec![1, 2, 3];
    f(&mut vec);
}
