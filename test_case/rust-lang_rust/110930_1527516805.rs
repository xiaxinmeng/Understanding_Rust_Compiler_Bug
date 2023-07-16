rust
pub enum E<A, B> {
    X(A),
    Y(B),
}
pub unsafe fn f<A, B>(e: *mut E<A, B>) {
    std::ptr::drop_in_place(e);
}
