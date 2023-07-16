rust
pub fn a() -> i32 {
    1
}
pub fn b(_: i32) {}

pub fn ice(arg: i32, discr: bool) -> i32 {
    loop {
        let mut c = 1;
        let mut ret = a();

        c = !arg;
        ret = c;
        if discr {
            b(c);
            return ret;
        }
    }
}

pub fn main() {
    ice(1, false);
}

