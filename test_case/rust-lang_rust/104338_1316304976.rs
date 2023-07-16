plain
..........i............................................................................. 3256/13853
........................................................................................ 3344/13853
............................................................................iiiii....... 3432/13853
........................................................................................ 3520/13853
.................................F..F.F.........F....................................... 3608/13853
........................................................................................ 3784/13853
........................................................................................ 3872/13853
........................................................................................ 3960/13853
........................................................i............................... 4048/13853
---
failures:

---- [ui] src/test/ui/dyn-star/align.rs#normal stdout ----

error in revision `normal`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/align.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.normal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.normal/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/dyn-star/align.rs:4:12
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = note: `#[warn(incomplete_features)]` on by default


error[E0277]: `AlignedUsize` needs to be a pointer-sized type
  --> /checkout/src/test/ui/dyn-star/align.rs:15:13
   |
LL |     let x = AlignedUsize(12) as dyn* Debug;
   |             ^^^^^^^^^^^^^^^^ `AlignedUsize` needs to be a pointer-sized type
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
   = help: the trait `PointerSized` is not implemented for `AlignedUsize`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dyn-star/box.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/box.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/box/auxiliary" "-C" "opt-level=0"
stdout: none
--- stderr -------------------------------
error[E0277]: `Box<i32>` needs to be a pointer-sized type
   |
   |
LL |     Box::new(42) as dyn* Display
   |     ^^^^^^^^^^^^ `Box<i32>` needs to be a pointer-sized type
   = help: the trait `PointerSized` is not implemented for `Box<i32>`

error: aborting due to previous error

---
---- [ui] src/test/ui/dyn-star/drop.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/drop.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/drop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `Foo` needs to be a pointer-sized type
   |
   |
LL |     let _dyn_i: dyn* Debug = i;
   |                              ^ `Foo` needs to be a pointer-sized type
   = help: the trait `PointerSized` is not implemented for `Foo`

error: aborting due to previous error

---
---- [ui] src/test/ui/dyn-star/upcast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/upcast.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/upcast" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/upcast/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `W` needs to be a pointer-sized type
   |
   |
LL |     let w: dyn* Foo = W(0);
   |                       ^^^^ `W` needs to be a pointer-sized type
   = help: the trait `PointerSized` is not implemented for `W`

error: aborting due to previous error

