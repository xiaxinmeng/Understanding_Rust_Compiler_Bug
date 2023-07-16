rust
trait ClonableFn<T> {
	fn clone(&self) -> Box<dyn Fn(T)>;
}

impl<T, F: 'static> ClonableFn<T> for F
where F: Fn(T) + Clone {
	fn clone(&self) -> Box<dyn Fn(T)> {
		Box::new(self.clone())
	}
}

struct Foo(Box<dyn for<'a> ClonableFn<&'a bool>>);

fn main() {
	Foo(Box::new(|_: &bool| ()));
	//             ~~~~~~~ this is what changed
}
