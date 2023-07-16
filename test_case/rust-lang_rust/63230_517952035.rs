rust
#![feature(never_type)]

struct MyStruct<T> {
	field_1: u8,
	field_2: T
}

fn make_struct<T, F: FnOnce() -> T>(f: F) -> MyStruct<T> {
	let foo: MyStruct<T>;
	foo.field_1 = 25;
	foo.field_2 = f();
	foo
}

fn boom() {
    let foo: MyStruct<!> = make_struct(|| panic!());
}
