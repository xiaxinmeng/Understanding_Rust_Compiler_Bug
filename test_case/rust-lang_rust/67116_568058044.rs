
note: future is not `Send` as this value is used across an await
  --> $DIR/nested-async-calls.rs:17:5
LL | async fn third() {
   |          ----- in function `third`
LL |     let _a: Outer;
   |         -- has type `third::{{closure}}#0::Outer`
LL |     dummy().await;
   |     ^^^^^^^^^^^^^ await occurs here, with `_a` maybe used later
LL | }
   | - `_a` is later dropped here
note: `third` was invoked by `second`
note: `second` was invoked by `first` which was passed to `require_send`
