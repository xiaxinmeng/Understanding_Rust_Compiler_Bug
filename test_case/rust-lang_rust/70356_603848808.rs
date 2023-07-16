rust
static A: UnsafeCell<u32> = UnsafeCell::new(42);
static B: &'static UnsafeCell<u32> = &A;
unsafe fn foo() -> u32 {
	*B.get()
}
