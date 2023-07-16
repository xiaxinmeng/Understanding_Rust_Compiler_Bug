txt
  --> $DIR/issue-64130-4-async-move.rs:21:26
   |
LL |         match client.status() {
   |               ------ has type `&Client`
LL |             200 => {
LL |                 let _x = get().await;
   |                          ^^^^^^^^^^^ await occurs here, with `client` maybe used later
...
LL |     }
   |     - `client` is later dropped here
   = note: the return type of a function must have a statically known size
