plain
---- [run-pass-valgrind] src/test/run-pass-valgrind/coerce-match.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/coerce-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/coerce-match/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/coerce-match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
  |
  |
7 |         if true { let b: Box<_> = Box::new([1, 2, 3]); b } else { let b: Box<_> = box [1]; b };
  |
  = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
  = help: add `#![feature(box_syntax)]` to the crate attributes to enable

