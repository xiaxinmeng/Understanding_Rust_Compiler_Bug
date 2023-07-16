rust
extern { fn abort() -> !; }

static B: bool = std::f64::NAN > std::f64::NAN;

fn main() {
    if B {
        unsafe { abort(); }
    }
}
