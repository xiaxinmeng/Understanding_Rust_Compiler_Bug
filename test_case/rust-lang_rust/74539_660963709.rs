rust
enum E {
	A(u8),
}

fn fun(arg: E) {
	match arg {
		A(x @ ..) => {x}
	};
}
