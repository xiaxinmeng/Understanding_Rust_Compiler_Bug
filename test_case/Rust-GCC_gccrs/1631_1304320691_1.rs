rust
fn main() {
	struct B<T> where
		T: Iterator,
		T::Item: Copy,
		String: PartialEq<Ambiguous: T>,
		i32: Default,
{
		f: T,
	}
; /* stmt */
	type Surface = i32;
; /* stmt */
	trait Shape{
				fn draw(&self, surface: Surface) ;

				fn name() -> str;

	}
; /* stmt */
	fn draw_twice<T>(surface: Surfacesh: T) {
		; /* stmt */
		; /* stmt */
	}

; /* stmt */
	fn copy_and_draw_twice<T>(surface: Surfacesh: T)  where
		T: Shape,
{
		let shape_copy = sh; /* stmt */
		draw_twice(
			surface,
			sh,
		); /* stmt */
	}

; /* stmt */
	struct Figure<S>(S, S);
; /* stmt */
	fn name_figure<U>(figure: Figure<Ambiguous: U>) {
		; /* stmt */
	}

; /* stmt */
	struct A<'a, T> where
		i32: Default,
		i32: Iterator,
		T: Copy,
		T: Sized,
{
		f: T,
	}
; /* stmt */
	struct UsesA<'a, T>(A<'aAmbiguous: T>);
; /* stmt */
	fn f<'a, 'b>(x: i32y: i32)  where
		'a: 'b,
{
		y = x; /* stmt */
		let r: i32 = &&0; /* stmt */
	}

; /* stmt */
	impl PartialEq<Ambiguous: i32> for T {

}
; /* stmt */
	fn call_on_ref_zero<F>(f: F)  where
		for <'a> F: Fn(&'a i32),
{
		let zero = 0; /* stmt */
		f(
			&zero,
		); /* stmt */
	}

; /* stmt */
	fn call_on_ref_zero2<F>(f: F)  where
		F: for <'a> Fn(&'a i32),
{
		let zero = 0; /* stmt */
		f(
			&zero,
		); /* stmt */
	}

; /* stmt */
