plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between cdbe2888979bb8797b05f0d58a6f6e60753983d2 and 40a78ab8c6ed100cf98a8d0f08b08ccb0a4973b9
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
---
   Compiling tester v0.9.0
warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
---

warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted

warning: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
---
diff of stderr:

-error: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
-  --> $DIR/infallible_destructuring_match.rs:2:33
-   |
-LL | #![feature(exhaustive_patterns, never_type)]
-   |                                 ^^^^^^^^^^
-   |
-   = note: `-D stable-features` implied by `-D warnings`
-
 error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`
    |
    |
 LL | /     let data = match wrapper {
error: test failed, to rerun pass '--test compile-test'
 LL | |         SingleVariantEnum::Variant(i) => i,
 LL | |     };
    | |______^ help: try this: `let SingleVariantEnum::Variant(data) = wrapper;`
    |
    = note: `-D clippy::infallible-destructuring-match` implied by `-D warnings`
 
 error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`
    |
    |
 LL | /     let data = match wrapper {
 LL | |         TupleStruct(i) => i,
 LL | |     };
    | |______^ help: try this: `let TupleStruct(data) = wrapper;`
 
 error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`
    |
    |
 LL | /     let data = match wrapper {
 LL | |         Ok(i) => i,
 LL | |     };
    | |______^ help: try this: `let Ok(data) = wrapper;`
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id.stderr
diff of fixed:

 // run-rustfix
-#![feature(exhaustive_patterns, never_type)]
+#![feature(exhaustive_patterns)]
 #![allow(dead_code, unreachable_code, unused_variables)]
 #![allow(clippy::let_and_return)]
 
 enum SingleVariantEnum {
     Variant(i32),
 
 struct TupleStruct(i32);
 
 
 enum EmptyEnum {}
 macro_rules! match_enum {
 macro_rules! match_enum {
     ($param:expr) => {
         let data = match $param {
             SingleVariantEnum::Variant(i) => i,
     };
 }
 
 
 fn infallible_destructuring_match_enum() {
     let wrapper = SingleVariantEnum::Variant(0);
     // This should lint!
     // This should lint!
     let SingleVariantEnum::Variant(data) = wrapper;
     // This shouldn't (inside macro)
     // This shouldn't (inside macro)
     match_enum!(wrapper);
     // This shouldn't!
     let data = match wrapper {
     let data = match wrapper {
         SingleVariantEnum::Variant(_) => -1,
 
 
     // Neither should this!
     let data = match wrapper {
         SingleVariantEnum::Variant(i) => -1,
 
 
     let SingleVariantEnum::Variant(data) = wrapper;
 
 macro_rules! match_struct {
 macro_rules! match_struct {
     ($param:expr) => {
         let data = match $param {
             TupleStruct(i) => i,
     };
 }
 
 fn infallible_destructuring_match_struct() {
 fn infallible_destructuring_match_struct() {
     let wrapper = TupleStruct(0);
 
     // This should lint!
     let TupleStruct(data) = wrapper;
     // This shouldn't (inside macro)
     // This shouldn't (inside macro)
     match_struct!(wrapper);
     // This shouldn't!
     let data = match wrapper {
     let data = match wrapper {
         TupleStruct(_) => -1,
 
 
     // Neither should this!
     let data = match wrapper {
         TupleStruct(i) => -1,
 
 
     let TupleStruct(data) = wrapper;
 
 
 macro_rules! match_never_enum {
     ($param:expr) => {
         let data = match $param {
             Ok(i) => i,
     };
 }
 
 
 fn never_enum() {
     let wrapper: Result<i32, !> = Ok(23);
     // This should lint!
     // This should lint!
     let Ok(data) = wrapper;
     // This shouldn't (inside macro)
     // This shouldn't (inside macro)
     match_never_enum!(wrapper);
     // This shouldn't!
     let data = match wrapper {
     let data = match wrapper {
         Ok(_) => -1,
 
 
     // Neither should this!
     let data = match wrapper {
         Ok(i) => -1,
 
 
     let Ok(data) = wrapper;
 
 
 impl EmptyEnum {
     fn match_on(&self) -> ! {
         // The lint shouldn't pick this up, as `let` won't work here!
         let data = match *self {};
     }
 }
 
 fn main() {}
 fn main() {}
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id.fixed
To only update this specific test, also pass `--test-args infallible_destructuring_match.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/infallible_destructuring_match.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-465fb9c5413ef78c.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-dcbfa95d21c9d3c4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-f75d20e88ff41cda.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-6083e1acaa6e9ab6.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-07a2fb86e1f5b8b7.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":505,"byte_end":582,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::infallible-destructuring-match` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":505,"byte_end":582,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let SingleVariantEnum::Variant(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:26:5\n   |\nLL | /     let data = match wrapper {\nLL | |         SingleVariantEnum::Variant(i) => i,\nLL | |     };\n   | |______^ help: try this: `let SingleVariantEnum::Variant(data) = wrapper;`\n   |\n   = note: `-D clippy::infallible-destructuring-match` implied by `-D warnings`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1170,"byte_end":1232,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1170,"byte_end":1232,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let TupleStruct(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:58:5\n   |\nLL | /     let data = match wrapper {\nLL | |         TupleStruct(i) => i,\nLL | |     };\n   | |______^ help: try this: `let TupleStruct(data) = wrapper;`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1753,"byte_end":1806,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1753,"byte_end":1806,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let Ok(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:90:5\n   |\nLL | /     let data = match wrapper {\nLL | |         Ok(i) => i,\nLL | |     };\n   | |______^ help: try this: `let Ok(data) = wrapper;`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
