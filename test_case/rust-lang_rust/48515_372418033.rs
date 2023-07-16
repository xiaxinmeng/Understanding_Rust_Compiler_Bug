
#![feature(specialization)]
trait Type {
	const FOO: bool;
}
default impl<T> Type for T {
	const FOO: bool = false;
}
fn main() {
    <u8 as Type>::FOO;
}
