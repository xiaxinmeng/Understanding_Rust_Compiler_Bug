rust
error[E0425]: cannot find value `y` in this scope
 --> src/test/ui/rfc-2294-if-let-guard/bindings.rs:6:14
  |
6 |         _ => y, //~ ERROR cannot find value `y`
  |              ^
  |
help: the binding `y` is available in a different scope in the same function
 --> src/test/ui/rfc-2294-if-let-guard/bindings.rs:5:29
  |
5 |         Some(x) if let Some(y) = x => (x, y),
  |                             ^

error[E0425]: cannot find value `y` in this scope
 --> src/test/ui/rfc-2294-if-let-guard/bindings.rs:8:5
  |
8 |     y //~ ERROR cannot find value `y`
  |     ^ not found in this scope

error: aborting due to 2 previous errors
