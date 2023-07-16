rust
fn main() {
    let mut v: Vec<i32> = vec![];
    let f = |_: &i32| true; // now it compiles
    v.retain(f);
}
