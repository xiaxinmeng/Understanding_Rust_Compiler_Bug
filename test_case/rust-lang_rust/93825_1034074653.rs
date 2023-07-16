plain
Successfully built ab88825a892f
Successfully tagged rust-ci:latest
Built container sha256:ab88825a892fa50f26d68fe12db01d5fc6e3a43c6af3f9a6616d347d7e8e5954
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---

---- [ui] ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint.rs stdout ----
diff of stderr:

- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:14:13
+ error: expected identifier, found `(`
3    |
3    |
- LL |     pub mod dyn {
-    |             ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
- note: the lint level is defined here
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:11:9
-    |
-    |
- LL | #![deny(keyword_idents)]
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:17:20
- LL |         pub struct dyn;
- LL |         pub struct dyn;
-    |                    ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:22:16
-    |
- LL | use outer_mod::dyn::dyn;
-    |                ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:22:21
-    |
- LL | use outer_mod::dyn::dyn;
-    |                     ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:29:11
-    |
- LL |     match dyn { dyn => {} }
-    |           ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:29:17
-    |
- LL |     match dyn { dyn => {} }
-    |                 ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:34:17
-    |
- LL |     macro_defn::dyn();
-    |                 ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:44:18
- LL |     macro_rules! dyn {
- LL |     macro_rules! dyn {
-    |                  ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:52:12
-    |
- LL |     pub fn dyn() -> ::outer_mod::dyn::dyn {
-    |            ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:52:34
-    |
- LL |     pub fn dyn() -> ::outer_mod::dyn::dyn {
-    |                                  ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:52:39
-    |
- LL |     pub fn dyn() -> ::outer_mod::dyn::dyn {
-    |                                       ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:59:22
-    |
- LL |         ::outer_mod::dyn::dyn
-    |                      ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:59:27
-    |
- LL |         ::outer_mod::dyn::dyn
-    |                           ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
- 
- 
- error: `dyn` is a keyword in the 2018 edition
-   --> $DIR/dyn-2015-edition-keyword-ident-lint.rs:68:23
-    |
126 LL |     pub fn boxed() -> dyn!(
-    |                       ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
-    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
-    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
+    |                           ^ expected identifier
131 
---
diff of fixed:

11 #![deny(keyword_idents)]
12 
13 mod outer_mod {
-     pub mod r#dyn {
+     pub mod dyn {
15 //~^ ERROR `dyn` is a keyword
16 //~| WARN this is accepted in the current edition
-         pub struct r#dyn;
+         pub struct dyn;
18 //~^ ERROR `dyn` is a keyword
19 //~| WARN this is accepted in the current edition

21 }
21 }
- use outer_mod::r#dyn::r#dyn;
+ use outer_mod::dyn::dyn;
23 //~^ ERROR `dyn` is a keyword
24 //~| WARN this is accepted in the current edition
25 //~| ERROR `dyn` is a keyword

26 //~| WARN this is accepted in the current edition
28 fn main() {
28 fn main() {
-     match r#dyn { r#dyn => {} }
+     match dyn { dyn => {} }
30 //~^ ERROR `dyn` is a keyword
31 //~| WARN this is accepted in the current edition
32 //~| ERROR `dyn` is a keyword

33 //~| WARN this is accepted in the current edition
-     macro_defn::r#dyn();
+     macro_defn::dyn();
35 //~^ ERROR `dyn` is a keyword
36 //~| WARN this is accepted in the current edition

41 mod macro_defn {
42     use super::Trait;
43 
43 
-     macro_rules! r#dyn {
+     macro_rules! dyn {
45 //~^ ERROR `dyn` is a keyword
46 //~| WARN this is accepted in the current edition


49         ($dyn:tt) => { (Box<dyn Trait>, Box<$dyn Trait>) }
51 
51 
-     pub fn r#dyn() -> ::outer_mod::r#dyn::r#dyn {
+     pub fn dyn() -> ::outer_mod::dyn::dyn {
53 //~^ ERROR `dyn` is a keyword
54 //~| WARN this is accepted in the current edition
55 //~| ERROR `dyn` is a keyword

56 //~| WARN this is accepted in the current edition
57 //~| ERROR `dyn` is a keyword
58 //~| WARN this is accepted in the current edition
-         ::outer_mod::r#dyn::r#dyn
+         ::outer_mod::dyn::dyn
60 //~^ ERROR `dyn` is a keyword
61 //~| WARN this is accepted in the current edition
62 //~| ERROR `dyn` is a keyword
65 
66 
67 
67 
-     pub fn boxed() -> r#dyn!(
+     pub fn boxed() -> dyn!(
69         //~^ ERROR `dyn` is a keyword
70         //~| WARN this is accepted in the current edition


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint/dyn-2015-edition-keyword-ident-lint.fixed
To only update this specific test, also pass `--test-args dyn-keyword/dyn-2015-edition-keyword-ident-lint.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found `(`
  --> /checkout/src/test/ui/dyn-keyword/dyn-2015-edition-keyword-ident-lint.rs:68:27
   |
LL |     pub fn boxed() -> dyn!(
   |                           ^ expected identifier
error: aborting due to previous error


------------------------------------------
