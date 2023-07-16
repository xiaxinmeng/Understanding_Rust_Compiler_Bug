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
Searching for toolstate changes between 456a03227e3c81a51631f87ec80cac301e5fa6d7 and a48b123b7eb5670f78bb3d986620bca0d5ec19d1
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:25:17
    |
 LL |     for<'a> Dst<A + 'a>: Sized,
    |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`
    |
    = note: `-D bare-trait-objects` implied by `-D warnings`
-   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
 
 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:27:16
    |
    |
 LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
    |                ^ help: use `dyn`: `dyn A`
-   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
-   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
 
 error: trait objects without an explicit `dyn` are deprecated
   --> $DIR/ice-3969.rs:27:57
    |
    |
 LL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);
    |                                                         ^ help: use `dyn`: `dyn A`
-   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
-   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
+   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
 
error: test failed, to rerun pass '--test compile-test'
 error: aborting due to 3 previous errors
 
---
To only update this specific test, also pass `--test-args crashes/ice-3969.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-3969.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-3969.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f9a8d786580456ff.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-07a2fb86e1f5b8b7.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-cca86affe1c40b3a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-f75d20e88ff41cda.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-6083e1acaa6e9ab6.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-3969.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":590,"byte_end":596,"line_start":25,"line_end":25,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    for<'a> Dst<A + 'a>: Sized,","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"dyn A + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:25:17\n   |\nLL |     for<'a> Dst<A + 'a>: Sized,\n   |                 ^^^^^^ help: use `dyn`: `dyn A + 'a`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":623,"byte_end":624,"line_start":27,"line_end":27,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:16\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                ^ help: use `dyn`: `dyn A`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>\n\n"}
{"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-3969.rs","byte_start":664,"byte_end":665,"line_start":27,"line_end":27,"column_start":57,"column_end":58,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":57,"highlight_end":58}],"label":null,"suggested_replacement":"dyn A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/crashes/ice-3969.rs:27:57\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                                                         ^ help: use `dyn`: `dyn A`\n   |\n   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!\n   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
