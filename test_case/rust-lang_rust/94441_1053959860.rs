plain

---- [ui] ui/simd/issue-94382.rs stdout ----
diff of stderr:

6 LL | |
7 LL | | struct U16x2(u16, u16);
8 ...  |
- LL | |     const Y0: i8 = unsafe { simd_extract(V, 0) };  
+ LL | |     const Y0: i8 = unsafe { simd_extract(V, 0) };
10 LL | | }
12 

13 error: any use of this value will cause an error
14   --> $DIR/issue-94382.rs:14:29
14   --> $DIR/issue-94382.rs:14:29
15    |
- LL |     const Y0: i8 = unsafe { simd_extract(V, 0) };  
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |     const Y0: i8 = unsafe { simd_extract(V, 0) };
18    |                             |
18    |                             |
19    |                             aborted execution: type `U16x2` is not `#[repr(simd)]`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-94382/issue-94382.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args simd/issue-94382.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-94382.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-94382" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-94382/auxiliary"
stdout: none
--- stderr -------------------------------
error: module has missing stability attribute
   |
   |
LL | / #![feature(platform_intrinsics)] //~ ERROR module has missing stability attribute
LL | | #![feature(staged_api)]
LL | |
LL | | struct U16x2(u16, u16);
...  |
LL | |     const Y0: i8 = unsafe { simd_extract(V, 0) };
LL | | }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/simd/issue-94382.rs:14:29
   |
   |
LL |     const Y0: i8 = unsafe { simd_extract(V, 0) };
   |                             |
   |                             |
   |                             aborted execution: type `U16x2` is not `#[repr(simd)]`
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors
------------------------------------------
