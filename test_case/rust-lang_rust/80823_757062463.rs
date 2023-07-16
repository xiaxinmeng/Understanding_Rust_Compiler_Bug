plain
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
................................................................................iiiiii..iiiiii.i.... 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.071 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.23s

 finished in 2.304 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 500/542
..........................................
failures:

---- src/borrow.rs - borrow::Cow::disown_if (line 337) stdout ----
error[E0282]: type annotations needed
 --> src/borrow.rs:341:16
  |
7 | assert_eq!(cow.disown_if(moo, |o| o.to_lowercase() == moo), Some(String::from("Moo")));
  |                ^^^^^^^^^ cannot infer type
  |
  = note: type must be known at this point

error[E0599]: no method named `disown_if` found for enum `Cow<'_, _>` in the current scope
 --> src/borrow.rs:341:16
  |
7 | assert_eq!(cow.disown_if(moo, |o| o.to_lowercase() == moo), Some(String::from("Moo")));
  |                ^^^^^^^^^ method not found in `Cow<'_, _>`
  |
  = note: `cow` is a function, perhaps you wish to call it

error[E0599]: no method named `disown_if` found for enum `Cow<'_, _>` in the current scope
 --> src/borrow.rs:342:16
  |
8 | assert_eq!(cow.disown_if(moo, |o| o.to_lowercase() == moo), None);
  |                ^^^^^^^^^ method not found in `Cow<'_, _>`
  |
  = note: `cow` is a function, perhaps you wish to call it
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0282, E0599.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
Couldn't compile the test.
---- src/borrow.rs - borrow::Cow::disown_if_eq (line 372) stdout ----
error[E0282]: type annotations needed
 --> src/borrow.rs:376:16
  |
7 | assert_eq!(cow.disown_if_eq(moo), Some(moo.to_string()));
  |                ^^^^^^^^^^^^ cannot infer type
  |
  = note: type must be known at this point

error[E0599]: no method named `disown_if_eq` found for enum `Cow<'_, _>` in the current scope
 --> src/borrow.rs:376:16
  |
7 | assert_eq!(cow.disown_if_eq(moo), Some(moo.to_string()));
  |                ^^^^^^^^^^^^ method not found in `Cow<'_, _>`
  |
  = note: `cow` is a function, perhaps you wish to call it
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0599.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
Couldn't compile the test.

failures:
    src/borrow.rs - borrow::Cow::disown_if (line 337)
    src/borrow.rs - borrow::Cow::disown_if_eq (line 372)
test result: FAILED. 539 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out; finished in 12.01s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:03
