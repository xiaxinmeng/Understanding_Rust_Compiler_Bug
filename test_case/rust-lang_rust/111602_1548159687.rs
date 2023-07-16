plain

---- [ui] tests/ui/consts/const-mut-refs/issue-76510.rs stdout ----
diff of 32bit.stderr:

19 LL | const S: &'static mut str = &mut " hello ";
20    |                             ^^^^^^^^^^^^^^ cannot borrow as mutable
- note: erroneous constant used
-   --> $DIR/issue-76510.rs:11:70
-    |
-    |
- LL |         let s = transmute::<(*const u8, usize), &ManuallyDrop<str>>((S.as_ptr(), 3));
- 
28 error: aborting due to 3 previous errors
29 
30 Some errors have detailed explanations: E0596, E0658, E0764.
30 Some errors have detailed explanations: E0596, E0658, E0764.


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/issue-76510.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-mut-refs/issue-76510.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-mut-refs/issue-76510.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/auxiliary"
stdout: none
error[E0764]: mutable references are not allowed in the final value of constants
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
   |
LL | const S: &'static mut str = &mut " hello ";

error[E0658]: mutation through a reference is not allowed in constants
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
   |
LL | const S: &'static mut str = &mut " hello ";
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0596]: cannot borrow data in a `&` reference as mutable
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
LL | const S: &'static mut str = &mut " hello ";
   |                             ^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
