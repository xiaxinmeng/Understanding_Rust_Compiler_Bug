plain

---- [ui] src/test/rustdoc-ui/check-doc-alias-attr-location.rs stdout ----
diff of stderr:

- error: `#[doc(alias = "...")]` isn't allowed on extern block
+ error: `#[doc(alias = "...")]` isn't allowed on foreign module
3    |
3    |
4 LL | #[doc(alias = "foo")]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr-location/check-doc-alias-attr-location.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-doc-alias-attr-location.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-doc-alias-attr-location.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr-location" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr-location/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[doc(alias = "...")]` isn't allowed on foreign module
   |
   |
LL | #[doc(alias = "foo")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on implementation block
   |
   |
LL | #[doc(alias = "bar")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on implementation block
   |
   |
LL | #[doc(alias = "foobar")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on type alias in implementation block
   |
   |
LL |     #[doc(alias = "assoc")] //~ ERROR

error: aborting due to 4 previous errors
------------------------------------------

