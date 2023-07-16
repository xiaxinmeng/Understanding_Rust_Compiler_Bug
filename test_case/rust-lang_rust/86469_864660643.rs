rust
use std::ptr;

trait Context {
    type BindGroupLayoutId;
}
pub struct C;
pub struct BindGroupLayout {
    id: <C as Context>::BindGroupLayoutId,
}
impl Context for C {
    type BindGroupLayoutId = ();
}

pub fn main() {
    let y: *const BindGroupLayout = ptr::null();

    let mut hasher = std::collections::hash_map::DefaultHasher::default();
    std::ptr::hash(y, &mut hasher);
}
