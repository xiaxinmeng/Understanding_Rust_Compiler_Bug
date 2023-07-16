plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

43    |                                            |
44    |                                            required by a bound introduced by this call
45    |
-    = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
-            - impl From<Ipv4Addr> for u32;
-            - impl From<NonZeroU32> for u32;
-            - impl From<bool> for u32;
-            - impl From<char> for u32;
-            - impl From<u16> for u32;
-            - impl From<u8> for u32;
-            - impl<T> From<!> for T;
-            - impl<T> From<T> for T;
+    = note: cannot satisfy `u32: From<_>`
+    = help: the following types implement trait `From<T>`:
+              <u32 as From<Ipv4Addr>>
+              <u32 as From<NonZeroU16>>
+              <u32 as From<NonZeroU32>>
+              <u32 as From<NonZeroU8>>
+              <u32 as From<bool>>
+              <u32 as From<char>>
+              <u32 as From<u16>>
+              <u32 as From<u8>>
56    |
56    |
57 LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
  --> /rustc/FAKE_PREFIX/library/std/src/collections/hash/map.rs:876:5
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> fake-test-src-base/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^     ------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl AsRef<[u8]> for String;
           - impl AsRef<str> for String;
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> fake-test-src-base/traits/issue-77982.rs:13:59
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            |
   |                                            required by a bound introduced by this call
   |
   |
   = note: cannot satisfy `u32: From<_>`
   = help: the following types implement trait `From<T>`:
             <u32 as From<Ipv4Addr>>
             <u32 as From<NonZeroU16>>
             <u32 as From<NonZeroU32>>
             <u32 as From<NonZeroU8>>
             <u32 as From<bool>>
             <u32 as From<char>>
             <u32 as From<u16>>
             <u32 as From<u8>>
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

error[E0283]: type annotations needed for `Box<T>`
  --> fake-test-src-base/traits/issue-77982.rs:36:9
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         ^      --- type must be known at this point
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
  --> fake-test-src-base/traits/issue-77982.rs:29:1
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = ().foo(); //~ ERROR type annotations needed

error[E0283]: type annotations needed for `Box<T>`
  --> fake-test-src-base/traits/issue-77982.rs:40:9
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         ^         --- type must be known at this point
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
  --> fake-test-src-base/traits/issue-77982.rs:32:1
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = (&()).bar(); //~ ERROR type annotations needed

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
