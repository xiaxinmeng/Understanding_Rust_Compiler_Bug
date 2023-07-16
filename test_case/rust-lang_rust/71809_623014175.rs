plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b45641a2-19c8-49d8-ae26-3129b8fa9c7a.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71809/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71809/merge:refs/remotes/pull/71809/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................i.................................................................................. 1800/9966
.................................................................................................... 1900/9966
.................................i.................................................................. 2000/9966
.................................................................................................... 2100/9966
.......................iiiii........................................................................ 2200/9966
.................................................................................................... 2400/9966
.................................................................................................... 2500/9966
.................................................................................................... 2600/9966
.................................................................................................... 2700/9966
---
.......i...............i............................................................................ 5100/9966
.................................................................................................... 5200/9966
.....................................................i.............................................. 5300/9966
............................................i....................................................... 5400/9966
..............................................ii.ii........i...i.................................... 5500/9966
.............................................................................................i...... 5700/9966
.................................................................................................... 5800/9966
............................ii.....................................i................................ 5900/9966
.................................................................................................... 6000/9966
.................................................................................................... 6000/9966
.................................................................................................... 6100/9966
..............................................................ii...i..ii...........i................ 6200/9966
.................................................................................................... 6400/9966
.................................................................................................... 6500/9966
.................................................................................................... 6500/9966
..............................................................................................i..ii. 6600/9966
.................................................................................................... 6800/9966
...............................................................................................i.... 6900/9966
.................................................................................................... 7000/9966
.................................................................................................... 7100/9966
---
.................................................................................................... 7900/9966
.................................................................................................... 8000/9966
.................................................................................................... 8100/9966
.....i.............................................................................................. 8200/9966
......................................................iiiiii.iiiii.i................................ 8300/9966
.......i............................................................................................ 8500/9966
.................................................................................................... 8600/9966
.................................................................................................... 8700/9966
.................................................................................................... 8800/9966
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
 finished in 6.069
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 2.240
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.169
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 16.383
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2500 tests
......iiiii......................................................................................... 100/2500
......................................................................................ii............ 200/2500
......................i............................................................................. 400/2500
............................................................................i..i..................ii 500/2500
ii.................................................................................................. 600/2500
.................................................................................................... 700/2500
---

running 1020 tests
i................................................................................................... 100/1020
.................................................................................................... 200/1020
....................iii.....i......i...i......i..................................................... 300/1020
.................................................................................................... 400/1020
....................................................i....i......................................ii.. 500/1020
.................................................................................................... 700/1020
.................................................................................................... 700/1020
..............................................iiii.................................................. 800/1020
.................................................................................................... 900/1020
....................................................................iiii............................ 1000/1020
test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 167.729
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 1.130
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-17c36066b7f316a1

running 0 tests

---
Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 211 tests
......................i...ii.......................................................................i 100/211
........................................iiiiii......i..............iiiF............................. 200/211
.......ii..
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [run-make] run-make-fulldeps/save-analysis stdout ----
---- [run-make] run-make-fulldeps/save-analysis stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  krate2.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  foo.rs -Zsave-analysis
Makefile:6: recipe for target 'code' failed
------------------------------------------
stderr:
------------------------------------------
warning: unused `std::result::Result` that must be used
warning: unused `std::result::Result` that must be used
 --> krate2.rs:7:5
  |
7 |     std::io::stdout().write_all(b"hello world!\n");
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this `Result` may be an `Err` variant, which should be handled


warning: 1 warning emitted

warning: unused import: `graphviz::RenderOption`
   |
   |
13 | use graphviz::RenderOption;
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `HashSet`
warning: unused import: `HashSet`
  --> foo.rs:14:32
   |
14 | use std::collections::{HashMap,HashSet};

warning: unused import: `std::mem::size_of`
  --> foo.rs:23:5
   |
---

warning: type `nested_enum` should have an upper camel case name
  --> foo.rs:83:18
   |
