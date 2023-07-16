plain
travis_time:end:07acfb50:start=1553734427266382890,finish=1553734429442298835,duration=2175915945
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:56] 
[01:20:56] running 9 tests
[01:20:56] iiiiiiiii
[01:20:56] 
[01:20:56]  finished in 0.153
[01:20:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:12] 
[01:21:12] running 120 tests
[01:21:38] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:42] .i......iii.i.....ii
[01:21:42] 
[01:21:42]  finished in 29.987
[01:21:42] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:48:40] 
[01:48:40] running 195 tests
[01:49:07] ..................i...i...............................................................Fi............ 100/195
[01:49:51] ...................................................i...F..................................i....
[01:49:51] 
[01:49:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:49:51] ---- [run-make] run-make-fulldeps/issues-41478-43796 stdout ----
[01:49:51] 
[01:49:51] 
[01:49:51] error: make failed
[01:49:51] status: exit code: 2
[01:49:51] command: "make"
[01:49:51] stdout:
[01:49:51] ------------------------------------------
[01:49:51] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issues-41478-43796'
[01:49:51] # Work in /tmp, because we need to create the `save-analysis-temp` folder.
[01:49:51] cp a.rs /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/
[01:49:51] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796 && LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796  -Zsave-analysis /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/a.rs 2> /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/stderr.txt || ( cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/stderr.txt && exit 1 )
[01:49:51] error: internal compiler error: src/librustc/hir/map/mod.rs:628: couldn't find node id 1 in the AST map
[01:49:51] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:49:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:49:51] error: aborting due to previous error
[01:49:51] 
---
[01:49:51] 
[01:49:51] ------------------------------------------
[01:49:51] stderr:
[01:49:51] ------------------------------------------
[01:49:51] make[1]: *** [all] Error 1
[01:49:51] ------------------------------------------
[01:49:51] 
[01:49:51] thread '[run-make] run-make-fulldeps/issues-41478-43796' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:49:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:49:51] status: exit code: 2
[01:49:51] command: "make"
[01:49:51] stdout:
[01:49:51] ------------------------------------------
[01:49:51] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/save-analysis'
[01:49:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  krate2.rs
[01:49:51] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  foo.rs -Zsave-analysis
[01:49:51] Makefile:6: recipe for target 'code' failed
[01:49:51] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/save-analysis'
[01:49:51] ------------------------------------------
[01:49:51] stderr:
[01:49:51] ------------------------------------------
[01:49:51] warning: unused `std::result::Result` that must be used
[01:49:51] warning: unused `std::result::Result` that must be used
[01:49:51]  --> krate2.rs:7:5
[01:49:51]   |
[01:49:51] 7 |     std::io::stdout().write_all(b"hello world!\n");
[01:49:51]   |
[01:49:51]   = note: #[warn(unused_must_use)] on by default
[01:49:51]   = note: #[warn(unused_must_use)] on by default
[01:49:51]   = note: this `Result` may be an `Err` variant, which should be handled
[01:49:51] 
[01:49:51] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:49:51]    --> foo.rs:422:40
[01:49:51]     |
[01:49:51] 422 | #[derive(Clone, Copy, Hash, Encodable, Decodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
[01:49:51] 
[01:49:51] 
[01:49:51] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:49:51]    --> foo.rs:422:29
[01:49:51]     |
[01:49:51] 422 | #[derive(Clone, Copy, Hash, Encodable, Decodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
[01:49:51] 
[01:49:51] 
[01:49:51] warning: unused import: `graphviz::RenderOption`
[01:49:51]   --> foo.rs:13:5
[01:49:51]    |
[01:49:51] 13 | use graphviz::RenderOption;
[01:49:51]    |
[01:49:51]    = note: #[warn(unused_imports)] on by default
[01:49:51] 
[01:49:51] warning: unused import: `HashSet`
[01:49:51] warning: unused import: `HashSet`
[01:49:51]   --> foo.rs:14:32
[01:49:51]    |
[01:49:51] 14 | use std::collections::{HashMap,HashSet};
[01:49:51] 
[01:49:51] warning: unused import: `std::mem::size_of`
[01:49:51]   --> foo.rs:23:5
[01:49:51]    |
---
[01:49:51]    |
[01:49:51] 70 |             use std::io::Write;
[01:49:51]    |                 ^^^^^^^^^^^^^^
[01:49:51] 
[01:49:51] warning: type `nested_struct` should have an upper camel case name
[01:49:51]   --> foo.rs:79:20
[01:49:51] 79 |         pub struct nested_struct {
[01:49:51] 79 |         pub struct nested_struct {
[01:49:51]    |                    ^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedStruct`
[01:49:51]    = note: #[warn(non_camel_case_types)] on by default
[01:49:51] 
[01:49:51] warning: type `nested_enum` should have an upper camel case name
[01:49:51]   --> foo.rs:83:18
[01:49:51]   --> foo.rs:83:18
[01:49:51]    |
[01:49:51] 83 |         pub enum nested_enum {
[01:49:51]    |                  ^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedEnum`
[01:49:51] 
[01:49:51] warning: unused import: `sub::sub2 as msalias`
[01:49:51]  --> SubDir/mod.rs:3:5
[01:49:51]   |
[01:49:51] 3 | use sub::sub2 as msalias;
[01:49:51] 
[01:49:51] 
[01:49:51] warning: unused import: `sub::sub2`
[01:49:51]  --> SubDir/mod.rs:4:5
[01:49:51]   |
[01:49:51] 4 | use sub::sub2;
[01:49:51] 
[01:49:51] 
[01:49:51] warning: type `nested_struct` should have an upper camel case name
[01:49:51]   --> SubDir/mod.rs:19:20
[01:49:51] 19 |         pub struct nested_struct {
[01:49:51] 19 |         pub struct nested_struct {
[01:49:51]    |                    ^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedStruct`
[01:49:51] 
[01:49:51] warning: type `nofields` should have an upper camel case name
[01:49:51]   --> foo.rs:96:8
[01:49:51]    |
[01:49:51] 96 | struct nofields;
[01:49:51]    |        ^^^^^^^^ help: convert the identifier to upper camel case: `Nofields`
[01:49:51] 
[01:49:51] warning: type `some_fields` should have an upper camel case name
[01:49:51]   --> foo.rs:99:8
[01:49:51] 99 | struct some_fields {
[01:49:51] 99 | struct some_fields {
[01:49:51]    |        ^^^^^^^^^^^ help: convert the identifier to upper camel case: `SomeFields`
[01:49:51] 
[01:49:51] warning: type `blah` should have an upper camel case name
[01:49:51]    --> foo.rs:281:12
[01:49:51]     |
[01:49:51] 281 | pub struct blah {
[01:49:51]     |            ^^^^ help: convert the identifier to upper camel case: `Blah`
[01:49:51] warning: unused variable: `s`
[01:49:51]   --> foo.rs:35:9
[01:49:51]    |
[01:49:51]    |
[01:49:51] 35 |     let s = sub_struct{ field2: 45u32, };
[01:49:51]    |         ^ help: consider prefixing with an underscore: `_s`
[01:49:51]    = note: #[warn(unused_variables)] on by default
[01:49:51] 
[01:49:51] warning: unused variable: `x`
[01:49:51]   --> foo.rs:41:9
---
[01:49:51] 
[01:49:51] warning: unused variable: `i`
[01:49:51]   --> foo.rs:34:28
[01:49:51]    |
[01:49:51] 34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {
[01:49:51]    |                            ^ help: consider prefixing with an underscore: `_i`
[01:49:51] warning: unused variable: `x`
[01:49:51]   --> foo.rs:38:12
[01:49:51]    |
[01:49:51] 38 |     fn foo(x: &Write) {}
[01:49:51] 38 |     fn foo(x: &Write) {}
[01:49:51]    |            ^ help: consider prefixing with an underscore: `_x`
[01:49:51] 
[01:49:51] warning: unused variable: `f1`
[01:49:51]    --> foo.rs:212:40
[01:49:51]     |
[01:49:51] 212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
[01:49:51]     |                                        ^^ help: consider prefixing with an underscore: `_f1`
[01:49:51] warning: unused variable: `f2`
[01:49:51]    --> foo.rs:222:29
[01:49:51]     |
[01:49:51]     |
[01:49:51] 222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
[01:49:51]     |                             ^^ help: consider prefixing with an underscore: `_f2`
[01:49:51] warning: unused variable: `f2`
[01:49:51]    --> foo.rs:223:49
[01:49:51]     |
[01:49:51]     |
[01:49:51] 223 |         EnumStruct3{f1, f3: SomeEnum::Ints(..), f2} => println(&f1.field1.to_string()),
[01:49:51]     |                                                 ^^ help: try ignoring the field: `f2: _`
[01:49:51] warning: unused variable: `y`
[01:49:51]    --> foo.rs:240:13
[01:49:51]     |
[01:49:51]     |
[01:49:51] 240 |     let (x, y): (u32, u32) = (5, 3);
[01:49:51]     |             ^ help: consider prefixing with an underscore: `_y`
[01:49:51] warning: unused variable: `r`
[01:49:51]    --> foo.rs:263:9
[01:49:51]     |
[01:49:51] 263 |     let r = some_fields::stat(y);
[01:49:51] 263 |     let r = some_fields::stat(y);
[01:49:51]     |         ^ help: consider prefixing with an underscore: `_r`
[01:49:51] 
[01:49:51] warning: unused variable: `r`
[01:49:51]    --> foo.rs:265:9
[01:49:51]     |
[01:49:51] 265 |     let r = SubTrait::stat2(&*s3);
[01:49:51]     |         ^ help: consider prefixing with an underscore: `_r`
[01:49:51] warning: unused variable: `z`
[01:49:51]    --> foo.rs:278:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 278 |     let z = closure(10, &*s);
[01:49:51]     |         ^ help: consider prefixing with an underscore: `_z`
[01:49:51] warning: unused variable: `a`
[01:49:51]    --> foo.rs:236:28
[01:49:51]     |
[01:49:51]     |
[01:49:51] 236 | fn hello<X: SomeTrait>((z, a) : (u32, String), ex: X) {
[01:49:51]     |                            ^ help: consider prefixing with an underscore: `_a`
[01:49:51] warning: unused variable: `ut`
[01:49:51]    --> foo.rs:315:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 315 |     let ut = "Les Miséééééééérables";
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_ut`
[01:49:51] warning: unused variable: `vs`
[01:49:51]    --> foo.rs:324:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 324 |     let vs = variable_str!(32);
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_vs`
[01:49:51] warning: unused variable: `candidates`
[01:49:51]    --> foo.rs:326:13
[01:49:51]     |
[01:49:51]     |
[01:49:51] 326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
[01:49:51]     |             ^^^^^^^^^^ help: consider prefixing with an underscore: `_candidates`
[01:49:51] warning: unused variable: `s1`
[01:49:51]    --> foo.rs:330:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 330 |     let s1 = nofields;
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_s1`
[01:49:51] warning: unused variable: `s3`
[01:49:51]    --> foo.rs:332:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 332 |     let s3: some_fields = some_fields{ field1: 55};
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_s3`
[01:49:51] warning: unused variable: `s4`
[01:49:51]    --> foo.rs:333:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 333 |     let s4: msalias::nested_struct = sub::sub2::nested_struct{ field2: 55};
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_s4`
[01:49:51] warning: unused variable: `s4`
[01:49:51]    --> foo.rs:334:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 334 |     let s4: msalias::nested_struct = sub2::nested_struct{ field2: 55};
[01:49:51]     |         ^^ help: consider prefixing with an underscore: `_s4`
[01:49:51] warning: unused variable: `s`
[01:49:51]    --> foo.rs:337:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 337 |     let s = SameDir::SameStruct{name: "Bob".to_string()};
[01:49:51]     |         ^ help: consider prefixing with an underscore: `_s`
[01:49:51] warning: unused variable: `s`
[01:49:51]    --> foo.rs:338:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 338 |     let s = SubDir::SubStruct{name:"Bob".to_string()};
[01:49:51]     |         ^ help: consider prefixing with an underscore: `_s`
[01:49:51] 
[01:49:51] warning: variable `x` is assigned to, but never used
[01:49:51]    --> foo.rs:299:21
[01:49:51]     |
[01:49:51] 299 |             let mut x = $src;
[01:49:51] ...
[01:49:51] ...
[01:49:51] 378 |     internal_vars!(x);
[01:49:51]     |
[01:49:51]     = note: consider using `_x` instead
[01:49:51] 
[01:49:51] warning: value assigned to `x` is never read
[01:49:51] warning: value assigned to `x` is never read
[01:49:51]    --> foo.rs:300:13
[01:49:51]     |
[01:49:51] 300 |             x += 100;
[01:49:51]     |             ^
[01:49:51] ...
[01:49:51] 378 |     internal_vars!(x);
[01:49:51]     |
[01:49:51]     = note: #[warn(unused_assignments)] on by default
[01:49:51]     = help: maybe it is overwritten before being read?
[01:49:51] 
[01:49:51] 
[01:49:51] warning: unused variable: `x`
[01:49:51]    --> foo.rs:122:14
[01:49:51]     |
[01:49:51] 122 |     fn stat2(x: &Self) -> u32 {
[01:49:51]     |              ^ help: consider prefixing with an underscore: `_x`
[01:49:51] warning: unused variable: `x`
[01:49:51]    --> foo.rs:144:14
[01:49:51]     |
[01:49:51]     |
[01:49:51] 144 |     fn stat2(x: &some_fields) -> u32 {
[01:49:51]     |              ^ help: consider prefixing with an underscore: `_x`
[01:49:51] warning: function cannot return without recursing
[01:49:51]    --> foo.rs:160:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 160 |     fn Method(&self, x: u32) -> u32 {
[01:49:51]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[01:49:51] 161 |         self.Method(x);
[01:49:51]     |         -------------- recursive call site
[01:49:51]     = note: #[warn(unconditional_recursion)] on by default
[01:49:51]     = note: #[warn(unconditional_recursion)] on by default
[01:49:51]     = help: a `loop` may express intention better if this is on purpose
[01:49:51] warning: variable does not need to be mutable
[01:49:51]    --> foo.rs:326:9
[01:49:51]     |
[01:49:51]     |
[01:49:51] 326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
[01:49:51]     |         |
[01:49:51]     |         help: remove this `mut`
[01:49:51]     |
[01:49:51]     = note: #[warn(unused_mut)] on by default
---
[01:49:51] 
[01:49:51] warning: static item is never used: `uni`
[01:49:51]   --> foo.rs:27:1
[01:49:51]    |
[01:49:51] 27 | static uni: &'static str = "Les Miséééééééérables";
[01:49:51]    |
[01:49:51]    = note: #[warn(dead_code)] on by default
[01:49:51] 
[01:49:51] warning: static item is never used: `bob`
[01:49:51] warning: static item is never used: `bob`
[01:49:51]   --> foo.rs:30:1
[01:49:51]    |
[01:49:51] 30 | static bob: Option<graphviz::RenderOption> = None;
[01:49:51] 
[01:49:51] warning: function is never used: `test_alias`
[01:49:51]   --> foo.rs:34:1
[01:49:51]    |
[01:49:51]    |
[01:49:51] 34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {
[01:49:51] 
[01:49:51] 
[01:49:51] warning: field is never used: `ac_lut`
[01:49:51]   --> foo.rs:53:5
[01:49:51]    |
[01:49:51] 53 |     ac_lut: Option<[(i16, u8); 1 << LUT_BITS]>,
[01:49:51] 
[01:49:51] 
[01:49:51] warning: struct is never constructed: `TupStruct`
[01:49:51]   --> foo.rs:56:1
[01:49:51]    |
[01:49:51] 56 | struct TupStruct(isize, isize, Box<str>);
[01:49:51] 
[01:49:51] warning: function is never used: `test_tup_struct`
[01:49:51]   --> foo.rs:58:1
[01:49:51]    |
[01:49:51]    |
[01:49:51] 58 | fn test_tup_struct(x: TupStruct) -> isize {
[01:49:51] 
[01:49:51] warning: enum is never used: `nested_enum`
[01:49:51]   --> foo.rs:83:9
[01:49:51]    |
[01:49:51]    |
[01:49:51] 83 |         pub enum nested_enum {
[01:49:51]    |         ^^^^^^^^^^^^^^^^^^^^
[01:49:51] 
[01:49:51] warning: static item is never used: `yy`
[01:49:51]  --> SubDir/mod.rs:6:1
[01:49:51]   |
[01:49:51] 6 | static yy: usize = 25;
[01:49:51] 
[01:49:51] 
[01:49:51] warning: function is never used: `hello`
[01:49:51]   --> SubDir/mod.rs:11:13
[01:49:51]    |
[01:49:51] 11 |             pub fn hello() {
[01:49:51] 
[01:49:51] 
[01:49:51] warning: function is never used: `hello`
[01:49:51]   --> SubDir/mod.rs:15:9
[01:49:51]    |
[01:49:51] 15 |         pub fn hello() {
[01:49:51] 
[01:49:51] warning: struct is never constructed: `nested_struct`
[01:49:51]   --> SubDir/mod.rs:19:9
[01:49:51]    |
[01:49:51]    |
[01:49:51] 19 |         pub struct nested_struct {
[01:49:51]    |         ^^^^^^^^^^^^^^^^^^^^^^^^
[01:49:51] 
[01:49:51] warning: method is never used: `stat2`
[01:49:51]    --> foo.rs:144:5
[01:49:51]     |
[01:49:51] 144 |     fn stat2(x: &some_fields) -> u32 {
[01:49:51] 
[01:49:51] warning: method is never used: `align_to`
[01:49:51]    --> foo.rs:148:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 148 |     fn align_to<T>(&mut self) {
[01:49:51] 
[01:49:51] warning: method is never used: `test`
[01:49:51]    --> foo.rs:152:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 152 |     fn test(&mut self) {
[01:49:51]     |     ^^^^^^^^^^^^^^^^^^
[01:49:51] 
[01:49:51] warning: function is never used: `f_with_params`
[01:49:51]    --> foo.rs:174:1
[01:49:51]     |
[01:49:51] 174 | fn f_with_params<T: SomeTrait>(x: &T) {
[01:49:51] 
[01:49:51] warning: variant is never constructed: `Ints`
[01:49:51]    --> foo.rs:181:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 181 |     Ints(isize, isize),
[01:49:51] 
[01:49:51] warning: variant is never constructed: `Floats`
[01:49:51]    --> foo.rs:182:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 182 |     Floats(f64, f64),
[01:49:51] 
[01:49:51] warning: variant is never constructed: `SomeConst1`
[01:49:51]    --> foo.rs:189:5
[01:49:51]     |
---
[01:49:51] 
[01:49:51] warning: variant is never constructed: `EnumStruct`
[01:49:51]    --> foo.rs:195:5
[01:49:51]     |
[01:49:51] 195 |     EnumStruct{a:isize, b:isize},
[01:49:51] 
[01:49:51] warning: variant is never constructed: `EnumStruct3`
[01:49:51]    --> foo.rs:197:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 197 |     EnumStruct3{f1:MyType, f2:MyType, f3:SomeEnum<'static>}
[01:49:51] 
[01:49:51] 
[01:49:51] warning: function is never used: `matchSomeStructEnum2`
[01:49:51]    --> foo.rs:218:1
[01:49:51]     |
[01:49:51] 218 | fn matchSomeStructEnum2(se: SomeStructEnum) {
[01:49:51] 
[01:49:51] warning: field is never used: `used_link_args`
[01:49:51]    --> foo.rs:282:5
[01:49:51]     |
[01:49:51]     |
[01:49:51] 282 |     used_link_args: RefCell<[&'static str; 0]>,
[01:49:51] 
[01:49:51] warning: struct is never constructed: `CharSearcher`
[01:49:51]    --> foo.rs:405:1
[01:49:51]     |
[01:49:51]     |
[01:49:51] 405 | struct CharSearcher<'a>(<CharEqPattern as Pattern<'a>>::Searcher);
[01:49:51] 
[01:49:51] warning: function is never used: `test_format_args`
[01:49:51]    --> foo.rs:425:1
[01:49:51]     |
---
[01:49:51]     |
[01:49:51] 436 | union TestUnion {
[01:49:51]     | ^^^^^^^^^^^^^^^
[01:49:51] 
[01:49:51] warning: struct is never constructed: `SilenceGenerator`
[01:49:51]    --> foo.rs:442:1
[01:49:51]     |
[01:49:51] 442 | struct SilenceGenerator;
[01:49:51] 
[01:49:51] 
[01:49:51] warning: struct is never constructed: `StructWithDocs`
[01:49:51]    --> foo.rs:457:1
[01:49:51]     |
[01:49:51] 457 | struct StructWithDocs;
[01:49:51] 
[01:49:51] 
[01:49:51] warning: static variable `uni` should have an upper case name
[01:49:51]   --> foo.rs:27:8
[01:49:51]    |
[01:49:51] 27 | static uni: &'static str = "Les Miséééééééérables";
[01:49:51]    |        ^^^ help: convert the identifier to upper case: `UNI`
[01:49:51]    = note: #[warn(non_upper_case_globals)] on by default
[01:49:51] 
[01:49:51] 
[01:49:51] warning: static variable `yy` should have an upper case name
[01:49:51]   --> foo.rs:28:8
[01:49:51]    |
[01:49:51] 28 | static yy: usize = 25;
[01:49:51]    |        ^^ help: convert the identifier to upper case: `YY`
[01:49:51] 
[01:49:51] warning: static variable `bob` should have an upper case name
[01:49:51]   --> foo.rs:30:8
[01:49:51]    |
[01:49:51] 30 | static bob: Option<graphviz::RenderOption> = None;
[01:49:51]    |        ^^^ help: convert the identifier to upper case: `BOB`
[01:49:51] warning: unused `std::result::Result` that must be used
[01:49:51]   --> foo.rs:63:5
[01:49:51]    |
[01:49:51]    |
[01:49:51] 63 |     std::io::stdout().write_all(s.as_bytes());
[01:49:51]    |
[01:49:51]    = note: #[warn(unused_must_use)] on by default
[01:49:51]    = note: #[warn(unused_must_use)] on by default
[01:49:51]    = note: this `Result` may be an `Err` variant, which should be handled
[01:49:51] 
[01:49:51] warning: module `SameDir` should have a snake case name
[01:49:51]   --> foo.rs:90:9
[01:49:51]    |
[01:49:51] 90 | pub mod SameDir;
[01:49:51]    |         ^^^^^^^ help: convert the identifier to snake case: `same_dir`
[01:49:51]    |
[01:49:51]    = note: #[warn(non_snake_case)] on by default
[01:49:51] 
[01:49:51] warning: module `SubDir` should have a snake case name
[01:49:51]   --> foo.rs:91:9
[01:49:51] 91 | pub mod SubDir;
[01:49:51] 91 | pub mod SubDir;
[01:49:51]    |         ^^^^^^ help: convert the identifier to snake case: `sub_dir`
[01:49:51] 
[01:49:51] warning: static variable `yy` should have an upper case name
[01:49:51]  --> SubDir/mod.rs:6:8
[01:49:51]   |
[01:49:51] 6 | static yy: usize = 25;
[01:49:51]   |        ^^ help: convert the identifier to upper case: `YY`
[01:49:51] 
[01:49:51] warning: module `SameDir2` should have a snake case name
[01:49:51]   --> foo.rs:94:9
[01:49:51]    |
[01:49:51] 94 | pub mod SameDir2;
[01:49:51]    |         ^^^^^^^^ help: convert the identifier to snake case: `same_dir2`
[01:49:51] 
[01:49:51] warning: trait method `Method` should have a snake case name
[01:49:51]    --> foo.rs:110:8
[01:49:51]     |
[01:49:51] 110 |     fn Method(&self, x: u32) -> u32;
[01:49:51]     |        ^^^^^^ help: convert the identifier to snake case: `method`
[01:49:51] 
[01:49:51] warning: function `matchSomeEnum` should have a snake case name
[01:49:51]    --> foo.rs:200:4
[01:49:51]     |
[01:49:51] 200 | fn matchSomeEnum(val: SomeEnum) {
[01:49:51]     |    ^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_enum`
[01:49:51] 
[01:49:51] warning: function `matchSomeStructEnum` should have a snake case name
[01:49:51]    --> foo.rs:209:4
[01:49:51]     |
[01:49:51] 209 | fn matchSomeStructEnum(se: SomeStructEnum) {
[01:49:51]     |    ^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum`
[01:49:51] 
[01:49:51] warning: the `a:` in this pattern is redundant
[01:49:51]    --> foo.rs:211:36
[01:49:51]     |
[01:49:51] 211 |         SomeStructEnum::EnumStruct{a:a, ..} => println(&a.to_string()),
[01:49:51]     |                                    --^
[01:49:51]     |                                    help: remove this
[01:49:51]     |
[01:49:51]     = note: #[warn(non_shorthand_field_patterns)] on by default
[01:49:51] 
[01:49:51] 
[01:49:51] warning: the `f1:` in this pattern is redundant
[01:49:51]    --> foo.rs:212:37
[01:49:51]     |
[01:49:51] 212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
[01:49:51]     |                                     ---^^
[01:49:51]     |                                     help: remove this
[01:49:51] 
[01:49:51] 
[01:49:51] warning: function `matchSomeStructEnum2` should have a snake case name
[01:49:51]    --> foo.rs:218:4
[01:49:51]     |
[01:49:51] 218 | fn matchSomeStructEnum2(se: SomeStructEnum) {
[01:49:51]     |    ^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum2`
[01:49:51] 
[01:49:51] warning: the `f2:` in this pattern is redundant
[01:49:51]    --> foo.rs:222:25
[01:49:51]     |
[01:49:51] 222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
[01:49:51]     |                         ---^^^
[01:49:51]     |                         help: remove this
[01:49:51] 
[01:49:51] 
[01:49:51] warning: function `matchSomeOtherEnum` should have a snake case name
[01:49:51]    --> foo.rs:228:4
[01:49:51]     |
[01:49:51] 228 | fn matchSomeOtherEnum(val: SomeOtherEnum) {
[01:49:51]     |    ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_other_enum`
[01:49:51] 
[01:49:51] error: internal compiler error: src/librustc/hir/map/mod.rs:628: couldn't find node id 1 in the AST map
[01:49:51] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
[01:49:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:49:51] error: aborting due to previous error
[01:49:51] 
---
[01:49:51] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:49:51] 
[01:49:51] note: compiler flags: -Z save-analysis
[01:49:51] 
[01:49:51] make[1]: *** [code] Error 101
[01:49:51] ------------------------------------------
[01:49:51] 
[01:49:51] thread '[run-make] run-make-fulldeps/save-analysis' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:49:51] 
---
[01:49:51] test result: FAILED. 188 passed; 2 failed; 5 ignored; 0 measured; 0 filtered out
[01:49:51] 
[01:49:51] 
[01:49:51] 
[01:49:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:49:51] 
[01:49:51] 
[01:49:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:51] Build completed unsuccessfully in 0:40:33
[01:49:51] Build completed unsuccessfully in 0:40:33
[01:49:51] make: *** [check] Error 1
[01:49:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34884870
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 28 02:43:52 UTC 2019
---
travis_time:end:01e1575a:start=1553741033898587782,finish=1553741033963149729,duration=64561947
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07df003e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:072e52f0
$ dmesg | grep -i kill
