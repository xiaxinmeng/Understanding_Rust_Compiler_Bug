
warning: `must_use` attribute on `async` functions applies to the anonymous Future returned by the function, not the value within.
  --> $DIR/unused-async.rs:5:1
   |
LL |   #[must_use]
   |   ^^^^^^^^^^^
LL |
LL | / async fn test() -> i32 {
LL | |     1
LL | | }
   | |_- this attribute does nothing, the Futures returned by async functions are already `must_use`
