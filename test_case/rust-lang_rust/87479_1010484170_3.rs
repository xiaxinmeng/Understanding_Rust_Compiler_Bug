
   Compiling playground v0.0.1 (/playground)
error: missing required bound on `Y`
 --> src/lib.rs:4:5
  |
4 |     type Y<'b>;
  |     ^^^^^^^^^^-
  |               |
  |               help: add the required where clause: `where 'b: 'a`
  |
  = note: this bound is currently required to ensure that impls have maximum flexibility
  = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information

error: could not compile `playground` due to previous error

