plain

---- [ui] src/test/ui/traits/issue-77982.rs stdout ----
diff of stderr:

36            - impl AsRef<str> for String;
37 help: use the fully qualified path for the potential candidates
38    |
- LL |     opts.get(<str as AsRef<Path>>::as_ref(opt));
-    |              +++++++++++++++++++++++++++++   ~
- LL |     opts.get(<str as AsRef<OsStr>>::as_ref(opt));
-    |              ++++++++++++++++++++++++++++++   ~
- LL |     opts.get(<str as AsRef<str>>::as_ref(opt));
-    |              ++++++++++++++++++++++++++++   ~
- LL |     opts.get(<str as AsRef<[u8]>>::as_ref(opt));
-    |              +++++++++++++++++++++++++++++   ~
+ LL |     opts.get(<String as AsRef<Path>>::as_ref(opt));
+    |              ++++++++++++++++++++++++++++++++   ~
+ LL |     opts.get(<String as AsRef<OsStr>>::as_ref(opt));
+    |              +++++++++++++++++++++++++++++++++   ~
+ LL |     opts.get(<String as AsRef<str>>::as_ref(opt));
+    |              +++++++++++++++++++++++++++++++   ~
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |     opts.get(<String as AsRef<[u8]>>::as_ref(opt));
+    |              ++++++++++++++++++++++++++++++++   ~
48 
49 error[E0283]: type annotations needed



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ this method call resolves to `&T`
   |          |
   |          cannot infer type for type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the type argument in the function call
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:18
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |              |   |
   |              |   |
   |              |   cannot infer type for type parameter `T` declared on the trait `AsRef`
   |              this method call resolves to `&T`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl AsRef<[u8]> for String;
           - impl AsRef<str> for String;
help: use the fully qualified path for the potential candidates
   |
LL |     opts.get(<String as AsRef<Path>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ++++++++++++++++++++++++++++++++   ~
LL |     opts.get(<String as AsRef<OsStr>>::as_ref(opt)); //~ ERROR type annotations needed
   |              +++++++++++++++++++++++++++++++++   ~
LL |     opts.get(<String as AsRef<str>>::as_ref(opt)); //~ ERROR type annotations needed
   |              +++++++++++++++++++++++++++++++   ~
LL |     opts.get(<String as AsRef<[u8]>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ++++++++++++++++++++++++++++++++   ~

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ ----------- this method call resolves to `T`
   |                                            |
   |                                            cannot infer type for type parameter `T` declared on the trait `From`
   |
   = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
           - impl From<Ipv4Addr> for u32;
           - impl From<NonZeroU32> for u32;
           - impl From<bool> for u32;
           - impl From<char> for u32;
           and 3 more
error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
