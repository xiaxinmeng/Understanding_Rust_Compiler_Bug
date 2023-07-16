rust
fn main() {
    let x = &mut 2;
    let _val = {({x}) as *mut i32};
}
