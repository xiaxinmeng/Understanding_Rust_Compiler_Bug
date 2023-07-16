rust
fn main() {
    // T = *mut _
    let mut constructor = RootedGuard::new(std::ptr::null_mut());

    // "warning: the type of this value must be known in this context"
    // But is this really ambiguous? Or does the warning have a false positive?
    let handle = constructor.handle_mut();

    // Type inference establishes `T = *mut JSObject` here.
    get_constructor_object_from_local_name(handle);
}

pub struct RootedGuard<T>(T);
pub struct MutableHandle<T>(T);
pub struct JSObject;

impl<T: Clone> RootedGuard<T> {
    pub fn new(x: T) -> Self { RootedGuard(x) }
    pub fn handle_mut(&mut self) -> MutableHandle<T> { unimplemented!() }
}

impl<T> std::ops::Deref for RootedGuard<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn get_constructor_object_from_local_name(_: MutableHandle<*mut JSObject>) {}
