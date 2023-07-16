
fn map_in_place<T,F>(dest: &mut T, f: F)
    where F: FnOnce(T) -> T
{
    use std::mem::{forget, replace, uninitialized};
    unsafe {
        let old = replace(dest, uninitialized());
        forget(replace(dest, f(old)));
    }
}
