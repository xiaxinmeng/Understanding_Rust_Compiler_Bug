
error[E0391]: cycle detected when processing `foo`
 --> src/lib.rs:3:1
  |
3 | async fn foo(x: bool) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: ...which requires evaluating trait selection obligation `std::pin::PinBox<impl std::future::Future>: std::marker::Send`...
note: ...which requires processing `foo::{{impl-Trait}}`...
 --> src/lib.rs:3:26
  |
3 | async fn foo(x: bool) -> u32 {
  |                          ^^^
  = note: ...which again requires processing `foo`, completing the cycle
note: cycle used when processing `foo::{{closure}}`
 --> src/lib.rs:3:30
  |
3 |   async fn foo(x: bool) -> u32 {
  |  ______________________________^
4 | |     if x {
5 | |         let f: std::future::FutureObj<u32> = std::future::FutureObj::new(std::pin::PinBox::new(foo(false)));
6 | |         await!(f) + 1
... |
10| |     }
11| | }
  | |_^
