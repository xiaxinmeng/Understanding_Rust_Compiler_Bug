rust
fn mutated<T, F, S: ?Sized = T>(mut t: T, mut f: F) -> T where F: FnMut(&mut S), T: AsMut<S> {
    f(t.as_mut());
    t
}

fn main() {
    let my_vec1 = mutated(vec![0, 3, 2, 1], |v| { v.sort() });
    let my_vec2 = mutated(vec![0, 3, 2, 1], <[i32]>::sort);
    println!("{:?}", my_vec1); // [0, 1, 2, 3]
}
