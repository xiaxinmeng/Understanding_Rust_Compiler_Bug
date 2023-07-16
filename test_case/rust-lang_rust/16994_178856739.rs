 rust
fn cb<'a,T>(x: Box<Fn((&'a i32, &'a (Vec<&'static i32>, bool))) -> T>) -> T {panic!()}

fn main() {
    cb(|(k, &(ref v, b))| (*k, v.clone(), b));
}
