
#![feature(asm)]

fn main() {
    unsafe {
        asm! {"mov $0,$1"::"0"("bx"),"1"(0x00)}
    }
}
