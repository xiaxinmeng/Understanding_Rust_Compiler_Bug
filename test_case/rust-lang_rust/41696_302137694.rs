Rust
fn f1(x: &mut u32) {
    let odd = (-((*x & 1) as i32)) as u32;
    *x = ((*x * 3 + 1) & odd)
       + ((*x / 2) & !odd);
}

fn f2(x: &mut u32) {
    f1(x);
    f1(x);
    f1(x);
    f1(x);
}

fn f3(x: &mut u32) {
    f2(x);
    f2(x);
    f2(x);
    f2(x);
}

fn f4(x: &mut u32) {
    f3(x);
    f3(x);
    f3(x);
    f3(x);
}

fn f5(x: &mut u32) {
    f4(x);
    f4(x);
    f4(x);
    f4(x);
}

fn f6(x: &mut u32) {
    f5(x);
    f5(x);
    f5(x);
    f5(x);
}

fn f7(x: &mut u32) {
    f6(x);
    f6(x);
    f6(x);
    f6(x);
}

fn f8(x: &mut u32) {
    f7(x);
    f7(x);
    f7(x);
    f7(x);
}

fn f9(x: &mut u32) {
    f8(x);
    f8(x);
    f8(x);
    f8(x);
}

fn fA(x: &mut u32) {
    f9(x);
    f9(x);
    f9(x);
    f9(x);
}

fn main() {
    let mut x = 44;
    fA(&mut x);
    println!("{}", x);
}
