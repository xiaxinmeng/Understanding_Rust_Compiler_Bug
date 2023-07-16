rust
macro_rules! foo {
    () => {
        (0 | 1)
    };
}

fn main() {
	match 0 {
		foo!() -> {}
	}
}
