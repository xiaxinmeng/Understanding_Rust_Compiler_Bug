rust

trait MyTrait {
}

trait Wrapper<T: MyTrait> {
}

mod inner {
	trait MyTrait {
	}
	fn method<T: MyTrait, W: super::Wrapper<T>>() {
	}
}

fn main() {}

