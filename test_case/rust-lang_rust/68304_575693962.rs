rust
#[repr(align(1048576))] // 1MiB
struct Overaligned(u8);

#[repr(align(16))]
struct Normal(u8);

trait TakeSelfByValue {
    fn f(self) {
        todo!()
    }
}

impl TakeSelfByValue for Overaligned {}
impl TakeSelfByValue for Normal {}

fn make_overaligned() -> Box<dyn TakeSelfByValue> {
    Box::new(Overaligned(0)) // errors due to too strict alignment
}

fn make_normal() -> Box<dyn TakeSelfByValue> {
    Box::new(Normal(0)) // doesn't error -- alignment is under limit
}
