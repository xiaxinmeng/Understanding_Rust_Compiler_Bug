
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
  --> src/lib.rs:14:68
   |
11 |     type SendMessageType = ();
   |     -------------------------- expected this associated type
...
14 |     fn send_message(&mut self, _message: Self::SendMessageType) -> Self::ReceiveMessageType {}
   |        ------------                                                ^^^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found `()`
   |        |
   |        implicitly returns `()` as its body has no tail or `return` expression
   |
   = note: expected associated type `<Meld as Hand>::ReceiveMessageType`
                    found unit type `()`

For more information about this error, try `rustc --explain E0308`.
