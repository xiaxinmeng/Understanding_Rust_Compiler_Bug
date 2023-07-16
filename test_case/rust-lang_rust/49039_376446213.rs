rust
#[allow(unreachable_code)]
pub fn demo(x: !) -> Box<std::error::Error> {
    Box::new(x) as _
}
