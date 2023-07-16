
error: expected identifier, found keyword `fn`
 --> src/lib.rs:2:15
  |
2 | struct FnS<F: fn()> {
  |               ^^
  |
help: use `Fn` to refer to the trait
  |
2 | struct FnS<F: Fn()> {
  |               ~~

error: expected identifier, found keyword `in`
 --> src/lib.rs:6:15
  |
6 | struct InS<I: in> {
  |               ^^ expected identifier, found keyword

error: expected identifier, found keyword `async`
  --> src/lib.rs:11:18
   |
11 | struct AsyncS<A: async> {
   |                  ^^^^^ expected identifier, found keyword

error[E0405]: cannot find trait `r#in` in this scope
 --> src/lib.rs:6:15
  |
6 | struct InS<I: in> {
  |               ^^ help: a trait with a similar name exists: `Fn`
 --> /rustc/270c94e484e19764a2832ef918c95224eb3f17c7/library/core/src/ops/function.rs:158:1
  |
  = note: similarly named trait `Fn` defined here

error[E0405]: cannot find trait `r#async` in this scope
  --> src/lib.rs:11:18
   |
11 | struct AsyncS<A: async> {
   |                  ^^^^^ not found in this scope

error[E0405]: cannot find trait `union` in this scope
  --> src/lib.rs:16:18
   |
16 | struct UnionS<U: union> {
   |                  ^^^^^ not found in this scope
