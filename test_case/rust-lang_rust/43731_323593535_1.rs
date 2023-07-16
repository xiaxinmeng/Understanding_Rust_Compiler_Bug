rust
fn main() {
    //let innum: i8 = 122;
    let in_num:i8=122;

    let a = 2_i8.checked_mul(in_num);
    let b= 2_i8.overflowing_mul(in_num);
    println!("{:?} / {:?}", a, b);
    println!("{:?}", 2*in_num);
    let c:i8=2*in_num;
}
