plain
.................................................................................................... 300/12155
.................................................................................................... 400/12155
.................................................................................................... 500/12155
.................................................................................................... 600/12155
.............................F.........F...........i................................................ 700/12155
.................................................................................................... 900/12155
.................................................................................................... 1000/12155
.................................................................................................... 1100/12155
..........................................i......................................................... 1200/12155
---
..................................................................................i....i.i.......... 7900/12155
.................................................ii................................................. 8000/12155
.................................................................................................... 8100/12155
....................................................i............................................... 8200/12155
.............................................i.................................F.....F.............. 8300/12155
...........................................................F........................i............... 8400/12155
......................................................F........F.................................... 8500/12155
.................................................................................................... 8700/12155
.................................................................................................... 8800/12155
.................................................................................................... 8900/12155
.................................................................................................... 9000/12155
---
diff of stderr:

2   --> $DIR/no-async-const.rs:4:11
3    |
4 LL | pub async const fn x() {}
-    |           ^^^^^ expected one of `extern`, `fn`, or `unsafe`
+    |     |     |
+    |     |     |
+    |     |     expected one of `extern`, `fn`, or `unsafe`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |     help: `const` must come before `async`: `const async`
+    |
+    = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-async-const/no-async-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/no-async-const.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-async-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-async-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-async-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `extern`, `fn`, or `unsafe`, found keyword `const`
   |
   |
LL | pub async const fn x() {}
   |     |     |
   |     |     |
   |     |     expected one of `extern`, `fn`, or `unsafe`
   |     help: `const` must come before `async`: `const async`
   |
   = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/no-unsafe-async.rs stdout ----
diff of stderr:

5    |        - while parsing this item list starting here
6 LL |     #[cfg(FALSE)]
7 LL |     unsafe async fn g() {}
-    |            ^^^^^ expected one of `extern` or `fn`
+    |     |      |
+    |     |      |
+    |     |      expected one of `extern` or `fn`
+    |     help: `async` must come before `unsafe`: `async unsafe`
9 LL | }
10    | - the item list ends here
+    |
+    = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
11 
12 error: expected one of `extern` or `fn`, found keyword `async`

14    |
14    |
15 LL | unsafe async fn f() {}
-    |        ^^^^^ expected one of `extern` or `fn`
+    | |      |
+    | |      |
+    | |      expected one of `extern` or `fn`
+    | help: `async` must come before `unsafe`: `async unsafe`
+    |
+    = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
18 error: aborting due to 2 previous errors
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-unsafe-async/no-unsafe-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/no-unsafe-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-unsafe-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-unsafe-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-unsafe-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `extern` or `fn`, found keyword `async`
   |
LL | impl S {
   |        - while parsing this item list starting here
LL |     #[cfg(FALSE)]
LL |     #[cfg(FALSE)]
LL |     unsafe async fn g() {} //~ ERROR expected one of `extern` or `fn`, found keyword `async`
   |     |      |
   |     |      |
   |     |      expected one of `extern` or `fn`
   |     help: `async` must come before `unsafe`: `async unsafe`
LL | }
   | - the item list ends here
   |
   = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`

error: expected one of `extern` or `fn`, found keyword `async`
   |
   |
LL | unsafe async fn f() {} //~ ERROR expected one of `extern` or `fn`, found keyword `async`
   | |      |
   | |      |
   | |      expected one of `extern` or `fn`
   | help: `async` must come before `unsafe`: `async unsafe`
   |
   = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs stdout ----
diff of stderr:

- error: expected one of `extern` or `fn`, found `WhereIsFerris`
+ error: expected identifier, found keyword `unsafe`
+    |
+    |
+ LL |     const unsafe WhereIsFerris Now() {}
+ 
+ 
+ error: expected one of `:`, `;`, or `=`, found `WhereIsFerris`
3    |
3    |
4 LL |     const unsafe WhereIsFerris Now() {}

-    |                  ^^^^^^^^^^^^^ expected one of `extern` or `fn`
+    |                  ^^^^^^^^^^^^^ expected one of `:`, `;`, or `=`
- error: aborting due to previous error
+ error: missing type for `const` item
+   --> $DIR/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs:5:11
+    |
+    |
+ LL |     const unsafe WhereIsFerris Now() {}
+    |           ^^^^^^ help: provide a type for the item: `r#unsafe: <type>`
+ error: aborting due to 3 previous errors
8 
9 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2/issue-68062-const-extern-fns-dont-need-fn-specifier-2.stderr
To only update this specific test, also pass `--test-args consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found keyword `unsafe`
  --> /checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs:5:11
   |
LL |     const unsafe WhereIsFerris Now() {}


error: expected one of `:`, `;`, or `=`, found `WhereIsFerris`
   |
   |
LL |     const unsafe WhereIsFerris Now() {}
   |                  ^^^^^^^^^^^^^ expected one of `:`, `;`, or `=`
error: missing type for `const` item
  --> /checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier-2.rs:5:11
   |
   |
LL |     const unsafe WhereIsFerris Now() {}
   |           ^^^^^^ help: provide a type for the item: `r#unsafe: <type>`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier.rs stdout ----
