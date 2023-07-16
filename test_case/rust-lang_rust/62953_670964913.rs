rust
fn main() {
    let mut stash: Option<&i32> = None;
    let mut thief = |r: &i32| { stash = Some(r) };
    thief(&42);
}
