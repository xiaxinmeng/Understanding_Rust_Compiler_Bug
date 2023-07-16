rust
// https://doc.rust-lang.org/std/panic/fn.resume_unwind.html
pub fn resume_unwind(payload: Box<dyn Any + Send>) -> ! {
    payload.vtable = payload.vtable.nonpanicking_drop_vtable;
    /* now resume using `payload` as the Box<dyn Any>... */
}
