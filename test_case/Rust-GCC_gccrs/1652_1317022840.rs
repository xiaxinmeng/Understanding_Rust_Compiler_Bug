rust
#![allow(unused)]

fn main() {
    // Slice type
    let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
    let slice: &[i32] = &boxed_array[..];

    // Array type
    let array: [i32; 3] = [1, 2, 3];

    // Never type (panic! will not compile)
    let x: ! = bar();

    // Tuple type
    type A = ();
    type B = (i32, f64, Vec<String>, Option<bool>);
    type C = (String, i32);

    // Inferred type
    let x: Vec<_> = (0..10).collect();

    // Bare function type
    type Binop = fn(i32, i32) -> i32;
    type BinopC = extern "C" fn(i32, i32) -> i32;
    type BinopUC = unsafe extern "C" fn(i32, i32) -> i32;
    type Variadic = unsafe extern "C" fn(i32, ...) -> i32;

    type printf = extern "C" fn(format: *const c_char, ...) -> c_int;
    type printf2 = extern "C" fn(#[cfg(windows)] format: *const c_char, ...) -> c_int;

    // Impl trait object type
    trait Trait {}

    // argument position: anonymous type parameter
    fn foo(arg: impl Trait) {}

    // return position: abstract return type
    fn bar() -> impl Trait {}

    // impl Trait in argument position
    fn foo2(arg: impl Trait) {}

    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    // Trait object type
    trait Printable {
        fn stringify(&self) -> String;
    }

    impl Printable for i32 {
        fn stringify(&self) -> String {
            self.to_string()
        }
    }

    fn print(a: Box<dyn Printable>) {}

    fn main2() {
        print(Box::new(10) as Box<dyn Printable>);
    }

    type Q = dyn Trait;
    type W = dyn Trait + Send;
    type E = dyn Trait + Send + Sync;
    type R = dyn Trait + 'static;
    type T = dyn Trait + Send + 'static;
    type Y = dyn (Trait);

    // Parenthesised type
    type I<'a> = &'a (dyn Any + Send);
}
