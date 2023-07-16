rust
fn main() {
    let a = {
        let v = 0;
        &v as *const _ as usize
    };
    let b = {
        let v = 0;
        &v as *const _ as usize
    };
    println!("{}", a==b); // false
    println!("{a}") // or {b}
    println!("{}", a==b); // true
}
