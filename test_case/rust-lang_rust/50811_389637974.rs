rust
extern { fn abort() -> !; }

fn main() {
    let f = &(std::f64::NAN > std::f64::NAN);
    if *f {
        unsafe { abort(); }
    }
}
