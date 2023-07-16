rust
impl Foo<u32> {
    #[no_mangle]
    pub fn bar() {}
}

impl Trait for Foo<u32> {
	#[no_mangle]
	fn foo() {}
}

impl Trait2<u32> for Foo<u32> {
	#[no_mangle]
	fn foo2() {}
}
