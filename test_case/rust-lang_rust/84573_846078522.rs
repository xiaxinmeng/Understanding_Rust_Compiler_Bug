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
Searching for toolstate changes between fc81ad22c453776de16acf9938976930cf8c9401 and 7041d821f6c3618f02668c37c310715bef70445f
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
 error: enum with no variants
-  --> $DIR/empty_enum.rs:5:1
+  --> $DIR/empty_enum.rs:4:1
    |
 LL | enum Empty {}
    |
    |
    = note: `-D clippy::empty-enum` implied by `-D warnings`
    = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated
 error: aborting due to previous error
 
 


error: test failed, to rerun pass '--test compile-test'
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/empty_enum.stage-id.stderr
To only update this specific test, also pass `--test-args empty_enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/empty_enum.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/empty_enum.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/empty_enum.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"enum with no variants","code":{"code":"clippy::empty_enum","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_enum.rs","byte_start":95,"byte_end":108,"line_start":4,"line_end":4,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"enum Empty {}","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::empty-enum` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: enum with no variants\n  --> tests/ui/empty_enum.rs:4:1\n   |\nLL | enum Empty {}\n   | ^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::empty-enum` implied by `-D warnings`\n   = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: enum with no variants
  --> $DIR/empty_enum_without_never_type.rs:5:1
   |
LL | enum Empty {}
   |
   |
   = note: `-D clippy::empty-enum` implied by `-D warnings`
   = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args empty_enum_without_never_type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/empty_enum_without_never_type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/empty_enum_without_never_type.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/empty_enum_without_never_type.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"enum with no variants","code":{"code":"clippy::empty_enum","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_enum_without_never_type.rs","byte_start":112,"byte_end":125,"line_start":5,"line_end":5,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"enum Empty {}","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::empty-enum` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: enum with no variants\n  --> tests/ui/empty_enum_without_never_type.rs:5:1\n   |\nLL | enum Empty {}\n   | ^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::empty-enum` implied by `-D warnings`\n   = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
+  --> $DIR/infallible_destructuring_match.rs:2:33
+   |
+LL | #![feature(exhaustive_patterns, never_type)]
+   |
+   |
+   = note: `-D stable-features` implied by `-D warnings`
+
 error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`
    |
    |
 LL | /     let data = match wrapper {
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
-error: aborting due to 3 previous errors
+error: aborting due to 4 previous errors
 
 
---
To only update this specific test, also pass `--test-args infallible_destructuring_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/infallible_destructuring_match.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":47,"byte_end":57,"line_start":2,"line_end":2,"column_start":33,"column_end":43,"is_primary":true,"text":[{"text":"#![feature(exhaustive_patterns, never_type)]","highlight_start":33,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable\n  --> tests/ui/infallible_destructuring_match.rs:2:33\n   |\nLL | #![feature(exhaustive_patterns, never_type)]\n   |                                 ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":517,"byte_end":594,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::infallible-destructuring-match` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":517,"byte_end":594,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let SingleVariantEnum::Variant(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:26:5\n   |\nLL | /     let data = match wrapper {\nLL | |         SingleVariantEnum::Variant(i) => i,\nLL | |     };\n   | |______^ help: try this: `let SingleVariantEnum::Variant(data) = wrapper;`\n   |\n   = note: `-D clippy::infallible-destructuring-match` implied by `-D warnings`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1182,"byte_end":1244,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1182,"byte_end":1244,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let TupleStruct(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:58:5\n   |\nLL | /     let data = match wrapper {\nLL | |         TupleStruct(i) => i,\nLL | |     };\n   | |______^ help: try this: `let TupleStruct(data) = wrapper;`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1765,"byte_end":1818,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1765,"byte_end":1818,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let Ok(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:90:5\n   |\nLL | /     let data = match wrapper {\nLL | |         Ok(i) => i,\nLL | |     };\n   | |______^ help: try this: `let Ok(data) = wrapper;`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this function could have a `#[must_use]` attribute
+  --> $DIR/must_use_candidates.rs:11:1
    |
    |
 LL | pub fn pure(i: u8) -> u8 {
    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`
    |
    = note: `-D clippy::must-use-candidate` implied by `-D warnings`
 
 error: this method could have a `#[must_use]` attribute
+  --> $DIR/must_use_candidates.rs:16:5
    |
    |
 LL |     pub fn inherent_pure(&self) -> u8 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`
 
 error: this function could have a `#[must_use]` attribute
+  --> $DIR/must_use_candidates.rs:47:1
    |
    |
 LL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`
 
 error: this function could have a `#[must_use]` attribute
+  --> $DIR/must_use_candidates.rs:59:1
    |
    |
 LL | pub fn rcd(_x: Rc<u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`
 
 error: this function could have a `#[must_use]` attribute
+  --> $DIR/must_use_candidates.rs:67:1
    |
    |
 LL | pub fn arcd(_x: Arc<u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`
 error: aborting due to 5 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/must_use_candidates.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args must_use_candidates.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/must_use_candidates.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/must_use_candidates.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/must_use_candidates.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":242,"byte_end":266,"line_start":11,"line_end":11,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub fn pure(i: u8) -> u8 {","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::must-use-candidate` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":242,"byte_end":266,"line_start":11,"line_end":11,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub fn pure(i: u8) -> u8 {","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":"#[must_use] pub fn pure(i: u8) -> u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:11:1\n   |\nLL | pub fn pure(i: u8) -> u8 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`\n   |\n   = note: `-D clippy::must-use-candidate` implied by `-D warnings`\n\n"}
{"message":"this method could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":296,"byte_end":329,"line_start":16,"line_end":16,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    pub fn inherent_pure(&self) -> u8 {","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":296,"byte_end":329,"line_start":16,"line_end":16,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    pub fn inherent_pure(&self) -> u8 {","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"#[must_use] pub fn inherent_pure(&self) -> u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this method could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:16:5\n   |\nLL |     pub fn inherent_pure(&self) -> u8 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":768,"byte_end":834,"line_start":47,"line_end":47,"column_start":1,"column_end":67,"is_primary":true,"text":[{"text":"pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {","highlight_start":1,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":768,"byte_end":834,"line_start":47,"line_end":47,"column_start":1,"column_end":67,"is_primary":true,"text":[{"text":"pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {","highlight_start":1,"highlight_end":67}],"label":null,"suggested_replacement":"#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:47:1\n   |\nLL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":989,"byte_end":1020,"line_start":59,"line_end":59,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"pub fn rcd(_x: Rc<u32>) -> bool {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":989,"byte_end":1020,"line_start":59,"line_end":59,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"pub fn rcd(_x: Rc<u32>) -> bool {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":"#[must_use] pub fn rcd(_x: Rc<u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:59:1\n   |\nLL | pub fn rcd(_x: Rc<u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1088,"byte_end":1121,"line_start":67,"line_end":67,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"pub fn arcd(_x: Arc<u32>) -> bool {","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1088,"byte_end":1121,"line_start":67,"line_end":67,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"pub fn arcd(_x: Arc<u32>) -> bool {","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":"#[must_use] pub fn arcd(_x: Arc<u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:67:1\n   |\nLL | pub fn arcd(_x: Arc<u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:22:5
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
    |
    = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:24:5
    |
    |
 LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:28:5
    |
    |
 LL |        x.field.map(|value| {
    |  |_____|
    | ||
    | ||
 LL | ||         do_nothing(value);
 LL | ||         do_nothing(value)
 LL | ||     });
    | ||______^- help: try this: `if let Ok(value) = x.field { ... }`
    | 
 
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:32:5
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:36:5
    |
    |
 LL |     "12".parse::<i32>().map(diverge);
    |     |
    |     |
    |     help: try this: `if let Ok(a) = "12".parse::<i32>() { diverge(a) }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
+  --> $DIR/result_map_unit_fn_unfixable.rs:42:5
    |
    |
 LL |     y.map(do_nothing);
    |     |
    |     |
    |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/result_map_unit_fn_unfixable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args result_map_unit_fn_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/result_map_unit_fn_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/result_map_unit_fn_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/result_map_unit_fn_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":324,"byte_end":385,"line_start":22,"line_end":22,"column_start":5,"column_end":66,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::result-map-unit-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":324,"byte_end":386,"line_start":22,"line_end":22,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:22:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n   |\n   = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":392,"byte_end":466,"line_start":24,"line_end":24,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":392,"byte_end":467,"line_start":24,"line_end":24,"column_start":5,"column_end":80,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":80}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:24:5\n   |\nLL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":603,"byte_end":684,"line_start":28,"line_end":31,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":603,"byte_end":685,"line_start":28,"line_end":31,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:28:5\n   |\nLL |        x.field.map(|value| {\n   |   _____^\n   |  |_____|\n   | ||\nLL | ||         do_nothing(value);\nLL | ||         do_nothing(value)\nLL | ||     });\n   | ||______^- help: try this: `if let Ok(value) = x.field { ... }`\n   | |_______|\n   | \n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":690,"byte_end":752,"line_start":32,"line_end":32,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":690,"byte_end":753,"line_start":32,"line_end":32,"column_start":5,"column_end":68,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":68}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:32:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":934,"byte_end":966,"line_start":36,"line_end":36,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":934,"byte_end":967,"line_start":36,"line_end":36,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:36:5\n   |\nLL |     \"12\".parse::<i32>().map(diverge);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1166,"byte_end":1183,"line_start":42,"line_end":42,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1166,"byte_end":1184,"line_start":42,"line_end":42,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"if let Ok(_y) = y { do_nothing(_y) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:42:5\n   |\nLL |     y.map(do_nothing);\n   |     ^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
