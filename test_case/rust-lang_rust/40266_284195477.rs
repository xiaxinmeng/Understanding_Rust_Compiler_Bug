
failures:

---- [ui] ui/macros/format-foreign.rs stdout ----
	ui: /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs
normalized stderr:
error: multiple unused formatting arguments
  --> $DIR/format-foreign.rs:12:5
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: argument never used
  --> $DIR/format-foreign.rs:12:40
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                                        ^^^^^^^
note: argument never used
  --> $DIR/format-foreign.rs:12:49
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                                                 ^
note: argument never used
  --> $DIR/format-foreign.rs:12:49
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                                                 ^
   = help: `%.*3$s` should be written as `{:.2$}`
   = help: `%s` should be written as `{}`
   = note: printf formatting not supported; see the documentation for `std::fmt`
   = note: this error originates in a macro outside of the current crate

error: argument never used
  --> $DIR/format-foreign.rs:13:29
   |
13 |     println!("%1$*2$.*3$f", 123.456);
   |                             ^^^^^^^
   |
   = help: `%1$*2$.*3$f` should be written as `{0:1$.2$}`
   = note: printf formatting not supported; see the documentation for `std::fmt`

error: argument never used
  --> $DIR/format-foreign.rs:17:30
   |
17 |     println!("{} %f", "one", 2.0);
   |                              ^^^

error: named argument never used
  --> $DIR/format-foreign.rs:19:39
   |
19 |     println!("Hi there, $NAME.", NAME="Tim");
   |                                       ^^^^^
   |
   = help: `$NAME` should be written as `{NAME}`
   = note: shell formatting not supported; see the documentation for `std::fmt`

error: aborting due to 4 previous errors



expected stderr:
error: multiple unused formatting arguments
  --> $DIR/format-foreign.rs:12:5
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: argument never used
  --> $DIR/format-foreign.rs:12:30
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                              ^^^^^^^^
note: argument never used
  --> $DIR/format-foreign.rs:12:40
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                                        ^^^^^^^
note: argument never used
  --> $DIR/format-foreign.rs:12:49
   |
12 |     println!("%.*3$s %s!/n", "Hello,", "World", 4);
   |                                                 ^
   = help: `%.*3$s` should be written as `{:.2$}`
   = help: `%s` should be written as `{}`
   = note: printf formatting not supported; see the documentation for `std::fmt`
   = note: this error originates in a macro outside of the current crate

error: argument never used
  --> $DIR/format-foreign.rs:13:29
   |
13 |     println!("%1$*2$.*3$f", 123.456);
   |                             ^^^^^^^
   |
   = help: `%1$*2$.*3$f` should be written as `{0:1$.2$}`
   = note: printf formatting not supported; see the documentation for `std::fmt`

error: argument never used
  --> $DIR/format-foreign.rs:17:30
   |
17 |     println!("{} %f", "one", 2.0);
   |                              ^^^

error: named argument never used
  --> $DIR/format-foreign.rs:19:39
   |
19 |     println!("Hi there, $NAME.", NAME="Tim");
   |                                       ^^^^^
   |
   = help: `$NAME` should be written as `{NAME}`
   = note: shell formatting not supported; see the documentation for `std::fmt`

error: aborting due to 4 previous errors



diff of stderr:

  7 - |  --> $DIR/format-foreign.rs:12:30|
    + |  --> $DIR/format-foreign.rs:12:40|

 10 - |   |                              ^^^^^^^^|
    + |   |                                        ^^^^^^^|

 12 - |  --> $DIR/format-foreign.rs:12:40|
    + |  --> $DIR/format-foreign.rs:12:49|

 15 - |   |                                        ^^^^^^^|
    + |   |                                                 ^|


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign.stderr
To update references, run this command from build directory:
/home/mark/Build/rust/src/test/ui/update-references.sh '/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui' 'macros/format-foreign.rs'

error: 1 errors occurred comparing output.
status: exit code: 101
command: /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs -L /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui --target=x86_64-unknown-linux-gnu -L /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign.stage2-x86_64-unknown-linux-gnu.ui.libaux -C prefer-dynamic -o /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: multiple unused formatting arguments
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:12:5
   |
