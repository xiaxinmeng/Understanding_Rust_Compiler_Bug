rust
#![allow(unused)]

fn main() {
    struct B<T>
        where
            T: Iterator,            // Could use A<T: Iterator> instead
            T::Item: Copy,          // Bound on an associated type
            String: PartialEq<T>,   // Bound on `String`, using the type parameter
            i32: Default,           // Allowed, but not useful
    {
        f: T,
    }

    type Surface = i32;
    trait Shape {
        fn draw(&self, surface: Surface);
        fn name() -> &'static str;
    }

    fn draw_twice<T: Shape>(surface: Surface, sh: T) {
        sh.draw(surface);           // Can call method because T: Shape
        sh.draw(surface);
    }

    fn copy_and_draw_twice<T: Copy>(surface: Surface, sh: T) where T: Shape {
        let shape_copy = sh;        // doesn't move sh because T: Copy
        draw_twice(surface, sh);    // Can use generic function because T: Shape
    }

    struct Figure<S: Shape>(S, S);

    fn name_figure<U: Shape>(
        figure: Figure<U>,          // Type Figure<U> is well-formed because U: Shape
    ) {
        println!(
            "Figure of two {}",
            U::name(),              // Can use associated function
        );
    }

    struct A<'a, T>
        where
            i32: Default,           // Allowed, but not useful
            i32: Iterator,          // Error: `i32` is not an iterator
            &'a mut T: Copy,        // (at use) Error: the trait bound is not satisfied
            [T]: Sized,             // (at use) Error: size cannot be known at compilation
    {
        f: &'a T,
    }
    struct UsesA<'a, T>(A<'a, T>);

    fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
        y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
        let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
    }

    impl<'a> PartialEq<i32> for &'a T {
        // ...
    }

    fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
        let zero = 0;
        f(&zero);
    }

    fn call_on_ref_zero2<F>(f: F) where F: for<'a> Fn(&'a i32) {
        let zero = 0;
        f(&zero);
    }
}
