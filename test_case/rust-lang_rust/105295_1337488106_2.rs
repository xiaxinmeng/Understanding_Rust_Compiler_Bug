
error[E0308]: method not compatible with trait
  --> src/main.rs:35:5
   |
35 |     fn listeners<'b>(&'b self)->&'a MessageListeners<'b> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected fn pointer `fn(&MessageListeners<'a>) -> &MessageListeners<'_>`
              found fn pointer `fn(&MessageListeners<'a>) -> &'a MessageListeners<'_>`
note: the lifetime `'a` as defined on the impl at 34:6...
  --> src/main.rs:34:6
   |
34 | impl<'a> MessageListenersInterface for MessageListeners<'a> {
   |      ^^
note: ...does not necessarily outlive the anonymous lifetime #1 defined on the method body at 35:5
  --> src/main.rs:35:5
   |
35 |     fn listeners<'b>(&'b self)->&'a MessageListeners<'b> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
