
error[E0277]: expected a `std::ops::Fn<(&usize,)>` closure, found `<F as Foo>::Bar`
  --> src/lib.rs:13:5
   |
13 | /     fn test<F>(this: F)
14 | |     where
15 | |         F: Foo<Arg = usize>,
16 | |     {
17 | |         let function: impl Fn(&F::Arg) = this.bar();
18 | |     }
   | |_____^ expected an `Fn<(&usize,)>` closure, found `<F as Foo>::Bar`
