
error[E0308]: `match` arms have incompatible types
  --> src/transitions.rs:26:14
   |
24 | /     match num {
25 | |         1 => { cx.answer_str("hi"); }
   | |                --------------------
   | |                |                  |
   | |                |                  help: consider removing this semicolon
   | |                this is found to be of type `()`
26 | |         _ => cx.answer_str("hi")
   | |              ^^^^^^^^^^^^^^^^^^^ expected `()`, found opaque type
27 | |     }
   | |_____- `match` arms have incompatible types
   |
  ::: /home/omer/rust/teloxide/src/dispatching/update_with_cx.rs:36:51
   |
36 |       pub async fn answer_str<T>(&self, text: T) -> ResponseResult<Message>
   |                                                     ----------------------- the `Output` of this `async fn`'s found opaque type
   |
   = note:     expected type `()`
           found opaque type `impl futures::Future`

error: aborting due to previous error
