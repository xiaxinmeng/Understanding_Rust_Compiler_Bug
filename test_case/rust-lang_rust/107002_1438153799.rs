rust
#![feature(async_fn_in_trait)]
#![feature(min_specialization)]

struct MyStruct;

trait MyTrait<T> {
	async fn foo(_: T);
}

impl<T> MyTrait<T> for MyStruct {
	default async fn foo(_: T) {
		println!("default");
	}
}

impl MyTrait<i32> for MyStruct {
	async fn foo(_: i32) {
		println!("specialized");
	}
}

#[tokio::main]
async fn main() {
	MyStruct::foo(42).await;
	indirection(42).await;
}

async fn indirection<T>(x: T) {
	//explicit type coercion is currently necessary because of https://github.com/rust-lang/rust/issues/67918
	<MyStruct as MyTrait<T>>::foo(x).await;
}
