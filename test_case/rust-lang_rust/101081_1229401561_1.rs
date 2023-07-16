rs
use std::sync::RwLockReadGuard;
fn test1<'a, T: ?Sized>(guard: RwLockReadGuard<'a, T>) -> RwLockReadGuard<'a, T> {
    // unsafe { std::mem::transmute(guard) } // this has the same problem as in my original comment
    // But. would transmute_copy (and forget) be unsound here?
    unsafe {
      let guard = std::mem::ManuallyDrop(guard);
      std::mem::transmute_copy(&from)
    }
}
