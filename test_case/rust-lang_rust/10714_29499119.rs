
enum v {}
fn main() {
    let y: v = unsafe { std::unstable::intrinsics::uninit() };
}
