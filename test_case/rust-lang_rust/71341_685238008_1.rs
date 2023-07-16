
error[E0759]: `val` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
  --> src/lib.rs:14:9
   |
13 | fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                    ------------------- this data with lifetime `'a`...
14 |     val.use_self()
   |         ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
  --> src/lib.rs:9:22
   |
9  | impl MyTrait for dyn ObjectTrait {
   |                      ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
10 |     fn use_self(&self) -> &() { panic!() }
   |        -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
9  | impl MyTrait for dyn ObjectTrait + '_ {
   |                                  ^^^^
