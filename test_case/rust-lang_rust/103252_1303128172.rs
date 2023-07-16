plain
diff of stderr:

23    |                                     ^^^^^^^^
24    |
25    = note: multiple `impl`s satisfying `_: TryFrom<&str>` found in the following crates: `core`, `std`:
-            - impl<> TryFrom<&str> for std::sys_common::net::LookupHost;
+            - impl<> TryFrom<&str> for std::sys::wasm::net::LookupHost;
27            - impl<T, U> TryFrom<U> for T
28              where U: Into<T>;
29    = note: required for `&str` to implement `TryInto<_>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/issue-72616.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-72616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72616.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-72616.rs:20:37
   |
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |                              |
   |                              type must be known at this point
   |
   |
   = note: multiple `impl`s satisfying `String: PartialEq<_>` found in the `alloc` crate:
           - impl PartialEq for String;
           - impl<'a, 'b> PartialEq<&'a str> for String;
           - impl<'a, 'b> PartialEq<Cow<'a, str>> for String;
           - impl<'a, 'b> PartialEq<str> for String;
   |
   |
LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}
   |                                 +++++++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72616.rs:20:37
   |
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |
   |
   = note: multiple `impl`s satisfying `_: TryFrom<&str>` found in the following crates: `core`, `std`:
           - impl<> TryFrom<&str> for std::sys::wasm::net::LookupHost;
           - impl<T, U> TryFrom<U> for T
             where U: Into<T>;
   = note: required for `&str` to implement `TryInto<_>`
   |
   |
LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}
   |                                 +++++++++++++++++++++++++++++++   ~
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
