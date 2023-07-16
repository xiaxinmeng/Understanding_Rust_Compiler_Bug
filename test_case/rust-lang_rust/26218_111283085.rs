 rust
> use std::ops::{Deref, Index};
> 
> pub struct Foo(Bar);
> 
> pub struct Bar;
> 
> impl Deref for Foo {
>     type Target = Bar;
> 
>     fn deref(&self) -> &Bar { &self.0 }
> }
> 
> impl Index<()> for Foo {
>     type Output = ();
> 
>     fn index(&self, _: ()) -> &() {
>         unimplemented!()
>     }
> }
> 
> impl Index<i32> for Bar {
>     type Output = ();
> 
>     fn index(&self, _: i32) -> &() {
>         unimplemented!()
>     }
> }
> 
> fn test(foo: Foo) {
>     foo[()]; // OK
>     foo[0];
>     //~^ error: expected `()` found `i32` (BUT, it should work just like the line below)
>     foo.deref()[0]; // OK
> }
> 
> fn main() {}
> 