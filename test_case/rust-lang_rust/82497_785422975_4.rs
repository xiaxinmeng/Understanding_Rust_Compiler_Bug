
---
-    |
- LL |     panic!("{}", 123);
-    |            ^^^^^
- 
- error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/panic-2021.rs:5:13
- LL |     panic!("{}");
-    |             ^^
- 
- 
- error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/panic-2021.rs:6:19
- LL |     core::panic!("{}");
-    |                   ^^
- 
- error: format argument must be a string literal
- error: format argument must be a string literal
-   --> $DIR/panic-2021.rs:7:20
-    |
- LL |     assert!(false, 123);
-    |
- help: you might be missing a string literal to format with
-    |
-    |
- LL |     assert!(false, "{}", 123);
- 
- 
- error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/panic-2021.rs:8:21
-    |
- LL |     assert!(false, "{}");
- 
- error: aborting due to 5 previous errors
- error: aborting due to 5 previous errors
+ error: edition 2021 is unstable and only available with -Z unstable-options.
43 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-2021/panic-2021.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/panic-2021.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/panic-2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: edition 2021 is unstable and only available with -Z unstable-options.

------------------------------------------



---- [ui] ui/privacy/pub-priv-dep/pub-priv1.rs stdout ----
diff of stderr:

- error: type `OtherType` from private dependency 'priv_dep' in public interface
-   --> $DIR/pub-priv1.rs:20:5
-    |
- LL |     pub field: OtherType,
-    |
- note: the lint level is defined here
-   --> $DIR/pub-priv1.rs:3:9
-    |
-    |
- LL | #![deny(exported_private_dependencies)]
- 
- 
- error: type `OtherType` from private dependency 'priv_dep' in public interface
-   --> $DIR/pub-priv1.rs:27:5
-    |
- LL |     pub fn pub_fn(param: OtherType) {}
- 
- 
- error: trait `OtherTrait` from private dependency 'priv_dep' in public interface
-   --> $DIR/pub-priv1.rs:34:5
-    |
- LL |     type Foo: OtherTrait;
- 
- error: aborting due to 3 previous errors
- error: aborting due to 3 previous errors
+ error: the `-Z unstable-options` flag must also be passed to enable `--extern options
27 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/pub-priv1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/pub-priv-dep/pub-priv1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/pub-priv-dep/pub-priv1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary" "--extern" "priv:priv_dep=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary/libpriv_dep.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `-Z unstable-options` flag must also be passed to enable `--extern options

------------------------------------------



---- [ui] ui/tool_lints.rs stdout ----
diff of stderr:

10 LL | #[warn(foo::bar)]
12 
12 
- error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
-   --> $DIR/tool_lints.rs:1:8
-    |
- LL | #[warn(foo::bar)]
- 
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
20 
20 
21 For more information about this error, try `rustc --explain E0710`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/tool_lints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args tool_lints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #[warn(foo::bar)]


error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #[warn(foo::bar)]

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0710`.
For more information about this error, try `rustc --explain E0710`.

------------------------------------------


---- [ui] ui/unknown-lint-tool-name.rs stdout ----
diff of stderr:

22 LL | #[allow(foo::bar)]
24 
24 
- error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
-   --> $DIR/unknown-lint-tool-name.rs:1:9
-    |
- LL | #![deny(foo::bar)]
- 
- 
- error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
-   --> $DIR/unknown-lint-tool-name.rs:5:9
-    |
- LL | #[allow(foo::bar)]
- 
- error: aborting due to 6 previous errors
+ error: aborting due to 4 previous errors
38 
38 
39 For more information about this error, try `rustc --explain E0710`.
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name/unknown-lint-tool-name.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unknown-lint-tool-name.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unknown-lint-tool-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #![deny(foo::bar)] //~ ERROR an unknown tool name found in scoped lint: `foo::bar`


error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #[allow(foo::bar)] //~ ERROR an unknown tool name found in scoped lint: `foo::bar`


error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #![deny(foo::bar)] //~ ERROR an unknown tool name found in scoped lint: `foo::bar`


error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
   |
   |
LL | #[allow(foo::bar)] //~ ERROR an unknown tool name found in scoped lint: `foo::bar`

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0710`.
For more information about this error, try `rustc --explain E0710`.

------------------------------------------


---- [ui] ui/unused-crate-deps/extern-loc-bad-loctype.rs stdout ----
diff of stderr:

- error: unknown location type `badloc`: use `raw` or `json`
+ error: `--extern-location` option is unstable: set `-Z unstable-options`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-bad-loctype/extern-loc-bad-loctype.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-bad-loctype.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-bad-loctype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-bad-loctype" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=badloc:in-the-test-file" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-bad-loctype/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-bad-loctype/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-json-bad-json.rs stdout ----
diff of stderr:

- error: `--extern-location`: malformed json location `[{"malformed`
+ error: `--extern-location` option is unstable: set `-Z unstable-options`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/extern-loc-json-bad-json.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-json-bad-json.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json-bad-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:[{\"malformed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-bad-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-json-json.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:{\"key\":123,\"value\":{}}" "--error-format" "json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-json.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=json:{\"key\":123,\"value\":{}}" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-raw-json.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-raw-json.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=raw:in-the-test-file" "--error-format" "json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-json/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-missing-loctype.rs stdout ----
diff of stderr:

- error: unknown location type `missing-loc-type`: use `raw` or `json`
+ error: `--extern-location` option is unstable: set `-Z unstable-options`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loctype/extern-loc-missing-loctype.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-missing-loctype.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-missing-loctype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loctype" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=missing-loc-type" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loctype/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loctype/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-missing-loc.rs stdout ----
diff of stderr:

- error: `--extern-location`: specify location for extern crate `bar`
+ error: `--extern-location` option is unstable: set `-Z unstable-options`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loc/extern-loc-missing-loc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-missing-loc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-missing-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loc/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-missing-loc/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-raw.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-raw.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=raw:in-the-test-file" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------



---- [ui] ui/unused-crate-deps/extern-loc-raw-missing-loc.rs stdout ----
diff of stderr:

- error: `--extern-location`: missing `raw` location
+ error: `--extern-location` option is unstable: set `-Z unstable-options`
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/extern-loc-raw-missing-loc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/extern-loc-raw-missing-loc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/extern-loc-raw-missing-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern-location" "bar=raw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/extern-loc-raw-missing-loc/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `--extern-location` option is unstable: set `-Z unstable-options`

------------------------------------------


---
test result: FAILED. 11362 passed; 36 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:19
