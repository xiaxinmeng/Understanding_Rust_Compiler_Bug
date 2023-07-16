rust
fn main() {
	let boxed_array: Box<[i32]> = Box::new(
		[1, 2, 3],
	);
	let slice: &[i32] = &boxed_array[..];
	let array: [i32; 3] = [1, 2, 3];
	let x: ! = bar(
	);
	type A = ();
	type B = (i32, f64, Vec<String>, Option<bool>);
	type C = (String, i32);
	let x: Vec<_> = ;
	type Binop = fn (i32, i32) -> i32;
	type BinopC = extern "C" fn (i32, i32) -> i32;
	type BinopUC = unsafe extern "C" fn (i32, i32) -> i32;
	type Variadic = unsafe extern "C" fn (i32, ...) -> i32;
	type printf = extern "C" fn ( format: *const c_char, ...) -> c_int;
	type printf2 = extern "C" fn (#[cfg = FIXME] format: *const c_char, ...) -> c_int;
	trait Trait{}
;
	fn foo(arg: impl Trait) {
	}

;
	fn bar() -> impl Trait {
	}

;
	fn foo2(arg: impl Trait) {
	}

;
	fn returns_closure() -> impl Fn(i32)->i32 {
		 /* tail expr */

	}

;
	trait Printable{
				fn stringify(&self) -> String;

	}
;
	impl Printable for i32 {
				fn stringify(&self) -> String {
			 /* tail expr */

		}


}
;
	fn print(a: Box<dyn Printable>) {
	}

;
	fn main2() {
		print(
			Box::new(
				10,
			) as Box<dyn Printable>,
		);
	}

;
	type Q = dyn Trait;
	type W = dyn Trait + Send;
	type E = dyn Trait + Send + Sync;
	type R = dyn Trait + 'static;
	type T = dyn Trait + Send + 'static;
	type Y = dyn Trait;
	type I<'a> = &'a (dyn Any + Send);
}

