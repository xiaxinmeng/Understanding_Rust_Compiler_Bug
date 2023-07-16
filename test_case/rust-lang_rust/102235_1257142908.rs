plain
---- [ui] src/test/ui/extern/extern-compare-with-return-type.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-compare-with-return-type/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: voidret1 as extern \"C\" fn() != voidret2 as extern \"C\" fn()', /checkout/src/test/ui/extern/extern-compare-with-return-type.rs:19:5
------------------------------------------


---- [ui] src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
---- [ui] src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
diff of run.stderr:

- [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
- [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
- [$DIR/dbg-macro-expected-behavior.rs:27] Point { x: 42, y: 24 } = Point {
-     x: 42,
-     y: 24,
- }
- [$DIR/dbg-macro-expected-behavior.rs:28] b = Point {
-     x: 42,
-     y: 24,
- [$DIR/dbg-macro-expected-behavior.rs:36]
- [$DIR/dbg-macro-expected-behavior.rs:36]
- [$DIR/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
- )
- )
- [$DIR/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
- )
- )
- [$DIR/dbg-macro-expected-behavior.rs:45] f(&42) = 42
19 before
- [$DIR/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
- [$DIR/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
-     "Yeah",
- )
- [$DIR/dbg-macro-expected-behavior.rs:61] 1 = 1
- [$DIR/dbg-macro-expected-behavior.rs:61] 2 = 2
- [$DIR/dbg-macro-expected-behavior.rs:65] 1u8 = 1
- [$DIR/dbg-macro-expected-behavior.rs:65] 2u32 = 2
- [$DIR/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
stdout: none
