plain



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.no_flag/detect-extra-ub.no_flag.stderr
To only update this specific test, also pass `--test-args consts/extra-const-ub/detect-extra-ub.rs`


error in revision `no_flag`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_flag" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.no_flag" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.no_flag/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_ptr_read` has been stable since 1.65.0-nightly and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_read)]
   |            ^^^^^^^^^^^^^^
   |
---
59 Future incompatibility report: Future breakage diagnostic:


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.with_flag/detect-extra-ub.with_flag.stderr
To only update this specific test, also pass `--test-args consts/extra-const-ub/detect-extra-ub.rs`


error in revision `with_flag`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "with_flag" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.with_flag" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zextra-const-ub-checks" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/extra-const-ub/detect-extra-ub.with_flag/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_ptr_read` has been stable since 1.65.0-nightly and no longer requires an attribute to enable
   |
LL | #![feature(const_ptr_read)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(stable_features)]` on by default

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:9:20
   |
LL |     let _x: bool = transmute(3u8);
   |                    ^^^^^^^^^^^^^^ constructing invalid value: encountered 0x03, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:15:21
   |
   |
LL |     let _x: usize = transmute(&3u8);
   |                     ^^^^^^^^^^^^^^^ constructing invalid value: encountered (potentially part of) a pointer, but expected an integer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:22:30
   |
   |
LL |     let _x: (usize, usize) = transmute(x);
   |                              ^^^^^^^^^^^^ constructing invalid value at .0: encountered (potentially part of) a pointer, but expected an integer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:28:20
   |
   |
LL |     let _x: &u32 = transmute(&[0u8; 4]);
   |                    ^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 4 byte alignment but found 1)
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:1118:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         accessing memory with alignment 1, but alignment 4 is required
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1118:9
   |
  ::: /checkout/library/core/src/ptr/const_ptr.rs:1162:18
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:1162:18
  ::: /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:41:9
   |
LL |         ptr.read();
   |         ---------- inside `INNER` at /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:41:9
   |         ---------- inside `INNER` at /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:41:9

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:34:5
   |
LL | const UNALIGNED_READ: () = {
   | ------------------------
LL |     INNER; //[with_flag]~ERROR any use of this value will cause an error
   |
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
---
  --> /checkout/src/test/ui/consts/extra-const-ub/detect-extra-ub.rs:34:5
   |
LL | const UNALIGNED_READ: () = {
   | ------------------------
LL |     INNER; //[with_flag]~ERROR any use of this value will cause an error
   |
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
