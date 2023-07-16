plain

---- [ui] src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs stdout ----
diff of stderr:

4 LL |     assert::is_transmutable::<Src, Dst, Context, {0u8}, false, false, false>();
5    |                                                   ^^^ expected `bool`, found `u8`
6 
- error[E0080]: evaluation of `assert::is_transmutable::<test::Src, test::Dst, test::Context, {0u8}, false, false, false>::{constant#0}` failed
-   --> $DIR/wrong-type-assume.rs:26:15
-    |
- LL |             { from_options(ASSUME_ALIGNMENT, ASSUME_LIFETIMES, ASSUME_SAFETY, ASSUME_VALIDITY) }
- 
13 error[E0308]: mismatched types
14   --> $DIR/wrong-type-assume.rs:54:58
15    |
15    |

16 LL |     assert::is_transmutable::<Src, Dst, Context, false, {0u8}, false, false>();
17    |                                                          ^^^ expected `bool`, found `u8`
18 
- error[E0080]: evaluation of `assert::is_transmutable::<test::Src, test::Dst, test::Context, false, {0u8}, false, false>::{constant#0}` failed
-   --> $DIR/wrong-type-assume.rs:26:15
-    |
- LL |             { from_options(ASSUME_ALIGNMENT, ASSUME_LIFETIMES, ASSUME_SAFETY, ASSUME_VALIDITY) }
- 
25 error[E0308]: mismatched types
26   --> $DIR/wrong-type-assume.rs:55:65
27    |
27    |

28 LL |     assert::is_transmutable::<Src, Dst, Context, false, false, {0u8}, false>();
29    |                                                                 ^^^ expected `bool`, found `u8`
30 
- error[E0080]: evaluation of `assert::is_transmutable::<test::Src, test::Dst, test::Context, false, false, {0u8}, false>::{constant#0}` failed
-   --> $DIR/wrong-type-assume.rs:26:15
-    |
- LL |             { from_options(ASSUME_ALIGNMENT, ASSUME_LIFETIMES, ASSUME_SAFETY, ASSUME_VALIDITY) }
- 
37 error[E0308]: mismatched types
38   --> $DIR/wrong-type-assume.rs:56:72
39    |
39    |

40 LL |     assert::is_transmutable::<Src, Dst, Context, false, false, false, {0u8}>();
41    |                                                                        ^^^ expected `bool`, found `u8`
42 
- error[E0080]: evaluation of `assert::is_transmutable::<test::Src, test::Dst, test::Context, false, false, false, {0u8}>::{constant#0}` failed
-   --> $DIR/wrong-type-assume.rs:26:15
-    |
- LL |             { from_options(ASSUME_ALIGNMENT, ASSUME_LIFETIMES, ASSUME_SAFETY, ASSUME_VALIDITY) }
+ error: aborting due to 4 previous errors
48 
- error: aborting due to 8 previous errors
- 
---
To only update this specific test, also pass `--test-args transmutability/malformed-program-gracefulness/wrong-type-assume.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs:53:51
   |
   |
LL |     assert::is_transmutable::<Src, Dst, Context, {0u8}, false, false, false>(); //~ ERROR mismatched types
   |                                                   ^^^ expected `bool`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs:54:58
   |
   |
LL |     assert::is_transmutable::<Src, Dst, Context, false, {0u8}, false, false>(); //~ ERROR mismatched types
   |                                                          ^^^ expected `bool`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs:55:65
   |
   |
LL |     assert::is_transmutable::<Src, Dst, Context, false, false, {0u8}, false>(); //~ ERROR mismatched types
   |                                                                 ^^^ expected `bool`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/transmutability/malformed-program-gracefulness/wrong-type-assume.rs:56:72
   |
   |
LL |     assert::is_transmutable::<Src, Dst, Context, false, false, false, {0u8}>(); //~ ERROR mismatched types
   |                                                                        ^^^ expected `bool`, found `u8`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
