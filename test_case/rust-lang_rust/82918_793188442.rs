plain
.................................................................................................... 2400/11534
.................................................................................................... 2500/11534
.................................................................................................... 2600/11534
.......................................................i..i......................................... 2700/11534
........................F........................................................................... 2800/11534
...............................................................iiiii................................ 2900/11534
..................................................................F..F...............F.....F.F...... 3000/11534
...F....F........................................................................................... 3100/11534
.................................................................................................... 3300/11534
.................................................................................................... 3400/11534
.................................................................................................... 3500/11534
................................................................F................................... 3600/11534
---
........................................F........................................................... 8500/11534
.................................................................................................... 8600/11534
.................................................................................................... 8700/11534
................................................................iiii.iiii........................... 8800/11534
..............................i...............i......................F..........F................... 8900/11534
.................................................................................................... 9100/11534
.................................................................................................... 9200/11534
.................................................................................................... 9300/11534
.................................................................................................... 9400/11534
.................................................................................................... 9400/11534
..........................................................................i......i.................. 9500/11534
..............................F..................................................................... 9600/11534
.............iiiiiii..iiiiii.i...................................................................... 9700/11534
.................................................................................................... 9900/11534
..............................F..................................................................... 10000/11534
.................................................................................................... 10100/11534
.................................................................................................... 10200/11534
---
failures:

---- [ui] ui/anon-params/anon-params-edition-hygiene.rs stdout ----
normalized stderr:
warning: anonymous parameters are deprecated and will be removed in the next edition.
  --> $DIR/anon-params-edition-hygiene.rs:8:1
   |
LL | generate_trait_2015!(u8);
   |
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene/anon-params-edition-hygiene.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args anon-params/anon-params-edition-hygiene.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params/anon-params-edition-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL | generate_trait_2015!(u8);
   |
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
---


---- [ui] ui/associated-types/cache/chrono-scan.rs stdout ----
normalized stderr:
warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         ($e:expr) => ({ let (s_, v) = try!($e); s = s_; v })
   |                                       ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/chrono-scan/chrono-scan.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/chrono-scan.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/chrono-scan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/chrono-scan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/chrono-scan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         ($e:expr) => ({ let (s_, v) = try!($e); s = s_; v })
   |                                       ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/async-await/edition-deny-async-fns-2015.rs stdout ----
diff of stderr:

61    = help: set `edition = "2018"` in `Cargo.toml`
62    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
63 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |         async fn foo() {}
+    |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |             async fn bar() {}
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
64 error[E0670]: `async fn` is not permitted in Rust 2015
66    |


