rust
#[derive(MyDerive)]
struct Bar {
    val: [u8; {
		struct Inner {
			#[cfg(FALSE)] field: u8
		}
		0
	}]
}
