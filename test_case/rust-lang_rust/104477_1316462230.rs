rust
mod case1 {
struct MyTy<'x, 'a, 'b>(std::cell::Cell<(&'x (), &'a u8, &'x &'b u8)>); // Diff here, extra parens
fn wf<T>(_: T) {}
fn test<'a, 'b>() {
|_: &'a u8, x: MyTy<'_, 'a, 'b>| wf(x);
|x: MyTy<'_, 'a, 'b>, _: &'a u8| wf(x);
}}
fn main() {}
