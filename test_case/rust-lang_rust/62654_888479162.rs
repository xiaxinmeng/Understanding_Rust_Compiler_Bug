
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> src/lib.rs:9:24
   |
8  |       fn test(&self) -> Box<dyn Future<Output=()> + 'static> {
   |               ----- this data with an anonymous lifetime `'_`...
9  |           Box::new(async {
   |  ________________________^
10 | |             foo(self).await
11 | |         })
   | |_________^ ...is captured here, requiring it to live as long as `'static`
   |
help: consider changing the trait object's explicit `'static` bound to the lifetime of argument `self`
   |
8  |     fn test(&self) -> Box<dyn Future<Output=()> + '_> {
   |                                                   ^^
help: alternatively, add an explicit `'static` bound to this reference
   |
8  |     fn test(&'static Test) -> Box<dyn Future<Output=()> + 'static> {
   |             ^^^^^^^^^^^^^
