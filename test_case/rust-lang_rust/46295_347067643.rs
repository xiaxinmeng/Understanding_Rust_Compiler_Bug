rust
fn main() {
    let x: Result<i32, ()> = Ok(1);
    x.map(|_: ()| ()).unwrap();
}
