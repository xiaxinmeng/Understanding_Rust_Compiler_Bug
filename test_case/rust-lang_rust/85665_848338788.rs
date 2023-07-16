rust
fn item_constructor(data: &[u8; 8]) -> Vec<u8> {
	data.to_vec()
}

let foo = Foo::new(TEMPLATE, item_constructor);
