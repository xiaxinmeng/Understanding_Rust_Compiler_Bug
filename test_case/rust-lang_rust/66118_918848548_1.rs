rust
struct Yyyyy {
	// This moves the debuginfo for `Xxxxx` into this unit,
	// including its methods.
	_xxxx: lib::Xxxxx,
}

impl Yyyyy {
	fn rrrr() {
		lib::Xxxxx::pppp();
	}
}

fn main() {
	// Use `Yyyyy` and `Xxxxx::pppp`.
	Yyyyy::rrrr();
}