12 |     println!("%.*3$s %s!\n", "Hello,", "World", 4);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:12:40
   |
12 |     println!("%.*3$s %s!\n", "Hello,", "World", 4);
   |                                        ^^^^^^^
note: argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:12:49
   |
12 |     println!("%.*3$s %s!\n", "Hello,", "World", 4);
   |                                                 ^
note: argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:12:49
   |
12 |     println!("%.*3$s %s!\n", "Hello,", "World", 4);
   |                                                 ^
   = help: `%.*3$s` should be written as `{:.2$}`
   = help: `%s` should be written as `{}`
   = note: printf formatting not supported; see the documentation for `std::fmt`
   = note: this error originates in a macro outside of the current crate

error: argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:13:29
   |
13 |     println!("%1$*2$.*3$f", 123.456);
   |                             ^^^^^^^
   |
   = help: `%1$*2$.*3$f` should be written as `{0:1$.2$}`
   = note: printf formatting not supported; see the documentation for `std::fmt`

error: argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:17:30
   |
17 |     println!("{} %f", "one", 2.0);
   |                              ^^^

error: named argument never used
  --> /home/mark/Build/rust/src/test/ui/macros/format-foreign.rs:19:39
   |
19 |     println!("Hi there, $NAME.", NAME="Tim");
   |                                       ^^^^^
   |
   = help: `$NAME` should be written as `{NAME}`
   = note: shell formatting not supported; see the documentation for `std::fmt`

error: aborting due to 4 previous errors


------------------------------------------

thread '[ui] ui/macros/format-foreign.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2637
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [ui] ui/macros/macro-backtrace-println.rs stdout ----
	ui: /home/mark/Build/rust/src/test/ui/macros/macro-backtrace-println.rs
normalized stderr:
error: invalid reference to argument `0` (no arguments given)
  --> $DIR/macro-backtrace-println.rs:24:36
   |
24 |     ($fmt:expr) => (myprint!(concat!($fmt, "/n")));
   |                                    ^^^^^^^^^^^^^
...
28 |     myprintln!("{}");
   |     ----------------- in this macro invocation

error: aborting due to previous error



expected stderr:
error: invalid reference to argument `0` (no arguments given)
  --> $DIR/macro-backtrace-println.rs:24:30
   |
24 |     ($fmt:expr) => (myprint!(concat!($fmt, "/n")));
   |                              ^^^^^^^^^^^^^^^^^^^
...
28 |     myprintln!("{}");
   |     ----------------- in this macro invocation

error: aborting due to previous error



diff of stderr:

  1 - |  --> $DIR/macro-backtrace-println.rs:24:30|
    + |  --> $DIR/macro-backtrace-println.rs:24:36|

  4 - |   |                              ^^^^^^^^^^^^^^^^^^^|
    + |   |                                    ^^^^^^^^^^^^^|


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-println.stderr
To update references, run this command from build directory:
/home/mark/Build/rust/src/test/ui/update-references.sh '/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui' 'macros/macro-backtrace-println.rs'

error: 1 errors occurred comparing output.
status: exit code: 101
command: /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/mark/Build/rust/src/test/ui/macros/macro-backtrace-println.rs -L /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui --target=x86_64-unknown-linux-gnu -L /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-println.stage2-x86_64-unknown-linux-gnu.ui.libaux -C prefer-dynamic -o /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-println.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: invalid reference to argument `0` (no arguments given)
  --> /home/mark/Build/rust/src/test/ui/macros/macro-backtrace-println.rs:24:36
   |
24 |     ($fmt:expr) => (myprint!(concat!($fmt, "\n")));
   |                                    ^^^^^^^^^^^^^
...
28 |     myprintln!("{}");
   |     ----------------- in this macro invocation

error: aborting due to previous error


------------------------------------------

thread '[ui] ui/macros/macro-backtrace-println.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2637


failures:
    [ui] ui/macros/format-foreign.rs
    [ui] ui/macros/macro-backtrace-println.rs

test result: FAILED. 190 passed; 2 failed; 1 ignored; 0 measured
