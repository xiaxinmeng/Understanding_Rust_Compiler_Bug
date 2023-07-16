rust
type A<'a> = Box<Display + 'a>;
type B<'a> = Box<'a + Display>;
