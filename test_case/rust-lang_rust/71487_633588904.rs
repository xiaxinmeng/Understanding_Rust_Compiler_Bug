
error[E0201]: duplicate definitions with name `pos`:
   --> src/librustc_parse/lexer/mod.rs:423:5
    |
74  | /     pub fn pos(&self) -> BytePos {
75  | |         self.pos
76  | |     }
    | |_____- previous definition of `pos` here
...
423 | /     pub fn pos(&self) -> BytePos {
424 | |         self.pos
425 | |     }
    | |_____^ duplicate definition

error: aborting due to previous error

For more information about this error, try `rustc --explain E0201`.
error: could not compile `rustc_parse`.
