rust
use std::any::Any;

trait Trait: Any {
	fn into_any(self: Box<Self>) -> Box<Any>;
	fn into_any_send(self: Box<Self>) -> Box<Any+Send> where Self: Send;
	fn into_any_sync(self: Box<Self>) -> Box<Any+Sync> where Self: Sync;
	fn into_any_send_sync(self: Box<Self>) -> Box<Any+Send+Sync> where Self: Send+Sync;
}

impl<T> Trait for T where T: Any {
	fn into_any(self: Box<Self>) -> Box<Any> {
		self
	}
	fn into_any_send(self: Box<Self>) -> Box<Any+Send> where Self: Send {
		self
	}
	fn into_any_sync(self: Box<Self>) -> Box<Any+Sync> where Self: Sync {
		self
	}
	fn into_any_send_sync(self: Box<Self>) -> Box<Any+Send+Sync> where Self: Send+Sync {
		self
	}
}

fn main() {
	let a: usize = 123;
	let b: Box<Trait+Send+Sync> = Box::new(a);
	let c: Box<Any+Send+Sync> = b.into_any_send_sync();
	let _d: usize = *Box::<Any>::downcast(c).unwrap();
}