90    = note: `async` trait functions are not currently supported
91    = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
- error: aborting due to 10 previous errors
+ error: aborting due to 10 previous errors; 2 warnings emitted
94 
95 Some errors have detailed explanations: E0670, E0706.
95 Some errors have detailed explanations: E0670, E0706.
96 For more information about an error, try `rustc --explain E0670`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/edition-deny-async-fns-2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/edition-deny-async-fns-2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | fn baz() { async fn foo() {} } //~ ERROR `async fn` is not permitted in Rust 2015
   |            ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn async_baz() { //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |         async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

warning: `async` is a keyword in the 2018 edition
   |
   |
LL |         async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |             async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |         async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |             async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |             ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error: aborting due to 10 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0670, E0706.
For more information about an error, try `rustc --explain E0670`.
For more information about an error, try `rustc --explain E0670`.

------------------------------------------


---- [ui] ui/async-await/suggest-switching-edition-on-await.rs stdout ----
diff of stderr:

+ warning: `await` is a keyword in the 2018 edition
+   --> $DIR/suggest-switching-edition-on-await.rs:9:7
+    |
+ LL |     x.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `await` is a keyword in the 2018 edition
+   --> $DIR/suggest-switching-edition-on-await.rs:22:7
+    |
+ LL |     x.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `await` is a keyword in the 2018 edition
+   --> $DIR/suggest-switching-edition-on-await.rs:31:7
+    |
+ LL |     x.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `await` is a keyword in the 2018 edition
+   --> $DIR/suggest-switching-edition-on-await.rs:40:7
+    |
+ LL |     x.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
1 error[E0609]: no field `await` on type `await_on_struct_missing::S`
2   --> $DIR/suggest-switching-edition-on-await.rs:9:7


38    = help: set `edition = "2018"` in `Cargo.toml`
39    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
- error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors; 4 warnings emitted
42 
43 For more information about this error, try `rustc --explain E0609`.
43 For more information about this error, try `rustc --explain E0609`.
44 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/suggest-switching-edition-on-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-switching-edition-on-await.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-switching-edition-on-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `await` is a keyword in the 2018 edition
   |
LL |     x.await;
LL |     x.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `await` is a keyword in the 2018 edition
   |
LL |     x.await;
LL |     x.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `await` is a keyword in the 2018 edition
   |
LL |     x.await;
LL |     x.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `await` is a keyword in the 2018 edition
   |
LL |     x.await;
LL |     x.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


error[E0609]: no field `await` on type `await_on_struct_missing::S`
   |
LL |     x.await;
   |       ^^^^^ unknown field
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `await_on_struct_similar::S`
   |
LL |     x.await;
LL |     x.await;
   |       ^^^^^ help: a field with a similar name exists: `awai`
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `Pin<&mut dyn Future<Output = ()>>`
   |
LL |     x.await;
   |       ^^^^^ unknown field
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `impl Future<Output = ()>`
   |
LL |     x.await;
   |       ^^^^^
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: aborting due to 4 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0609`.


------------------------------------------


---- [ui] ui/derived-errors/issue-31997.rs stdout ----
diff of stderr:

+ warning: `try` is a keyword in the 2018 edition
+    |
+    |
+ LL |     try!(closure(|| bar(core::ptr::null_mut())));
+    |     ^^^ help: you can use a raw identifier to stay compatible: `r#try`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
1 error[E0425]: cannot find function `bar` in this scope
3    |


4 LL |     try!(closure(|| bar(core::ptr::null_mut())));
6 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
8 
8 
9 For more information about this error, try `rustc --explain E0425`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997/issue-31997.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derived-errors/issue-31997.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derived-errors/issue-31997.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-31997/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
  --> /checkout/src/test/ui/derived-errors/issue-31997.rs:14:5
   |
LL |     try!(closure(|| bar(core::ptr::null_mut()))); //~ ERROR cannot find function `bar` in this scope
   |     ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


error[E0425]: cannot find function `bar` in this scope
  --> /checkout/src/test/ui/derived-errors/issue-31997.rs:14:21
   |
LL |     try!(closure(|| bar(core::ptr::null_mut()))); //~ ERROR cannot find function `bar` in this scope

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.

------------------------------------------


---- [ui] ui/dyn-trait-compatibility.rs stdout ----
diff of stderr:

+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A0 = dyn;
+    |           ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A1 = dyn::dyn;
+    |           ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A1 = dyn::dyn;
+    |                ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A2 = dyn<dyn, dyn>;
+    |           ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A2 = dyn<dyn, dyn>;
+    |               ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A2 = dyn<dyn, dyn>;
+    |                    ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `dyn` is a keyword in the 2018 edition
+    |
+    |
+ LL | type A3 = dyn<<dyn as dyn>::dyn>;
---

---- [ui] ui/editions/async-block-2015.rs stdout ----
diff of stderr:

29    = help: set `edition = "2018"` in `Cargo.toml`
30    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
31 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     let x = async {};
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `await` is a keyword in the 2018 edition
+    |
+    |
+ LL |     y.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `await` is a keyword in the 2018 edition
+    |
+    |
+ LL |     z.await;
+    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
32 error[E0422]: cannot find struct, variant or union type `async` in this scope
34    |


35 LL |     let x = async {};
36    |             ^^^^^ `async` blocks are only allowed in Rust 2018 or later
- error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors; 3 warnings emitted
39 
40 Some errors have detailed explanations: E0422, E0670.
40 Some errors have detailed explanations: E0422, E0670.
41 For more information about an error, try `rustc --explain E0422`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/async-block-2015/async-block-2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args editions/async-block-2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/async-block-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/async-block-2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/async-block-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn foo() {
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: expected identifier, found keyword `let`
  --> /checkout/src/test/ui/editions/async-block-2015.rs:11:9
   |
   |
LL |     let y = async { //~ NOTE `async` blocks are only allowed in Rust 2018 or later
   |             ----- `async` blocks are only allowed in Rust 2018 or later
LL |         let x = 42;
   |
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: expected identifier, found `42`
  --> /checkout/src/test/ui/editions/async-block-2015.rs:19:9
   |
   |
LL |     let z = async { //~ NOTE `async` blocks are only allowed in Rust 2018 or later
   |             ----- `async` blocks are only allowed in Rust 2018 or later
   |         ^^ expected identifier
   |
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let x = async {};
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `await` is a keyword in the 2018 edition
   |
   |
LL |     y.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `await` is a keyword in the 2018 edition
   |
   |
LL |     z.await;
   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


error[E0422]: cannot find struct, variant or union type `async` in this scope
   |
   |
LL |     let x = async {};
   |             ^^^^^ `async` blocks are only allowed in Rust 2018 or later
error: aborting due to 4 previous errors; 3 warnings emitted

Some errors have detailed explanations: E0422, E0670.
For more information about an error, try `rustc --explain E0422`.
For more information about an error, try `rustc --explain E0422`.

------------------------------------------


---- [ui] ui/edition-keywords-2015-2015.rs stdout ----
normalized stderr:
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
LL |     one_async::async(); // OK
LL |     one_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     two_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 5 warnings emitted
warning: 5 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2015/edition-keywords-2015-2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args edition-keywords-2015-2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2015-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2015/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
LL |     one_async::async(); // OK
LL |     one_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     two_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 5 warnings emitted
warning: 5 warnings emitted


------------------------------------------


---- [ui] ui/edition-keywords-2015-2018.rs stdout ----
normalized stderr:
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     two_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 4 warnings emitted
warning: 4 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018/edition-keywords-2015-2018.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args edition-keywords-2015-2018.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2015-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     two_async::async(); // OK
   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 4 warnings emitted
---

---- [ui] ui/editions/edition-keywords-2015-2015-parsing.rs stdout ----
diff of stderr:

+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     pub fn async() {}
+    |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     let mut async = 1; // OK
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     r#async = consumes_async!(async); // OK
+    |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     r#async = consumes_async_raw!(async);
+    |                                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     if passes_ident!(async) == 1 {} // OK
+    |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+ LL |     module::async(); // OK
+ LL |     module::async(); // OK
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
1 error: no rules expected the token `r#async`
3    |


10 LL |     r#async = consumes_async_raw!(async);
11    |                                   ^^^^^ no rules expected this token in macro call
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 6 warnings emitted
14 
15 
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2015-parsing/edition-keywords-2015-2015-parsing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args editions/edition-keywords-2015-2015-parsing.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2015-2015-parsing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2015-parsing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2015-parsing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     pub fn async() {}
   |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
   |                                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
LL |     module::async(); // OK
LL |     module::async(); // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


error: no rules expected the token `r#async`
   |
   |
LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
   |                               ^^^^^^^ no rules expected this token in macro call

error: no rules expected the token `async`
   |
   |
LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
   |                                   ^^^^^ no rules expected this token in macro call
error: aborting due to 2 previous errors; 6 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/editions/edition-keywords-2015-2018-parsing.rs stdout ----
diff of stderr:

+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     pub fn async() {}
+    |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     let mut async = 1; // OK
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     r#async = consumes_async!(async); // OK
+    |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     r#async = consumes_async_raw!(async);
+    |                                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+    |
+ LL |     if passes_ident!(async) == 1 {} // OK
+    |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
+ warning: `async` is a keyword in the 2018 edition
+    |
+ LL |     module::async(); // OK
+ LL |     module::async(); // OK
+    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+ 
+ 
1 error: no rules expected the token `r#async`
3    |


10 LL |     r#async = consumes_async_raw!(async);
11    |                                   ^^^^^ no rules expected this token in macro call
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 6 warnings emitted
14 
15 
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-parsing/edition-keywords-2015-2018-parsing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args editions/edition-keywords-2015-2018-parsing.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2015-2018-parsing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-parsing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-parsing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     pub fn async() {}
   |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     let mut async = 1; // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async!(async); // OK
   |                               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
   |                                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
   |
LL |     if passes_ident!(async) == 1 {} // OK
   |                      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `async` is a keyword in the 2018 edition
   |
LL |     module::async(); // OK
LL |     module::async(); // OK
   |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


error: no rules expected the token `r#async`
   |
   |
LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
   |                               ^^^^^^^ no rules expected this token in macro call

error: no rules expected the token `async`
   |
   |
LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
   |                                   ^^^^^ no rules expected this token in macro call
error: aborting due to 2 previous errors; 6 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/feature-gates/feature-gate-object_safe_for_dispatch.rs stdout ----
diff of stderr:

+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+   --> $DIR/feature-gate-object_safe_for_dispatch.rs:15:19
+    |
+ LL |     fn foo(&self, &Self);
+    |                   ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: &Self`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
1 error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
2   --> $DIR/feature-gate-object_safe_for_dispatch.rs:18:38

78    |       |
78    |       |
79    |       this trait cannot be made into an object...
- error: aborting due to 5 previous errors
+ error: aborting due to 5 previous errors; 1 warning emitted
82 
83 For more information about this error, try `rustc --explain E0038`.
83 For more information about this error, try `rustc --explain E0038`.
84 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/feature-gate-object_safe_for_dispatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-object_safe_for_dispatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:15:19
   |
LL |     fn foo(&self, &Self);
   |                   ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: &Self`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:18:38
   |
LL | fn takes_non_object_safe_ref<T>(obj: &dyn NonObjectSafe1) {
   |                                      ^^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...

error[E0038]: the trait `NonObjectSafe2` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:22:36
   |
LL | fn return_non_object_safe_ref() -> &'static dyn NonObjectSafe2 {
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe2` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:7:8
   |
LL | trait NonObjectSafe2 {
   |       -------------- this trait cannot be made into an object...
LL |     fn static_fn() {}
   |        ^^^^^^^^^ ...because associated function `static_fn` has no `self` parameter
help: consider turning `static_fn` into a method by giving it a `&self` argument
   |
LL |     fn static_fn(&self) {}
   |                  ^^^^^
help: alternatively, consider constraining `static_fn` so it does not apply to trait objects
   |
LL |     fn static_fn() where Self: Sized {}


error[E0038]: the trait `NonObjectSafe3` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:27:35
   |
LL | fn takes_non_object_safe_box(obj: Box<dyn NonObjectSafe3>) {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe3` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:11:8
   |
LL | trait NonObjectSafe3 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo<T>(&self);
   |        ^^^ ...because method `foo` has generic type parameters

error[E0038]: the trait `NonObjectSafe4` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:31:35
   |
LL | fn return_non_object_safe_rc() -> std::rc::Rc<dyn NonObjectSafe4> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe4` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:15:19
   |
LL | trait NonObjectSafe4 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo(&self, &Self);
   |                   ^^^^^ ...because method `foo` references the `Self` type in this parameter

error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:38:16
   |
LL | impl Trait for dyn NonObjectSafe1 {}
   |                ^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...
error: aborting due to 5 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0038`.


------------------------------------------


---- [ui] ui/issues/issue-65611.rs stdout ----
diff of stderr:

+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn from(usize) -> Self;
+    |             ^^^^^ help: try naming the parameter or explicitly ignoring it: `_: usize`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
1 error[E0282]: type annotations needed
2   --> $DIR/issue-65611.rs:59:20
3    |

15 LL |     let x = buffer.last().unwrap().0.clone();
17 
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
19 
19 
20 Some errors have detailed explanations: E0282, E0609.
21 For more information about an error, try `rustc --explain E0282`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/issue-65611.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-65611.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn from(usize) -> Self;
---

---- [ui] ui/issues/issue-78720.rs stdout ----
diff of stderr:

24 LL | struct Map2<Segment2, F> {
26 
26 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn map2<F>(self, F) -> Map2<F> {}
+    |                      ^ help: try naming the parameter or explicitly ignoring it: `_: F`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
27 error[E0308]: mismatched types
28   --> $DIR/issue-78720.rs:7:36
29    |

49 LL |     fn map2<F>(&self, F) -> Map2<F> {}
51 
- error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors; 1 warning emitted
53 
53 
54 Some errors have detailed explanations: E0277, E0308, E0412.
55 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/issue-78720.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-78720.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-78720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/issues/issue-78720.rs:1:16
   |
LL | fn server() -> impl {

error[E0412]: cannot find type `F` in this scope
  --> /checkout/src/test/ui/issues/issue-78720.rs:13:12
   |
   |
LL |     _func: F,
   | 
  ::: /checkout/library/core/src/ops/function.rs:67:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here
help: a trait with a similar name exists
   |
   |
LL |     _func: Fn,
help: you might be missing a type parameter
   |
   |
LL | struct Map2<Segment2, F> {


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                      ^ help: try naming the parameter or explicitly ignoring it: `_: F`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-78720.rs:7:36
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                                    ^^ expected struct `Map2`, found `()`
   |
   = note: expected struct `Map2<F>`


error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                ^^^^ doesn't have a size known at compile-time
   |
   = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
   |
LL |     fn map2<F>(self, F) -> Map2<F> where Self: Sized {}
   |                                    ^^^^^^^^^^^^^^^^^
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL |     fn map2<F>(&self, F) -> Map2<F> {}

error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0277, E0308, E0412.
---

---- [ui] ui/lint/lint-qualification.rs stdout ----
diff of stderr:

+ warning: `try` is a keyword in the 2018 edition
+    |
+    |
+ LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
+    |                                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
1 error: unnecessary qualification
2   --> $DIR/lint-qualification.rs:10:5
3    |
3    |

10 LL | #![deny(unused_qualifications)]
12 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
14 
14 
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification/lint-qualification.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-qualification.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-qualification.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
   |                                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

error: unnecessary qualification
  --> /checkout/src/test/ui/lint/lint-qualification.rs:10:5
   |
   |
LL |     foo::bar(); //~ ERROR: unnecessary qualification
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-qualification.rs:1:9
   |
   |
LL | #![deny(unused_qualifications)]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/macros/macro-comma-support-rpass.rs#core stdout ----
normalized stderr:
warning: `try` is a keyword in the 2018 edition
   |
   |
LL | fn try() {
   |    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()));
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()),);
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 3 warnings emitted
warning: 3 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/macro-comma-support-rpass.core.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-comma-support-rpass.rs`

error in revision `core`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
   |
LL | fn try() {
   |    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()));
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()),);
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 3 warnings emitted
warning: 3 warnings emitted


------------------------------------------


---- [ui] ui/macros/macro-comma-support-rpass.rs#std stdout ----
normalized stderr:
warning: `try` is a keyword in the 2018 edition
   |
   |
LL | fn try() {
   |    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()));
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()),);
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 3 warnings emitted
warning: 3 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/macro-comma-support-rpass.std.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-comma-support-rpass.rs`

error in revision `std`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
   |
LL | fn try() {
   |    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()));
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |         try!(Ok(()),);
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 3 warnings emitted
warning: 3 warnings emitted


------------------------------------------


---- [ui] ui/macros/try-macro.rs stdout ----
normalized stderr:
warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:14:8
   |
LL |     Ok(try!("1".parse()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:18:13
   |
LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
   |             ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:18:8
   |
LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:22:8
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:22:42
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:26:8
   |
LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:26:42
   |
LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:30:8
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-macro.rs:30:42
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 9 warnings emitted
warning: 9 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/try-macro/try-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/try-macro.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/try-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/try-macro/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/try-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("1".parse()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
   |             ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
   |        ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
   |                                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 9 warnings emitted
warning: 9 warnings emitted


------------------------------------------


---- [ui] ui/parser/extern-crate-async.rs stdout ----
normalized stderr:
warning: `async` is a keyword in the 2018 edition
   |
LL | extern crate async;
LL | extern crate async;
   |              ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/extern-crate-async/extern-crate-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/extern-crate-async.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/extern-crate-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/extern-crate-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/extern-crate-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `async` is a keyword in the 2018 edition
   |
LL | extern crate async;
LL | extern crate async;
   |              ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/parser/variadic-ffi-semantic-restrictions.rs stdout ----
diff of stderr:

202 LL |     fn t_f6(..., x: isize);
204 
- error: aborting due to 34 previous errors
- error: aborting due to 34 previous errors
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f1(x: isize, ...) {}
+    |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn t_f2(x: isize, ...);
+    |                       ^^^ help: try naming the parameter or explicitly ignoring it: `_: ...`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
---

---- [ui] ui/proc-macro/raw-ident.rs stdout ----
diff of stderr:

+ warning: `await` is a keyword in the 2018 edition
+    |
+    |
+ LL |     make_struct!(await);
+    |                  ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
+ 
1 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `S`
3    |

6    |
7    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/raw-ident/raw-ident.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/raw-ident.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/raw-ident.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/raw-ident" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/raw-ident/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `await` is a keyword in the 2018 edition
   |
   |
LL |     make_struct!(await);
   |                  ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `S`
   |
   |
LL |     make_bad_struct!(S); //~ ERROR expected one of
   |     ^^^^^^^^^^^^^^^^^^^^ expected one of 8 possible tokens
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/proc-macro/trait-fn-args-2015.rs stdout ----
normalized stderr:
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
LL |     fn method(u8);
LL |     fn method(u8);
   |               ^^ help: try naming the parameter or explicitly ignoring it: `_: u8`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015/trait-fn-args-2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/trait-fn-args-2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/trait-fn-args-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/trait-fn-args-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
LL |     fn method(u8);
LL |     fn method(u8);
   |               ^^ help: try naming the parameter or explicitly ignoring it: `_: u8`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

---

---- [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs stdout ----
diff of stderr:

172 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
173    |                                                        ^^ not a non-macro attribute
- error: aborting due to 29 previous errors
- error: aborting due to 29 previous errors
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
+    |                                                                ^^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: Vec<u8>`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ error: aborting due to 29 previous errors; 1 warning emitted
176 
177 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2565-param-attrs/proc-macro-cannot-be-used.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                       ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | fn free(#[id] arg1: u8) {
   |           ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                  ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                                ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                                               ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                             ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                                         ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                                        ^^ not a non-macro attribute

warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                                                ^^^^^^^ help: try naming the parameter or explicitly ignoring it: `_: Vec<u8>`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

---
---- [ui] ui/specialization/issue-39448.rs stdout ----
diff of stderr:

8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+    |
+    |
+ LL |     fn from(T) -> Self;
+    |             ^ help: try naming the parameter or explicitly ignoring it: `_: T`
+    = note: `#[warn(anonymous_parameters)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
11 error[E0275]: overflow evaluating the requirement `T: FromA<U>`
13    |


17    = note: required because of the requirements on the impl of `FromA<U>` for `T`
18    = note: required because of the requirements on the impl of `ToA<T>` for `U`
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
21 
22 For more information about this error, try `rustc --explain E0275`.
22 For more information about this error, try `rustc --explain E0275`.
23 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/issue-39448.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-39448.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-39448.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn from(T) -> Self;
   |             ^ help: try naming the parameter or explicitly ignoring it: `_: T`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error[E0275]: overflow evaluating the requirement `T: FromA<U>`
   |
   |
LL |     x.foo(y.to()).to() //~ ERROR overflow evaluating the requirement
   |
   |
   = note: required because of the requirements on the impl of `FromA<U>` for `T`
   = note: required because of the requirements on the impl of `ToA<T>` for `U`
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0275`.

---

7 LL |         let x = 5;
8    |         ^^^ expected identifier, found keyword
9 
+ warning: `try` is a keyword in the 2018 edition
+   --> $DIR/try-block-in-edition2015.rs:4:33
+    |
+ LL |     let try_result: Option<_> = try {
+    |                                 ^^^ help: you can use a raw identifier to stay compatible: `r#try`
+    |
+    = note: `#[warn(keyword_idents)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+ 
10 error[E0574]: expected struct, variant or union type, found macro `try`
11   --> $DIR/try-block-in-edition2015.rs:4:33
12    |
12    |

19 LL |     let try_result: Option<_> = try! {
21 
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
23 
23 
24 For more information about this error, try `rustc --explain E0574`.
25 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-in-edition2015/try-block-in-edition2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-in-edition2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-in-edition2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-in-edition2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-in-edition2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found keyword `let`
  --> /checkout/src/test/ui/try-block/try-block-in-edition2015.rs:6:9
   |
LL |     let try_result: Option<_> = try {
   |                                 --- while parsing this struct
LL |     //~^ ERROR expected struct, variant or union type, found macro `try`
LL |         let x = 5; //~ ERROR expected identifier, found keyword


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try_result: Option<_> = try {
   |                                 ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!

error[E0574]: expected struct, variant or union type, found macro `try`
  --> /checkout/src/test/ui/try-block/try-block-in-edition2015.rs:4:33
   |
   |
LL |     let try_result: Option<_> = try {
   |                                 ^^^ not a struct, variant or union type
   |
   = note: if you want the `try` keyword, you need Rust 2018 or later
help: use `!` to invoke the macro
   |
LL |     let try_result: Option<_> = try! {

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0574`.
For more information about this error, try `rustc --explain E0574`.

------------------------------------------


---- [ui] ui/try-is-identifier-edition2015.rs stdout ----
normalized stderr:
warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:7:9
LL |     let try = 2;
LL |     let try = 2;
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:8:12
   |
LL |     struct try { try: u32 }
   |            ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:8:18
   |
LL |     struct try { try: u32 }
   |                  ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:9:9
   |
LL |     let try: try = try { try };
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:9:14
   |
LL |     let try: try = try { try };
   |              ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:9:20
   |
LL |     let try: try = try { try };
   |                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:9:26
   |
LL |     let try: try = try { try };
   |                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:9:26
   |
LL |     let try: try = try { try };
   |                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:10:16
   |
LL |     assert_eq!(try.try, 2);
   |                ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
  --> $DIR/try-is-identifier-edition2015.rs:10:20
   |
LL |     assert_eq!(try.try, 2);
   |                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 10 warnings emitted
warning: 10 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-is-identifier-edition2015/try-is-identifier-edition2015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-is-identifier-edition2015.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-is-identifier-edition2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-is-identifier-edition2015/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-is-identifier-edition2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `try` is a keyword in the 2018 edition
   |
LL |     let try = 2;
LL |     let try = 2;
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   |
   = note: `#[warn(keyword_idents)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     struct try { try: u32 }
   |            ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     struct try { try: u32 }
   |                  ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try: try = try { try };
   |         ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try: try = try { try };
   |              ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try: try = try { try };
   |                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try: try = try { try };
   |                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     let try: try = try { try };
   |                          ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     assert_eq!(try.try, 2);
   |                ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>


warning: `try` is a keyword in the 2018 edition
   |
   |
LL |     assert_eq!(try.try, 2);
   |                    ^^^ help: you can use a raw identifier to stay compatible: `r#try`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: 10 warnings emitted
---
test result: FAILED. 11414 passed; 27 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.83s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:46
