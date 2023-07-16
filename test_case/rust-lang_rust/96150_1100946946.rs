plain

 finished in 94.280 seconds
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error: 2nd rule of macro `u8to64_le` is never used
  --> library/core/tests/hash/sip.rs:29:5
   |
29 |     ($buf:expr, $i:expr, $len:expr) => {{
   |
   |
   = note: `-D unused-macro-rules` implied by `-D warnings`

error: 2nd rule of macro `test_op` is never used
  --> library/core/tests/num/ops.rs:52:5
   |
52 |     ($fn_name:ident, $op:ident::$method:ident(&mut $lhs:literal, $rhs:literal), $result:literal, $($t:ty),+) => {

error: 1st rule of macro `test_op` is never used
  --> library/core/tests/num/ops.rs:46:5
   |
   |
46 |     ($fn_name:ident, $op:ident::$method:ident($lhs:literal, $rhs:literal), $result:literal, $($t:ty),+) => {

error: could not compile `core` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:16:37
