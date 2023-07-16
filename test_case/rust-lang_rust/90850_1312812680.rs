rust
pub fn cast(a: &dyn std::any::Any) -> &i32 {
    unsafe { a.downcast_ref_unchecked() }
}
