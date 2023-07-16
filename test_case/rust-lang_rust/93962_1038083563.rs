plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.69
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unused imports: `Greater`, `Less`
 --> library/core/src/slice/cmp.rs:4:34
  |
4 | use crate::cmp::Ordering::{self, Greater, Less};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:10
