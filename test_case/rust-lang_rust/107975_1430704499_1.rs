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
    println!("{}", a==b); // false
    let c = a;
    println!("{} {} {}", a==b, a==c, b==c); // false true false
    println!("{a} {b}");
    println!("{} {} {}", a==b, a==c, b==c); // true true true
}