83 |         pub enum nested_enum {
   |                  ^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedEnum`

warning: unused import: `sub::sub2 as msalias`
 --> SubDir/mod.rs:3:5
  |
3 | use sub::sub2 as msalias;

warning: unused import: `sub::sub2`
 --> SubDir/mod.rs:4:5
  |
  |
4 | use sub::sub2;

warning: type `nested_struct` should have an upper camel case name
  --> SubDir/mod.rs:19:20
   |
   |
19 |         pub struct nested_struct {
   |                    ^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedStruct`

warning: type `nofields` should have an upper camel case name
   |
96 | struct nofields;
96 | struct nofields;
   |        ^^^^^^^^ help: convert the identifier to upper camel case: `Nofields`
warning: type `some_fields` should have an upper camel case name
  --> foo.rs:99:8
   |
99 | struct some_fields {
99 | struct some_fields {
   |        ^^^^^^^^^^^ help: convert the identifier to upper camel case: `SomeFields`

warning: trait objects without an explicit `dyn` are deprecated
   --> foo.rs:251:16
    |
251 |     let s: Box<SomeTrait> = box some_fields {field1: 43};

warning: trait objects without an explicit `dyn` are deprecated
   --> foo.rs:267:24
    |
    |
267 |     let s4 = s3 as Box<SomeTrait>;
    |                        ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`

warning: trait objects without an explicit `dyn` are deprecated
   --> foo.rs:273:32
    |
273 |     let closure = |x: u32, s: &SomeTrait| {


warning: type `blah` should have an upper camel case name
    |
281 | pub struct blah {
    |            ^^^^ help: convert the identifier to upper camel case: `Blah`

---

warning: trait objects without an explicit `dyn` are deprecated
   --> foo.rs:418:10
    |
418 |         <Error + 'static>::is::<T>(self)

warning: unused variable: `s`
  --> foo.rs:35:9
   |
   |
35 |     let s = sub_struct{ field2: 45u32, };
   |         ^ help: if this is intentional, prefix it with an underscore: `_s`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> foo.rs:41:9
---

warning: unused variable: `i`
  --> foo.rs:34:28
   |
34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {
   |                            ^ help: if this is intentional, prefix it with an underscore: `_i`
warning: unused variable: `x`
  --> foo.rs:38:12
   |
38 |     fn foo(x: &Write) {}
38 |     fn foo(x: &Write) {}
   |            ^ help: if this is intentional, prefix it with an underscore: `_x`

warning: unused variable: `f1`
   --> foo.rs:212:40
    |
212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
    |                                        ^^ help: if this is intentional, prefix it with an underscore: `_f1`
warning: unused variable: `f2`
   --> foo.rs:222:29
    |
    |
222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
    |                             ^^ help: if this is intentional, prefix it with an underscore: `_f2`
warning: unused variable: `f2`
   --> foo.rs:223:49
    |
    |
223 |         EnumStruct3{f1, f3: SomeEnum::Ints(..), f2} => println(&f1.field1.to_string()),
    |                                                 ^^ help: try ignoring the field: `f2: _`
warning: unused variable: `y`
   --> foo.rs:240:13
    |
    |
240 |     let (x, y): (u32, u32) = (5, 3);
    |             ^ help: if this is intentional, prefix it with an underscore: `_y`
warning: unused variable: `r`
   --> foo.rs:263:9
    |
    |
263 |     let r = some_fields::stat(y);
    |         ^ help: if this is intentional, prefix it with an underscore: `_r`
warning: unused variable: `r`
   --> foo.rs:265:9
    |
    |
265 |     let r = SubTrait::stat2(&*s3);
    |         ^ help: if this is intentional, prefix it with an underscore: `_r`
warning: unused variable: `z`
   --> foo.rs:278:9
    |
    |
278 |     let z = closure(10, &*s);
    |         ^ help: if this is intentional, prefix it with an underscore: `_z`
warning: unused variable: `a`
   --> foo.rs:236:28
    |
    |
236 | fn hello<X: SomeTrait>((z, a) : (u32, String), ex: X) {
    |                            ^ help: if this is intentional, prefix it with an underscore: `_a`
warning: unused variable: `ut`
   --> foo.rs:315:9
    |
    |
315 |     let ut = "Les Miséééééééérables";
    |         ^^ help: if this is intentional, prefix it with an underscore: `_ut`
warning: unused variable: `vs`
   --> foo.rs:324:9
    |
    |
324 |     let vs = variable_str!(32);
    |         ^^ help: if this is intentional, prefix it with an underscore: `_vs`
warning: unused variable: `candidates`
   --> foo.rs:326:9
    |
    |
326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
    |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_candidates`
warning: unused variable: `s1`
   --> foo.rs:330:9
    |
330 |     let s1 = nofields;
330 |     let s1 = nofields;
    |         ^^ help: if this is intentional, prefix it with an underscore: `_s1`

warning: unused variable: `s3`
   --> foo.rs:332:9
    |
332 |     let s3: some_fields = some_fields{ field1: 55};
    |         ^^ help: if this is intentional, prefix it with an underscore: `_s3`
warning: unused variable: `s4`
   --> foo.rs:333:9
    |
    |
333 |     let s4: msalias::nested_struct = sub::sub2::nested_struct{ field2: 55};
    |         ^^ help: if this is intentional, prefix it with an underscore: `_s4`
warning: unused variable: `s4`
   --> foo.rs:334:9
    |
    |
334 |     let s4: msalias::nested_struct = sub2::nested_struct{ field2: 55};
    |         ^^ help: if this is intentional, prefix it with an underscore: `_s4`
warning: unused variable: `s`
   --> foo.rs:337:9
    |
    |
337 |     let s = SameDir::SameStruct{name: "Bob".to_string()};
    |         ^ help: if this is intentional, prefix it with an underscore: `_s`
warning: unused variable: `s`
   --> foo.rs:338:9
    |
    |
338 |     let s = SubDir::SubStruct{name:"Bob".to_string()};
    |         ^ help: if this is intentional, prefix it with an underscore: `_s`
warning: variable `x` is assigned to, but never used
   --> foo.rs:299:17
    |
    |
299 |             let mut x = $src;
...
...
378 |     internal_vars!(x);
    |
    = note: consider using `_x` instead
    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


warning: value assigned to `x` is never read
   --> foo.rs:300:13
    |
300 |             x += 100;
    |             ^
...
378 |     internal_vars!(x);
    |
    = note: `#[warn(unused_assignments)]` on by default
    = help: maybe it is overwritten before being read?
    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---

warning: unused variable: `x`
   --> foo.rs:144:14
    |
144 |     fn stat2(x: &some_fields) -> u32 {
    |              ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: function cannot return without recursing
   --> foo.rs:160:5
    |
    |
160 |     fn Method(&self, x: u32) -> u32 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
161 |         self.Method(x);
    |         -------------- recursive call site
    = note: `#[warn(unconditional_recursion)]` on by default
    = note: `#[warn(unconditional_recursion)]` on by default
    = help: a `loop` may express intention better if this is on purpose
warning: variable does not need to be mutable
   --> foo.rs:326:9
    |
    |
326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default
---

warning: static is never used: `uni`
  --> foo.rs:27:1
   |
27 | static uni: &'static str = "Les Miséééééééérables";
   |
   = note: `#[warn(dead_code)]` on by default

warning: static is never used: `bob`
warning: static is never used: `bob`
  --> foo.rs:30:1
   |
30 | static bob: Option<graphviz::RenderOption> = None;

warning: function is never used: `test_alias`
  --> foo.rs:34:4
   |
   |
34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {


warning: field is never read: `ac_lut`
   |
   |
53 |     ac_lut: Option<[(i16, u8); 1 << LUT_BITS]>,


warning: struct is never constructed: `TupStruct`
   |
   |
56 | struct TupStruct(isize, isize, Box<str>);


warning: function is never used: `test_tup_struct`
   |
   |
58 | fn test_tup_struct(x: TupStruct) -> isize {

warning: enum is never used: `nested_enum`
  --> foo.rs:83:18
   |
   |
83 |         pub enum nested_enum {
   |                  ^^^^^^^^^^^

warning: static is never used: `yy`
 --> SubDir/mod.rs:6:1
  |
6 | static yy: usize = 25;

warning: function is never used: `hello`
  --> SubDir/mod.rs:11:20
   |
---

warning: associated function is never used: `stat2`
   --> foo.rs:144:5
    |
144 |     fn stat2(x: &some_fields) -> u32 {

warning: associated function is never used: `align_to`
   --> foo.rs:148:5
    |
---
    |
152 |     fn test(&mut self) {
    |     ^^^^^^^^^^^^^^^^^^

warning: function is never used: `f_with_params`
    |
    |
174 | fn f_with_params<T: SomeTrait>(x: &T) {

warning: variant is never constructed: `Ints`
   --> foo.rs:181:5
    |
    |
181 |     Ints(isize, isize),
    |     ^^^^^^^^^^^^^^^^^^

warning: variant is never constructed: `Floats`
   --> foo.rs:182:5
    |
182 |     Floats(f64, f64),

warning: variant is never constructed: `SomeConst1`
   --> foo.rs:189:5
    |
    |
189 |     SomeConst1,
    |     ^^^^^^^^^^

warning: variant is never constructed: `EnumStruct`
    |
    |
195 |     EnumStruct{a:isize, b:isize},


warning: variant is never constructed: `EnumStruct3`
    |
    |
197 |     EnumStruct3{f1:MyType, f2:MyType, f3:SomeEnum<'static>}


warning: function is never used: `matchSomeStructEnum2`
    |
    |
218 | fn matchSomeStructEnum2(se: SomeStructEnum) {

warning: field is never read: `used_link_args`
   --> foo.rs:282:5
    |
    |
282 |     used_link_args: RefCell<[&'static str; 0]>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct is never constructed: `CharSearcher`
   --> foo.rs:405:8
    |
405 | struct CharSearcher<'a>(<CharEqPattern as Pattern<'a>>::Searcher);

warning: function is never used: `test_format_args`
   --> foo.rs:426:4
    |
---
    |
437 | union TestUnion {
    |       ^^^^^^^^^

warning: struct is never constructed: `StructWithDocs`
    |
    |
458 | struct StructWithDocs;

warning: static variable `uni` should have an upper case name
  --> foo.rs:27:8
   |
   |
27 | static uni: &'static str = "Les Miséééééééérables";
   |        ^^^ help: convert the identifier to upper case: `UNI`
   = note: `#[warn(non_upper_case_globals)]` on by default


warning: static variable `yy` should have an upper case name
   |
   |
28 | static yy: usize = 25;
   |        ^^ help: convert the identifier to upper case (notice the capitalization): `YY`

warning: static variable `bob` should have an upper case name
   |
   |
30 | static bob: Option<graphviz::RenderOption> = None;
   |        ^^^ help: convert the identifier to upper case: `BOB`
warning: unused `std::result::Result` that must be used
  --> foo.rs:63:5
   |
   |
63 |     std::io::stdout().write_all(s.as_bytes());
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

---

warning: module `SubDir` should have a snake case name
  --> foo.rs:91:9
   |
91 | pub mod SubDir;
   |         ^^^^^^ help: convert the identifier to snake case: `sub_dir`
warning: module `SameDir2` should have a snake case name
  --> foo.rs:94:9
   |
94 | pub mod SameDir2;
94 | pub mod SameDir2;
   |         ^^^^^^^^ help: convert the identifier to snake case: `same_dir2`

warning: trait method `Method` should have a snake case name
   --> foo.rs:110:8
    |
110 |     fn Method(&self, x: u32) -> u32;
    |        ^^^^^^ help: convert the identifier to snake case: `method`

warning: function `matchSomeEnum` should have a snake case name
    |
    |
200 | fn matchSomeEnum(val: SomeEnum) {
    |    ^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_enum`

warning: function `matchSomeStructEnum` should have a snake case name
    |
    |
209 | fn matchSomeStructEnum(se: SomeStructEnum) {
    |    ^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum`
warning: the `a:` in this pattern is redundant
   --> foo.rs:211:36
    |
    |
211 |         SomeStructEnum::EnumStruct{a:a, ..} => println(&a.to_string()),
    |                                    ^^^ help: use shorthand field pattern: `a`
    = note: `#[warn(non_shorthand_field_patterns)]` on by default

warning: the `f1:` in this pattern is redundant
   --> foo.rs:212:37
   --> foo.rs:212:37
    |
212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
    |                                     ^^^^^ help: use shorthand field pattern: `f1`

warning: function `matchSomeStructEnum2` should have a snake case name
    |
    |
218 | fn matchSomeStructEnum2(se: SomeStructEnum) {
    |    ^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum2`
warning: the `f2:` in this pattern is redundant
   --> foo.rs:222:25
    |
    |
222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
    |                         ^^^^^^ help: use shorthand field pattern: `f2`

warning: function `matchSomeOtherEnum` should have a snake case name
    |
    |
228 | fn matchSomeOtherEnum(val: SomeOtherEnum) {
    |    ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_other_enum`

warning: static variable `yy` should have an upper case name
 --> SubDir/mod.rs:6:8
  |
6 | static yy: usize = 25;
  |        ^^ help: convert the identifier to upper case (notice the capitalization): `YY`
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/librustc_hir/definitions.rs:356:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (7461e6a1e 2020-05-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z save-analysis

---
test result: FAILED. 194 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-8/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:49:40
Build completed unsuccessfully in 1:49:40
== clock drift check ==
  local time: Sat May  2 21:15:44 UTC 2020
  network time: Sat, 02 May 2020 21:15:44 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71809/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71809/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4300) (python)
##[section]Finishing: Finalize Job
