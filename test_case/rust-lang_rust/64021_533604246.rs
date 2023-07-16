rust
extern {
    type Foo {
        fn size_of_val(&self) -> usize {
            unsafe {
				let this = self as *const Foo as *const CStr;
				this.len()
			};
        }
        fn align_of_val(&self) -> usize {
            1
        }
    }
}
