rust
struct Foo;

impl Foo {
    fn bar(&mut self) -> bool { true }
}

fn error(foo: &mut Foo) {
    if let Some(_) = Some(true) {
    } else if foo.bar() {}
}

fn desugar_test(foo: &mut Foo) {
	let x = Some(true);
	match x {
		Some(_) => {},
		_ => {
			if foo.bar() { }
		}
	}
}

fn desugar_error(foo: &mut Foo) {
	let x = Some(true);
	match x {
		Some(_) => {},
		_ if foo.bar() => {},
		_ => {}
	}
}

fn ok(foo: &mut Foo) {
    if let Some(_) = Some(true) {
    } else {
        if foo.bar() {}
    }
}

fn main() {}
