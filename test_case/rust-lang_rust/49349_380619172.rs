plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:49:42] ..............................................................................i.....................
[00:49:48] .....................i.......................................................................F....F.
[00:49:52] F.F.F.F.............................................................................................
---
[00:50:31] i..........................................................................i........................
---
[00:50:54] 4 LL | const CON : Box<i32> = box 0; //~ ERROR E0010
[00:50:54] 5    |                        ^^^^^ allocation not allowed in constants
[00:50:54] -    |
[00:50:54] -    = note: The value of statics and constants must be known at compile time, and they live for the entire lifetime of a program. Creating a boxed value allocates memory on the heap at runtime, and therefore cannot be done at compile time.
---
[00:50:54] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0010-teach.rs'
[00:50:54]
[00:50:54] error: 1 errors occurred comparing output.
[00:50:54] status: exit code: 101
[00:50:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-co,"rendered":"error[E0010]: allocations are not allowed in constants\n  --> /checkout/src/test/ui/error-codes/E0010-teach.rs:16:24\n   |\nLL | const CON : Box<i32> = box 0; //~ ERROR E0010\n   |                        ^^^^^ allocation not allowed in constants\n\n"}
[00:50:54] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:54] {"message":"For more information about this error, try `rustc --explain E0010`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0010`.\n"}
---
[00:50:54] 4 LL |         Thing { x, y, z } => {}
[00:50:54] 5    |                       ^ struct `Thing` does not have this field
[00:50:54] -    |
[00:50:54] -    = note: This error indicates that a struct pattern attempted to extract a non-existent field from a struct. Struct fields are identified by the name used before the colon : so struct patterns should resemble the declaration of the struct type being matched.
[00:50:54] -
[00:50:54] -            If you are using shorthand field patterns but want to refer to the struct field by a different name, you shouldcause the compiler checks that the range is non-empty at compile-time, and is unable to evaluate arbitrary comparison functions. If you want to capture values of an orderable type between two end-points, you can use a guard.
---
[00:50:54] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0029-teach.rs'
[00:50:54]
[00:50:54] error: 1 errors occurred comparing output.
[00:50:54] status: exit code: 101
[00:50:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0029-teach.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0029-teach.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "teach" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0029-teach.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:50:54] {"message":"only char and numeric types are allowed in range patterns","code":{"code":"E0029","explanation":"\nIn a match expression, only numbers and characters can be matched against a\nrange. This is because the compiler checks that the range is non-empty at\ncompile-time, and is unable to evaluate arbitrary comparison functions. If you\nwant to capture values of an orderable type between two end-points, you can use\na guard.\n\n