diff of stderr:

- error: expected `fn`, found `PUT_ANYTHING_YOU_WANT_HERE`
-   --> $DIR/issue-68062-const-extern-fns-dont-need-fn-specifier.rs:5:25
+ error: expected identifier, found keyword `extern`
3    |
3    |
4 LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }
-    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `fn`
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: expected one of `:`, `;`, or `=`, found `"Rust"`
+    |
+    |
+ LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }
+    |                  ^^^^^^ expected one of `:`, `;`, or `=`
+ error: missing type for `const` item
+   --> $DIR/issue-68062-const-extern-fns-dont-need-fn-specifier.rs:5:11
+    |
+    |
+ LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }
+    |           ^^^^^^ help: provide a type for the item: `r#extern: <type>`
+ error: aborting due to 3 previous errors
8 
9 

---
To only update this specific test, also pass `--test-args consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found keyword `extern`
  --> /checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier.rs:5:11
   |
LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }


error: expected one of `:`, `;`, or `=`, found `"Rust"`
   |
   |
LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }
   |                  ^^^^^^ expected one of `:`, `;`, or `=`
error: missing type for `const` item
  --> /checkout/src/test/ui/consts/const-extern-fn/issue-68062-const-extern-fns-dont-need-fn-specifier.rs:5:11
   |
   |
LL |     const extern "Rust" PUT_ANYTHING_YOU_WANT_HERE bug() -> usize { 1 }
   |           ^^^^^^ help: provide a type for the item: `r#extern: <type>`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/duplicate-visibility.rs stdout ----
diff of stderr:

- error: expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `pub`, `unsafe`, or `use`, found keyword `pub`
+ error: expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `unsafe`, or `use`, found keyword `pub`
3    |
4 LL | extern "C" {

6 LL |     pub pub fn foo();
6 LL |     pub pub fn foo();
7    |         ^^^
8    |         |
-    |         expected one of 9 possible tokens
+    |         expected one of 8 possible tokens
10    |         help: visibility `pub` must come before `pub pub`: `pub pub pub`
12 LL | }


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/duplicate-visibility/duplicate-visibility.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/duplicate-visibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/duplicate-visibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/duplicate-visibility" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/duplicate-visibility/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `unsafe`, or `use`, found keyword `pub`
   |
LL | extern "C" {
   |            - while parsing this item list starting here
LL |     pub pub fn foo();
LL |     pub pub fn foo();
   |         ^^^
   |         |
   |         expected one of 8 possible tokens
   |         help: visibility `pub` must come before `pub pub`: `pub pub pub`
LL |     //~^ ERROR expected one of `(`, `async`, `const`, `default`, `extern`, `fn`, `pub`, `unsafe`, or `use`, found keyword `pub`
LL | }
   | - the item list ends here
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/extern-abi-from-mac-literal-frag.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/extern-abi-from-mac-literal-frag.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/extern-abi-from-mac-literal-frag" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/extern-abi-from-mac-literal-frag/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `{`, found keyword `fn`
  --> /checkout/src/test/ui/parser/extern-abi-from-mac-literal-frag.rs:15:21
   |
LL |         extern $abi fn _export() {}
...
...
LL |     abi_from_lit_frag!("Rust");
   |
   |
   = note: this error originates in the macro `abi_from_lit_frag` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected `{`, found keyword `fn`
  --> /checkout/src/test/ui/parser/extern-abi-from-mac-literal-frag.rs:15:21
   |
   |
LL |         extern $abi fn _export() {}
...
...
LL |     abi_from_lit_frag!("C");
   |
   |
   = note: this error originates in the macro `abi_from_lit_frag` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected `{`, found keyword `fn`
  --> /checkout/src/test/ui/parser/extern-abi-from-mac-literal-frag.rs:27:21
   |
   |
