
error: the `bar` method cannot be invoked on a trait object
  --> file.rs:15:15
   |
2  |     fn bar(&self) where Self: Sized { }
   |                               ----- this has a `Sized` requirement
...
15 |         foo().bar();
   |               ^^^
   |
   = note: another candidate was found in the following trait, perhaps add a `use` for it:
           `use Foo;`
