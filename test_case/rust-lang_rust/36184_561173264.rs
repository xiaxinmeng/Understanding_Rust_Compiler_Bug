rust

trait MyTrait {
}

struct Wrapper<T: MyTrait>(std::marker::PhantomData<T>);

struct Value;
impl MyTrait for Value {
}


mod inner {

	trait MyTrait {
	}

	fn method<T: MyTrait>() {
	}

	fn call() {
		method::<super::Wrapper<super::Value>>();
	}
}