LL |         extern $abi fn _export() {}
...
...
LL |     abi_from_expr_frag!("Rust");
   |
   |
   = note: this error originates in the macro `abi_from_expr_frag` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected `{`, found keyword `fn`
  --> /checkout/src/test/ui/parser/extern-abi-from-mac-literal-frag.rs:27:21
   |
   |
LL |         extern $abi fn _export() {}
...
...
LL |     abi_from_expr_frag!("C");
   |
   |
   = note: this error originates in the macro `abi_from_expr_frag` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-19398.rs stdout ----
diff of stderr:

- error: expected `{`, found keyword `unsafe`
+ error: expected `fn`, found keyword `unsafe`
3    |
4 LL | trait T {

5    |         - while parsing this item list starting here
5    |         - while parsing this item list starting here
6 LL |     extern "Rust" unsafe fn foo();
+    |     --------------^^^^^^
+    |     |             |
+    |     |             expected `fn`
+    |     |             expected `fn`
+    |     help: `unsafe` must come before `extern "Rust"`: `unsafe extern "Rust"`
9 LL | }
9 LL | }
10    | - the item list ends here
+    |
+    |
+    = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-19398/issue-19398.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-19398.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-19398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-19398" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-19398/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `fn`, found keyword `unsafe`
   |
LL | trait T {
   |         - while parsing this item list starting here
   |         - while parsing this item list starting here
LL |     extern "Rust" unsafe fn foo();
   |     |             |
   |     |             expected `fn`
   |     |             expected `fn`
   |     help: `unsafe` must come before `extern "Rust"`: `unsafe extern "Rust"`
LL |     //~^ ERROR expected `{`, found keyword `unsafe`
LL | }
   | - the item list ends here
   |
   = note: keyword order for functions declaration is `<visibility>`, `const`, `async`, `unsafe`, `extern`, `"<ABI>"`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-76437-pub-crate-unsafe.rs stdout ----
diff of stderr:

- error: expected one of `extern` or `fn`, found keyword `pub`
-   --> $DIR/issue-76437-pub-crate-unsafe.rs:4:12
+ error: expected item, found keyword `unsafe`
3    |
3    |
4 LL |     unsafe pub(crate) fn t() {}
-    |     -------^^^-------
-    |     |      |
-    |     |      expected one of `extern` or `fn`
-    |     help: visibility `pub(crate)` must come before `unsafe`: `pub(crate) unsafe`
9 
10 error: aborting due to previous error
11 

---
To only update this specific test, also pass `--test-args parser/issue-76437-pub-crate-unsafe.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-76437-pub-crate-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-76437-pub-crate-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-76437-pub-crate-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected item, found keyword `unsafe`
  --> /checkout/src/test/ui/parser/issue-76437-pub-crate-unsafe.rs:4:5
   |
LL |     unsafe pub(crate) fn t() {}

error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/issue-86895.rs stdout ----
diff of stderr:

- error: expected one of `async`, `extern`, `fn`, or `unsafe`, found keyword `pub`
+ error: expected identifier, found keyword `pub`
3    |
4 LL | const pub () {}


-    |       ^^^ expected one of `async`, `extern`, `fn`, or `unsafe`
+    |
+    |
+ help: you can escape reserved keywords to use them as identifiers
+    |
+ LL | const r#pub () {}
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error: expected one of `:`, `;`, or `=`, found `(`
+    |
+ LL | const pub () {}
+ LL | const pub () {}
+    |           ^ expected one of `:`, `;`, or `=`
+ error: missing type for `const` item
+   --> $DIR/issue-86895.rs:1:7
+    |
+ LL | const pub () {}
+ LL | const pub () {}
+    |       ^^^ help: provide a type for the item: `r#pub: <type>`
+ error: aborting due to 3 previous errors
8 
9 

---
To only update this specific test, also pass `--test-args parser/issue-86895.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-86895.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-86895" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-86895/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found keyword `pub`
  --> /checkout/src/test/ui/parser/issue-86895.rs:1:7
   |
LL | const pub () {}
   |       ^^^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
LL | const r#pub () {}


error: expected one of `:`, `;`, or `=`, found `(`
   |
LL | const pub () {}
LL | const pub () {}
   |           ^ expected one of `:`, `;`, or `=`
error: missing type for `const` item
  --> /checkout/src/test/ui/parser/issue-86895.rs:1:7
   |
LL | const pub () {}
LL | const pub () {}
   |       ^^^ help: provide a type for the item: `r#pub: <type>`
error: aborting due to 3 previous errors


------------------------------------------
---
test result: FAILED. 12045 passed; 9 failed; 101 ignored; 0 measured; 0 filtered out; finished in 124.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:40
