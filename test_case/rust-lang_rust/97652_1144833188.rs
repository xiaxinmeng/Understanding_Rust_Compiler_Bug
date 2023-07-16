plain
---- [run-pass-valgrind] src/test/run-pass-valgrind/cast-enum-with-dtor.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/cast-enum-with-dtor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/cast-enum-with-dtor/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/cast-enum-with-dtor/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot cast enum `E` into integer `u32` because it implements `Drop`
   |
   |
30 |         assert_eq!(e as u32, 2);
   |
   |
   = note: `#[deny(cenum_impl_drop_cast)]` on by default
   = note: for more information, see issue #73333 <https://github.com/rust-lang/rust/issues/73333>


warning: unnecessary `unsafe` block
   |
   |
20 |         unsafe { mem::forget(mem::replace(self, E::B)) };
   |         ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

error: aborting due to previous error; 1 warning emitted
------------------------------------------
