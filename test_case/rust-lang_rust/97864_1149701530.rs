plain
........................................................................................ 176/529
........................................................................................ 264/529
........................................................................................ 352/529
....i.......................................................................i........... 440/529
..................................F.......F............................................. 528/529
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] src/test/rustdoc/synthetic_auto/lifetimes.rs stdout ----
---- [rustdoc] src/test/rustdoc/synthetic_auto/lifetimes.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes" "--deny" "warnings" "/checkout/src/test/rustdoc/synthetic_auto/lifetimes.rs"
stdout: none
--- stderr -------------------------------
error: unnecessary lifetime parameter `'a`
  |
  |
7 |     'a: 'static,
  |
  |
  = note: `-D needless-lifetime` implied by `-D warnings`
  = help: you can use the `'static` lifetime directly, in place of `'a`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/synthetic_auto/project.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/project/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/project" "--deny" "warnings" "/checkout/src/test/rustdoc/synthetic_auto/project.rs"
stdout: none
--- stderr -------------------------------
error: unnecessary lifetime parameter `'a`
   |
   |
13 |     'a: 'static,
   |
   |
   = note: `-D needless-lifetime` implied by `-D warnings`
   = help: you can use the `'static` lifetime directly, in place of `'a`
error: unnecessary lifetime parameter `'a`
  --> /checkout/src/test/rustdoc/synthetic_auto/project.rs:19:5
   |
   |
19 |     'a: 'static,
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'a`
error: aborting due to 2 previous errors
------------------------------------------


