text
error: the `function` method cannot be invoked on a trait object
  --> src/main.rs:12:35
   |
4  |     fn function(&mut self) where Self: Sized;
   |                                        ----- this has a `Sized` requirement
...
12 |     (&mut Type as &mut dyn Trait).function();
   |                                   ^^^^^^^^
   |
   = note: you need `&dyn Trait` instead of `&mut dyn Trait`
