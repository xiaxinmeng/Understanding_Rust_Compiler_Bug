\nenum something_that_does_exist {\n  t":1,"column_end":1,"is_primary":true,"text":[{"text":"struct Test;","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::prelude::v1::drop;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0425]: cannot find function `drop` in this scope\n  --> /checkout/src/test/ui/no-implicit-prelude.rs:27:5\n   |\nLL |     drop(2) //~ ERROR cannot find function `drop` in this scope\n   |     ^^^^ not found in this scope\nhelp: possible candidates are found in other modules, you can import them into scope\n   |\nLL | use std::mem::drop;\n   |\nLL | use std::prelude::v1::drop;\n   |\n\n"}
[00:47:57] thread 'main' panicked at 'internal error: entered unreachable code', librustc_typeck/astconv.rs:696:18
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:47:57] {"message":"Some errors occurred: E0405, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0405, E0425.\n"}
[00:47:57] {"message":"For more information about an error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0405`.\n"}
[00:47:57] error: internal compiler error: unexpected panic
[00:47:57] 
[00:47:57] note: the compiler unexpectedly panicked. this is a bug.
[00:47:57] 
[00:47:57] 
[00:47:57] note: we would appreciate a bug report: https://github.comhis scope\n   |              ^^^ not found in this scope\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL |         use std::ops::Add;\n   |\n\n"}
[00:47:57] {"message":"cannot find trait `Clone` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n