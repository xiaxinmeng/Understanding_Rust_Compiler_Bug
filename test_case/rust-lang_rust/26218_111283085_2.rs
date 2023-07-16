 rust
> impl Index<i32> for Foo {
>     type Output = ();
> 
>     fn index(&self, i: i32) -> &() {
>         self.deref().index(i)
>     }
> }
> 