rust
#![feature(asm)]

fn main() {
    let mut x: isize = 0;
    let y: isize = 1;
    let mut z: isize;

    unsafe {
        asm!("mov ($1), $0"
             : "=r"(*{z=2; &mut x})
             : "r"(&{z+=3; y}));
             // ^ value assigned is never read -- wrong
    }
    assert_eq!(z, 5); // Passes
}
