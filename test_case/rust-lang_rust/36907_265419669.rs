
#![feature(asm)]
fn abcd() {
    println!("lo");
}
fn main() {
    unsafe{asm!(".global abc;.set abc, ${0:P}" : : "i"(abcd as fn()))};
    println!("hi");
    unsafe { abc() };
}
extern {
    fn abc();
}
