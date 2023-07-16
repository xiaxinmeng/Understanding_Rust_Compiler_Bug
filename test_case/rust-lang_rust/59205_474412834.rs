plain
travis_time:end:003ecc99:start=1553000054958622615,finish=1553000057204115123,duration=2245492508
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:57] 
[01:42:57] running 120 tests
[01:43:23] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:43:28] .i......iii.i.....ii
[01:43:28] 
[01:43:28]  finished in 30.885
[01:43:28] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:11:22] 
[02:11:22] running 195 tests
[02:11:52] ..................i...i................................................................Fi........... 100/195
[02:12:36] ...................................................i....FF................................i....
[02:12:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[02:12:36] 
[02:12:36] ---- [run-make] run-make-fulldeps/issues-41478-43796 stdout ----
[02:12:36] 
[02:12:36] 
[02:12:36] error: make failed
[02:12:36] status: exit code: 2
[02:12:36] command: "make"
[02:12:36] stdout:
[02:12:36] ------------------------------------------
[02:12:36] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issues-41478-43796'
[02:12:36] # Work in /tmp, because we need to create the `save-analysis-temp` folder.
[02:12:36] cp a.rs /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/
[02:12:36] cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796 && LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796  -Zsave-analysis /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/a.rs 2> /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/stderr.txt || ( cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issues-41478-43796/issues-41478-43796/stderr.txt && exit 1 )
[02:12:36] thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:499:9
[02:12:36] 
[02:12:36] error: internal compiler error: unexpected panic
[02:12:36] 
[02:12:36] note: the compiler unexpectedly panicked. this is a bug.
---
[02:12:36] 
[02:12:36] ------------------------------------------
[02:12:36] stderr:
[02:12:36] ------------------------------------------
[02:12:36] make[1]: *** [all] Error 1
[02:12:36] ------------------------------------------
[02:12:36] 
[02:12:36] thread '[run-make] run-make-fulldeps/issues-41478-43796' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[02:12:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[02:12:36] status: exit code: 2
[02:12:36] command: "make"
[02:12:36] stdout:
[02:12:36] ------------------------------------------
[02:12:36] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/save-analysis'
[02:12:36] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  krate2.rs
[02:12:36] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis/save-analysis  foo.rs -Zsave-analysis
[02:12:36] Makefile:6: recipe for target 'code' failed
[02:12:36] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/save-analysis'
[02:12:36] ------------------------------------------
[02:12:36] stderr:
[02:12:36] ------------------------------------------
[02:12:36] warning: unused `std::result::Result` that must be used
[02:12:36] warning: unused `std::result::Result` that must be used
[02:12:36]  --> krate2.rs:7:5
[02:12:36]   |
[02:12:36] 7 |     std::io::stdout().write_all(b"hello world!\n");
[02:12:36]   |
[02:12:36]   = note: #[warn(unused_must_use)] on by default
[02:12:36]   = note: #[warn(unused_must_use)] on by default
[02:12:36]   = note: this `Result` may be an `Err` variant, which should be handled
[02:12:36] 
[02:12:36] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[02:12:36]    --> foo.rs:422:40
[02:12:36]     |
[02:12:36] 422 | #[derive(Clone, Copy, Hash, Encodable, Decodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
[02:12:36] 
[02:12:36] 
[02:12:36] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[02:12:36]    --> foo.rs:422:29
[02:12:36]     |
[02:12:36] 422 | #[derive(Clone, Copy, Hash, Encodable, Decodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
[02:12:36] 
[02:12:36] 
[02:12:36] warning: unused import: `graphviz::RenderOption`
[02:12:36]   --> foo.rs:13:5
[02:12:36]    |
[02:12:36] 13 | use graphviz::RenderOption;
[02:12:36]    |
[02:12:36]    = note: #[warn(unused_imports)] on by default
[02:12:36] 
[02:12:36] warning: unused import: `HashSet`
[02:12:36] warning: unused import: `HashSet`
[02:12:36]   --> foo.rs:14:32
[02:12:36]    |
[02:12:36] 14 | use std::collections::{HashMap,HashSet};
[02:12:36] 
[02:12:36] warning: unused import: `std::mem::size_of`
[02:12:36]   --> foo.rs:23:5
[02:12:36]    |
---
[02:12:36]    |
[02:12:36] 70 |             use std::io::Write;
[02:12:36]    |                 ^^^^^^^^^^^^^^
[02:12:36] 
[02:12:36] warning: type `nested_struct` should have an upper camel case name
[02:12:36]   --> foo.rs:79:20
[02:12:36] 79 |         pub struct nested_struct {
[02:12:36] 79 |         pub struct nested_struct {
[02:12:36]    |                    ^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedStruct`
[02:12:36]    = note: #[warn(non_camel_case_types)] on by default
[02:12:36] 
[02:12:36] warning: type `nested_enum` should have an upper camel case name
[02:12:36]   --> foo.rs:83:18
[02:12:36]   --> foo.rs:83:18
[02:12:36]    |
[02:12:36] 83 |         pub enum nested_enum {
[02:12:36]    |                  ^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedEnum`
[02:12:36] 
[02:12:36] warning: unused import: `sub::sub2 as msalias`
[02:12:36]  --> SubDir/mod.rs:3:5
[02:12:36]   |
[02:12:36] 3 | use sub::sub2 as msalias;
[02:12:36] 
[02:12:36] 
[02:12:36] warning: unused import: `sub::sub2`
[02:12:36]  --> SubDir/mod.rs:4:5
[02:12:36]   |
[02:12:36] 4 | use sub::sub2;
[02:12:36] 
[02:12:36] 
[02:12:36] warning: type `nested_struct` should have an upper camel case name
[02:12:36]   --> SubDir/mod.rs:19:20
[02:12:36] 19 |         pub struct nested_struct {
[02:12:36] 19 |         pub struct nested_struct {
[02:12:36]    |                    ^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NestedStruct`
[02:12:36] 
[02:12:36] warning: type `nofields` should have an upper camel case name
[02:12:36]   --> foo.rs:96:8
[02:12:36]    |
[02:12:36] 96 | struct nofields;
[02:12:36]    |        ^^^^^^^^ help: convert the identifier to upper camel case: `Nofields`
[02:12:36] 
[02:12:36] warning: type `some_fields` should have an upper camel case name
[02:12:36]   --> foo.rs:99:8
[02:12:36] 99 | struct some_fields {
[02:12:36] 99 | struct some_fields {
[02:12:36]    |        ^^^^^^^^^^^ help: convert the identifier to upper camel case: `SomeFields`
[02:12:36] 
[02:12:36] warning: type `blah` should have an upper camel case name
[02:12:36]    --> foo.rs:281:12
[02:12:36]     |
[02:12:36] 281 | pub struct blah {
[02:12:36]     |            ^^^^ help: convert the identifier to upper camel case: `Blah`
[02:12:36] warning: unused variable: `s`
[02:12:36]   --> foo.rs:35:9
[02:12:36]    |
[02:12:36]    |
[02:12:36] 35 |     let s = sub_struct{ field2: 45u32, };
[02:12:36]    |         ^ help: consider prefixing with an underscore: `_s`
[02:12:36]    = note: #[warn(unused_variables)] on by default
[02:12:36] 
[02:12:36] warning: unused variable: `x`
[02:12:36]   --> foo.rs:41:9
---
[02:12:36] 
[02:12:36] warning: unused variable: `i`
[02:12:36]   --> foo.rs:34:28
[02:12:36]    |
[02:12:36] 34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {
[02:12:36]    |                            ^ help: consider prefixing with an underscore: `_i`
[02:12:36] warning: unused variable: `x`
[02:12:36]   --> foo.rs:38:12
[02:12:36]    |
[02:12:36] 38 |     fn foo(x: &Write) {}
[02:12:36] 38 |     fn foo(x: &Write) {}
[02:12:36]    |            ^ help: consider prefixing with an underscore: `_x`
[02:12:36] 
[02:12:36] warning: unused variable: `f1`
[02:12:36]    --> foo.rs:212:40
[02:12:36]     |
[02:12:36] 212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
[02:12:36]     |                                        ^^ help: consider prefixing with an underscore: `_f1`
[02:12:36] warning: unused variable: `f2`
[02:12:36]    --> foo.rs:222:29
[02:12:36]     |
[02:12:36]     |
[02:12:36] 222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
[02:12:36]     |                             ^^ help: consider prefixing with an underscore: `_f2`
[02:12:36] warning: unused variable: `f2`
[02:12:36]    --> foo.rs:223:49
[02:12:36]     |
[02:12:36]     |
[02:12:36] 223 |         EnumStruct3{f1, f3: SomeEnum::Ints(..), f2} => println(&f1.field1.to_string()),
[02:12:36]     |                                                 ^^ help: try ignoring the field: `f2: _`
[02:12:36] warning: unused variable: `y`
[02:12:36]    --> foo.rs:240:13
[02:12:36]     |
[02:12:36]     |
[02:12:36] 240 |     let (x, y): (u32, u32) = (5, 3);
[02:12:36]     |             ^ help: consider prefixing with an underscore: `_y`
[02:12:36] warning: unused variable: `r`
[02:12:36]    --> foo.rs:263:9
[02:12:36]     |
[02:12:36] 263 |     let r = some_fields::stat(y);
[02:12:36] 263 |     let r = some_fields::stat(y);
[02:12:36]     |         ^ help: consider prefixing with an underscore: `_r`
[02:12:36] 
[02:12:36] warning: unused variable: `r`
[02:12:36]    --> foo.rs:265:9
[02:12:36]     |
[02:12:36] 265 |     let r = SubTrait::stat2(&*s3);
[02:12:36]     |         ^ help: consider prefixing with an underscore: `_r`
[02:12:36] warning: unused variable: `z`
[02:12:36]    --> foo.rs:278:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 278 |     let z = closure(10, &*s);
[02:12:36]     |         ^ help: consider prefixing with an underscore: `_z`
[02:12:36] warning: unused variable: `a`
[02:12:36]    --> foo.rs:236:28
[02:12:36]     |
[02:12:36]     |
[02:12:36] 236 | fn hello<X: SomeTrait>((z, a) : (u32, String), ex: X) {
[02:12:36]     |                            ^ help: consider prefixing with an underscore: `_a`
[02:12:36] warning: unused variable: `ut`
[02:12:36]    --> foo.rs:315:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 315 |     let ut = "Les Miséééééééérables";
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_ut`
[02:12:36] warning: unused variable: `vs`
[02:12:36]    --> foo.rs:324:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 324 |     let vs = variable_str!(32);
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_vs`
[02:12:36] warning: unused variable: `candidates`
[02:12:36]    --> foo.rs:326:13
[02:12:36]     |
[02:12:36]     |
[02:12:36] 326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
[02:12:36]     |             ^^^^^^^^^^ help: consider prefixing with an underscore: `_candidates`
[02:12:36] warning: unused variable: `s1`
[02:12:36]    --> foo.rs:330:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 330 |     let s1 = nofields;
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_s1`
[02:12:36] warning: unused variable: `s3`
[02:12:36]    --> foo.rs:332:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 332 |     let s3: some_fields = some_fields{ field1: 55};
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_s3`
[02:12:36] warning: unused variable: `s4`
[02:12:36]    --> foo.rs:333:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 333 |     let s4: msalias::nested_struct = sub::sub2::nested_struct{ field2: 55};
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_s4`
[02:12:36] warning: unused variable: `s4`
[02:12:36]    --> foo.rs:334:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 334 |     let s4: msalias::nested_struct = sub2::nested_struct{ field2: 55};
[02:12:36]     |         ^^ help: consider prefixing with an underscore: `_s4`
[02:12:36] warning: unused variable: `s`
[02:12:36]    --> foo.rs:337:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 337 |     let s = SameDir::SameStruct{name: "Bob".to_string()};
[02:12:36]     |         ^ help: consider prefixing with an underscore: `_s`
[02:12:36] warning: unused variable: `s`
[02:12:36]    --> foo.rs:338:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 338 |     let s = SubDir::SubStruct{name:"Bob".to_string()};
[02:12:36]     |         ^ help: consider prefixing with an underscore: `_s`
[02:12:36] 
[02:12:36] warning: variable `x` is assigned to, but never used
[02:12:36]    --> foo.rs:299:21
[02:12:36]     |
[02:12:36] 299 |             let mut x = $src;
[02:12:36] ...
[02:12:36] ...
[02:12:36] 378 |     internal_vars!(x);
[02:12:36]     |
[02:12:36]     = note: consider using `_x` instead
[02:12:36] 
[02:12:36] warning: value assigned to `x` is never read
[02:12:36] warning: value assigned to `x` is never read
[02:12:36]    --> foo.rs:300:13
[02:12:36]     |
[02:12:36] 300 |             x += 100;
[02:12:36]     |             ^
[02:12:36] ...
[02:12:36] 378 |     internal_vars!(x);
[02:12:36]     |
[02:12:36]     = note: #[warn(unused_assignments)] on by default
[02:12:36]     = help: maybe it is overwritten before being read?
[02:12:36] 
[02:12:36] 
[02:12:36] warning: unused variable: `x`
[02:12:36]    --> foo.rs:122:14
[02:12:36]     |
[02:12:36] 122 |     fn stat2(x: &Self) -> u32 {
[02:12:36]     |              ^ help: consider prefixing with an underscore: `_x`
[02:12:36] warning: unused variable: `x`
[02:12:36]    --> foo.rs:144:14
[02:12:36]     |
[02:12:36]     |
[02:12:36] 144 |     fn stat2(x: &some_fields) -> u32 {
[02:12:36]     |              ^ help: consider prefixing with an underscore: `_x`
[02:12:36] warning: function cannot return without recursing
[02:12:36]    --> foo.rs:160:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 160 |     fn Method(&self, x: u32) -> u32 {
[02:12:36]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[02:12:36] 161 |         self.Method(x);
[02:12:36]     |         -------------- recursive call site
[02:12:36]     = note: #[warn(unconditional_recursion)] on by default
[02:12:36]     = note: #[warn(unconditional_recursion)] on by default
[02:12:36]     = help: a `loop` may express intention better if this is on purpose
[02:12:36] warning: variable does not need to be mutable
[02:12:36]    --> foo.rs:326:9
[02:12:36]     |
[02:12:36]     |
[02:12:36] 326 |     let mut candidates: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
[02:12:36]     |         |
[02:12:36]     |         help: remove this `mut`
[02:12:36]     |
[02:12:36]     = note: #[warn(unused_mut)] on by default
---
[02:12:36] 
[02:12:36] warning: static item is never used: `uni`
[02:12:36]   --> foo.rs:27:1
[02:12:36]    |
[02:12:36] 27 | static uni: &'static str = "Les Miséééééééérables";
[02:12:36]    |
[02:12:36]    = note: #[warn(dead_code)] on by default
[02:12:36] 
[02:12:36] warning: static item is never used: `bob`
[02:12:36] warning: static item is never used: `bob`
[02:12:36]   --> foo.rs:30:1
[02:12:36]    |
[02:12:36] 30 | static bob: Option<graphviz::RenderOption> = None;
[02:12:36] 
[02:12:36] warning: function is never used: `test_alias`
[02:12:36]   --> foo.rs:34:1
[02:12:36]    |
[02:12:36]    |
[02:12:36] 34 | fn test_alias<I: Iterator>(i: Option<<I as Iterator>::Item>) {
[02:12:36] 
[02:12:36] 
[02:12:36] warning: field is never used: `ac_lut`
[02:12:36]   --> foo.rs:53:5
[02:12:36]    |
[02:12:36] 53 |     ac_lut: Option<[(i16, u8); 1 << LUT_BITS]>,
[02:12:36] 
[02:12:36] 
[02:12:36] warning: struct is never constructed: `TupStruct`
[02:12:36]   --> foo.rs:56:1
[02:12:36]    |
[02:12:36] 56 | struct TupStruct(isize, isize, Box<str>);
[02:12:36] 
[02:12:36] warning: function is never used: `test_tup_struct`
[02:12:36]   --> foo.rs:58:1
[02:12:36]    |
[02:12:36]    |
[02:12:36] 58 | fn test_tup_struct(x: TupStruct) -> isize {
[02:12:36] 
[02:12:36] warning: enum is never used: `nested_enum`
[02:12:36]   --> foo.rs:83:9
[02:12:36]    |
[02:12:36]    |
[02:12:36] 83 |         pub enum nested_enum {
[02:12:36]    |         ^^^^^^^^^^^^^^^^^^^^
[02:12:36] 
[02:12:36] warning: static item is never used: `yy`
[02:12:36]  --> SubDir/mod.rs:6:1
[02:12:36]   |
[02:12:36] 6 | static yy: usize = 25;
[02:12:36] 
[02:12:36] 
[02:12:36] warning: function is never used: `hello`
[02:12:36]   --> SubDir/mod.rs:11:13
[02:12:36]    |
[02:12:36] 11 |             pub fn hello() {
[02:12:36] 
[02:12:36] 
[02:12:36] warning: function is never used: `hello`
[02:12:36]   --> SubDir/mod.rs:15:9
[02:12:36]    |
[02:12:36] 15 |         pub fn hello() {
[02:12:36] 
[02:12:36] warning: struct is never constructed: `nested_struct`
[02:12:36]   --> SubDir/mod.rs:19:9
[02:12:36]    |
[02:12:36]    |
[02:12:36] 19 |         pub struct nested_struct {
[02:12:36]    |         ^^^^^^^^^^^^^^^^^^^^^^^^
[02:12:36] 
[02:12:36] warning: method is never used: `stat2`
[02:12:36]    --> foo.rs:144:5
[02:12:36]     |
[02:12:36] 144 |     fn stat2(x: &some_fields) -> u32 {
[02:12:36] 
[02:12:36] warning: method is never used: `align_to`
[02:12:36]    --> foo.rs:148:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 148 |     fn align_to<T>(&mut self) {
[02:12:36] 
[02:12:36] warning: method is never used: `test`
[02:12:36]    --> foo.rs:152:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 152 |     fn test(&mut self) {
[02:12:36]     |     ^^^^^^^^^^^^^^^^^^
[02:12:36] 
[02:12:36] warning: function is never used: `f_with_params`
[02:12:36]    --> foo.rs:174:1
[02:12:36]     |
[02:12:36] 174 | fn f_with_params<T: SomeTrait>(x: &T) {
[02:12:36] 
[02:12:36] warning: variant is never constructed: `Ints`
[02:12:36]    --> foo.rs:181:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 181 |     Ints(isize, isize),
[02:12:36] 
[02:12:36] warning: variant is never constructed: `Floats`
[02:12:36]    --> foo.rs:182:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 182 |     Floats(f64, f64),
[02:12:36] 
[02:12:36] warning: variant is never constructed: `SomeConst1`
[02:12:36]    --> foo.rs:189:5
[02:12:36]     |
---
[02:12:36] 
[02:12:36] warning: variant is never constructed: `EnumStruct`
[02:12:36]    --> foo.rs:195:5
[02:12:36]     |
[02:12:36] 195 |     EnumStruct{a:isize, b:isize},
[02:12:36] 
[02:12:36] warning: variant is never constructed: `EnumStruct3`
[02:12:36]    --> foo.rs:197:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 197 |     EnumStruct3{f1:MyType, f2:MyType, f3:SomeEnum<'static>}
[02:12:36] 
[02:12:36] 
[02:12:36] warning: function is never used: `matchSomeStructEnum2`
[02:12:36]    --> foo.rs:218:1
[02:12:36]     |
[02:12:36] 218 | fn matchSomeStructEnum2(se: SomeStructEnum) {
[02:12:36] 
[02:12:36] warning: field is never used: `used_link_args`
[02:12:36]    --> foo.rs:282:5
[02:12:36]     |
[02:12:36]     |
[02:12:36] 282 |     used_link_args: RefCell<[&'static str; 0]>,
[02:12:36] 
[02:12:36] warning: struct is never constructed: `CharSearcher`
[02:12:36]    --> foo.rs:405:1
[02:12:36]     |
[02:12:36]     |
[02:12:36] 405 | struct CharSearcher<'a>(<CharEqPattern as Pattern<'a>>::Searcher);
[02:12:36] 
[02:12:36] warning: function is never used: `test_format_args`
[02:12:36]    --> foo.rs:425:1
[02:12:36]     |
---
[02:12:36]     |
[02:12:36] 436 | union TestUnion {
[02:12:36]     | ^^^^^^^^^^^^^^^
[02:12:36] 
[02:12:36] warning: struct is never constructed: `SilenceGenerator`
[02:12:36]    --> foo.rs:442:1
[02:12:36]     |
[02:12:36] 442 | struct SilenceGenerator;
[02:12:36] 
[02:12:36] 
[02:12:36] warning: struct is never constructed: `StructWithDocs`
[02:12:36]    --> foo.rs:457:1
[02:12:36]     |
[02:12:36] 457 | struct StructWithDocs;
[02:12:36] 
[02:12:36] 
[02:12:36] warning: static variable `uni` should have an upper case name
[02:12:36]   --> foo.rs:27:8
[02:12:36]    |
[02:12:36] 27 | static uni: &'static str = "Les Miséééééééérables";
[02:12:36]    |        ^^^ help: convert the identifier to upper case: `UNI`
[02:12:36]    = note: #[warn(non_upper_case_globals)] on by default
[02:12:36] 
[02:12:36] 
[02:12:36] warning: static variable `yy` should have an upper case name
[02:12:36]   --> foo.rs:28:8
[02:12:36]    |
[02:12:36] 28 | static yy: usize = 25;
[02:12:36]    |        ^^ help: convert the identifier to upper case: `YY`
[02:12:36] 
[02:12:36] warning: static variable `bob` should have an upper case name
[02:12:36]   --> foo.rs:30:8
[02:12:36]    |
[02:12:36] 30 | static bob: Option<graphviz::RenderOption> = None;
[02:12:36]    |        ^^^ help: convert the identifier to upper case: `BOB`
[02:12:36] warning: unused `std::result::Result` that must be used
[02:12:36]   --> foo.rs:63:5
[02:12:36]    |
[02:12:36]    |
[02:12:36] 63 |     std::io::stdout().write_all(s.as_bytes());
[02:12:36]    |
[02:12:36]    = note: #[warn(unused_must_use)] on by default
[02:12:36]    = note: #[warn(unused_must_use)] on by default
[02:12:36]    = note: this `Result` may be an `Err` variant, which should be handled
[02:12:36] 
[02:12:36] warning: module `SameDir` should have a snake case name
[02:12:36]   --> foo.rs:90:9
[02:12:36]    |
[02:12:36] 90 | pub mod SameDir;
[02:12:36]    |         ^^^^^^^ help: convert the identifier to snake case: `same_dir`
[02:12:36]    |
[02:12:36]    = note: #[warn(non_snake_case)] on by default
[02:12:36] 
[02:12:36] warning: module `SubDir` should have a snake case name
[02:12:36]   --> foo.rs:91:9
[02:12:36] 91 | pub mod SubDir;
[02:12:36] 91 | pub mod SubDir;
[02:12:36]    |         ^^^^^^ help: convert the identifier to snake case: `sub_dir`
[02:12:36] 
[02:12:36] warning: static variable `yy` should have an upper case name
[02:12:36]  --> SubDir/mod.rs:6:8
[02:12:36]   |
[02:12:36] 6 | static yy: usize = 25;
[02:12:36]   |        ^^ help: convert the identifier to upper case: `YY`
[02:12:36] 
[02:12:36] warning: module `SameDir2` should have a snake case name
[02:12:36]   --> foo.rs:94:9
[02:12:36]    |
[02:12:36] 94 | pub mod SameDir2;
[02:12:36]    |         ^^^^^^^^ help: convert the identifier to snake case: `same_dir2`
[02:12:36] 
[02:12:36] warning: trait method `Method` should have a snake case name
[02:12:36]    --> foo.rs:110:8
[02:12:36]     |
[02:12:36] 110 |     fn Method(&self, x: u32) -> u32;
[02:12:36]     |        ^^^^^^ help: convert the identifier to snake case: `method`
[02:12:36] 
[02:12:36] warning: function `matchSomeEnum` should have a snake case name
[02:12:36]    --> foo.rs:200:4
[02:12:36]     |
[02:12:36] 200 | fn matchSomeEnum(val: SomeEnum) {
[02:12:36]     |    ^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_enum`
[02:12:36] 
[02:12:36] warning: function `matchSomeStructEnum` should have a snake case name
[02:12:36]    --> foo.rs:209:4
[02:12:36]     |
[02:12:36] 209 | fn matchSomeStructEnum(se: SomeStructEnum) {
[02:12:36]     |    ^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum`
[02:12:36] 
[02:12:36] warning: the `a:` in this pattern is redundant
[02:12:36]    --> foo.rs:211:36
[02:12:36]     |
[02:12:36] 211 |         SomeStructEnum::EnumStruct{a:a, ..} => println(&a.to_string()),
[02:12:36]     |                                    --^
[02:12:36]     |                                    help: remove this
[02:12:36]     |
[02:12:36]     = note: #[warn(non_shorthand_field_patterns)] on by default
[02:12:36] 
[02:12:36] 
[02:12:36] warning: the `f1:` in this pattern is redundant
[02:12:36]    --> foo.rs:212:37
[02:12:36]     |
[02:12:36] 212 |         SomeStructEnum::EnumStruct2{f1:f1, f2:f_2} => println(&f_2.field1.to_string()),
[02:12:36]     |                                     ---^^
[02:12:36]     |                                     help: remove this
[02:12:36] 
[02:12:36] 
[02:12:36] warning: function `matchSomeStructEnum2` should have a snake case name
[02:12:36]    --> foo.rs:218:4
[02:12:36]     |
[02:12:36] 218 | fn matchSomeStructEnum2(se: SomeStructEnum) {
[02:12:36]     |    ^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_struct_enum2`
[02:12:36] 
[02:12:36] warning: the `f2:` in this pattern is redundant
[02:12:36]    --> foo.rs:222:25
[02:12:36]     |
[02:12:36] 222 |         EnumStruct2{f1, f2: f2} => println(&f1.field1.to_string()),
[02:12:36]     |                         ---^^^
[02:12:36]     |                         help: remove this
[02:12:36] 
[02:12:36] 
[02:12:36] warning: function `matchSomeOtherEnum` should have a snake case name
[02:12:36]    --> foo.rs:228:4
[02:12:36]     |
[02:12:36] 228 | fn matchSomeOtherEnum(val: SomeOtherEnum) {
[02:12:36]     |    ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `match_some_other_enum`
[02:12:36] 
[02:12:36] thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:499:9
[02:12:36] 
[02:12:36] error: internal compiler error: unexpected panic
[02:12:36] 
[02:12:36] note: the compiler unexpectedly panicked. this is a bug.
[02:12:36] note: the compiler unexpectedly panicked. this is a bug.
[02:12:36] 
[02:12:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[02:12:36] 
[02:12:36] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[02:12:36] 
[02:12:36] note: compiler flags: -Z save-analysis
[02:12:36] 
[02:12:36] make[1]: *** [code] Error 101
[02:12:36] ------------------------------------------
[02:12:36] 
[02:12:36] thread '[run-make] run-make-fulldeps/save-analysis' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[02:12:36] 
[02:12:36] 
[02:12:36] ---- [run-make] run-make-fulldeps/save-analysis-rfc2126 stdout ----
[02:12:36] 
[02:12:36] error: make failed
[02:12:36] status: exit code: 2
[02:12:36] command: "make"
[02:12:36] stdout:
[02:12:36] ------------------------------------------
[02:12:36] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/save-analysis-rfc2126'
[02:12:36] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126  krate2.rs
[02:12:36] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126  extern_absolute_paths.rs -Zsave-analysis --edition=2018 \
[02:12:36]  -Z unstable-options --extern krate2
[02:12:36] Makefile:4: recipe for target 'all' failed
[02:12:36] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/save-analysis-rfc2126'
[02:12:36] ------------------------------------------
[02:12:36] stderr:
[02:12:36] ------------------------------------------
[02:12:36] ------------------------------------------
[02:12:36] thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:499:9
[02:12:36] 
[02:12:36] error: internal compiler error: unexpected panic
[02:12:36] 
[02:12:36] note: the compiler unexpectedly panicked. this is a bug.
[02:12:36] note: the compiler unexpectedly panicked. this is a bug.
[02:12:36] 
[02:12:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[02:12:36] 
[02:12:36] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[02:12:36] 
[02:12:36] note: compiler flags: -Z save-analysis -Z unstable-options
[02:12:36] 
[02:12:36] make[1]: *** [all] Error 101
[02:12:36] ------------------------------------------
[02:12:36] 
[02:12:36] thread '[run-make] run-make-fulldeps/save-analysis-rfc2126' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[02:12:36] 
---
[02:12:36] test result: FAILED. 187 passed; 3 failed; 5 ignored; 0 measured; 0 filtered out
[02:12:36] 
[02:12:36] 
[02:12:36] 
[02:12:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:12:36] 
[02:12:36] 
[02:12:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:12:36] Build completed unsuccessfully in 0:41:59
[02:12:36] Build completed unsuccessfully in 0:41:59
[02:12:36] Makefile:48: recipe for target 'check' failed
[02:12:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01a4d85a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 15:07:05 UTC 2019
---
travis_time:end:0247b93b:start=1553008026827689630,finish=1553008026832520372,duration=4830742
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03a3faa0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c4869aa
travis_time:start:0c4869aa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c9dee9d
$ dmesg | grep -i kill
