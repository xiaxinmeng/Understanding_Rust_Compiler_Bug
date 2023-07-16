
#![feature(unsafe_no_drop_flag)]
#![feature(test)]

extern crate test;

#[unsafe_no_drop_flag]
pub struct BigAndComplicatedDestructor {
    x: [u8; 255],
}

impl Drop for BigAndComplicatedDestructor {
    #[inline(never)]
    fn drop(&mut self) {
        if !self.x.iter().all(|&x| x == 0) {
            println!("DROP");
            test::black_box(self);
        }
    }
}

#[inline(never)]
fn f(x: BigAndComplicatedDestructor) {
    println!("1");
    drop(x)
}

pub fn look_at_me_copy(i: i32, x: BigAndComplicatedDestructor) {
    f(x)
}
