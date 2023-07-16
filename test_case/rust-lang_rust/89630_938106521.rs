plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0157cc977fd71297ce73e2f249321f5ba2555d42 and 162044ae50c2c73fbf4fe67f36ae32a4fdd49cf2
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
warning: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: `miri` (lib) generated 1 warning
---
   Compiling rustc_version v0.4.0
warning: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
 --> src/tools/miri/src/lib.rs:4:12
  |
4 | #![feature(never_type)]
  |
  = note: `#[warn(stable_features)]` on by default

warning: `miri` (lib test) generated 1 warning (1 duplicate)
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestsa0ogQ/available-concurrency.stage-id.stderr
To only update this specific test, also pass `--test-args available-concurrency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/available-concurrency.rs" "-L" "/tmp/compiletestsa0ogQ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestsa0ogQ/available-concurrency.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestsa0ogQ/available-concurrency.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

+error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
+  --> $DIR/empty_enum.rs:4:12
+   |
+LL | #![feature(never_type)]
error: test failed, to rerun pass '--test compile-test'
+   |
+   |
+   = note: `-D stable-features` implied by `-D warnings`
 error: enum with no variants
   --> $DIR/empty_enum.rs:5:1
    |
    |
 LL | enum Empty {}
    |
    |
    = note: `-D clippy::empty-enum` implied by `-D warnings`
    = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args empty_enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/empty_enum.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/empty_enum.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/empty_enum.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_enum.rs","byte_start":106,"byte_end":116,"line_start":4,"line_end":4,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(never_type)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable\n  --> tests/ui/empty_enum.rs:4:12\n   |\nLL | #![feature(never_type)]\n   |            ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"enum with no variants","code":{"code":"clippy::empty_enum","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_enum.rs","byte_start":119,"byte_end":132,"line_start":5,"line_end":5,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"enum Empty {}","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::empty-enum` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: enum with no variants\n  --> tests/ui/empty_enum.rs:5:1\n   |\nLL | enum Empty {}\n   | ^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::empty-enum` implied by `-D warnings`\n   = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated\n\n"}

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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/empty_enum_without_never_type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/empty_enum_without_never_type.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/empty_enum_without_never_type.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"enum with no variants","code":{"code":"clippy::empty_enum","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_enum_without_never_type.rs","byte_start":112,"byte_end":125,"line_start":5,"line_end":5,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"enum Empty {}","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::empty-enum` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: enum with no variants\n  --> tests/ui/empty_enum_without_never_type.rs:5:1\n   |\nLL | enum Empty {}\n   | ^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::empty-enum` implied by `-D warnings`\n   = help: consider using the uninhabited type `!` (never type) or a wrapper around it to introduce a type which can't be instantiated\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/infallible_destructuring_match.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/infallible_destructuring_match.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/infallible_destructuring_match.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":47,"byte_end":57,"line_start":2,"line_end":2,"column_start":33,"column_end":43,"is_primary":true,"text":[{"text":"#![feature(exhaustive_patterns, never_type)]","highlight_start":33,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable\n  --> tests/ui/infallible_destructuring_match.rs:2:33\n   |\nLL | #![feature(exhaustive_patterns, never_type)]\n   |                                 ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":517,"byte_end":594,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::infallible-destructuring-match` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":517,"byte_end":594,"line_start":26,"line_end":28,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        SingleVariantEnum::Variant(i) => i,","highlight_start":1,"highlight_end":44},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let SingleVariantEnum::Variant(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:26:5\n   |\nLL | /     let data = match wrapper {\nLL | |         SingleVariantEnum::Variant(i) => i,\nLL | |     };\n   | |______^ help: try this: `let SingleVariantEnum::Variant(data) = wrapper;`\n   |\n   = note: `-D clippy::infallible-destructuring-match` implied by `-D warnings`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1182,"byte_end":1244,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1182,"byte_end":1244,"line_start":58,"line_end":60,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        TupleStruct(i) => i,","highlight_start":1,"highlight_end":29},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let TupleStruct(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:58:5\n   |\nLL | /     let data = match wrapper {\nLL | |         TupleStruct(i) => i,\nLL | |     };\n   | |______^ help: try this: `let TupleStruct(data) = wrapper;`\n\n"}
{"message":"you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`","code":{"code":"clippy::infallible_destructuring_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1765,"byte_end":1818,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1765,"byte_end":1818,"line_start":90,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let data = match wrapper {","highlight_start":5,"highlight_end":31},{"text":"        Ok(i) => i,","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"let Ok(data) = wrapper;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` to destructure a single infallible pattern. Consider using `let`\n  --> tests/ui/infallible_destructuring_match.rs:90:5\n   |\nLL | /     let data = match wrapper {\nLL | |         Ok(i) => i,\nLL | |     };\n   | |______^ help: try this: `let Ok(data) = wrapper;`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
+  --> $DIR/must_use_candidates.rs:2:12
+   |
+LL | #![feature(never_type)]
+   |
+   |
+   = note: `-D stable-features` implied by `-D warnings`
+
 error: this function could have a `#[must_use]` attribute
    |
    |
 LL | pub fn pure(i: u8) -> u8 {
    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`
    |
    = note: `-D clippy::must-use-candidate` implied by `-D warnings`
 
 error: this method could have a `#[must_use]` attribute
    |
    |
 LL |     pub fn inherent_pure(&self) -> u8 {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`
 
 error: this function could have a `#[must_use]` attribute
    |
    |
 LL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`
 
 error: this function could have a `#[must_use]` attribute
    |
    |
 LL | pub fn rcd(_x: Rc<u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`
 
 error: this function could have a `#[must_use]` attribute
    |
    |
 LL | pub fn arcd(_x: Arc<u32>) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`
-error: aborting due to 5 previous errors
+error: aborting due to 6 previous errors
 
 
---
To only update this specific test, also pass `--test-args must_use_candidates.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/must_use_candidates.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/must_use_candidates.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/must_use_candidates.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":26,"byte_end":36,"line_start":2,"line_end":2,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(never_type)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable\n  --> tests/ui/must_use_candidates.rs:2:12\n   |\nLL | #![feature(never_type)]\n   |            ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":266,"byte_end":290,"line_start":12,"line_end":12,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub fn pure(i: u8) -> u8 {","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::must-use-candidate` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":266,"byte_end":290,"line_start":12,"line_end":12,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub fn pure(i: u8) -> u8 {","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":"#[must_use] pub fn pure(i: u8) -> u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:12:1\n   |\nLL | pub fn pure(i: u8) -> u8 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`\n   |\n   = note: `-D clippy::must-use-candidate` implied by `-D warnings`\n\n"}
{"message":"this method could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":320,"byte_end":353,"line_start":17,"line_end":17,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    pub fn inherent_pure(&self) -> u8 {","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":320,"byte_end":353,"line_start":17,"line_end":17,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    pub fn inherent_pure(&self) -> u8 {","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"#[must_use] pub fn inherent_pure(&self) -> u8","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this method could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:17:5\n   |\nLL |     pub fn inherent_pure(&self) -> u8 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":792,"byte_end":858,"line_start":48,"line_end":48,"column_start":1,"column_end":67,"is_primary":true,"text":[{"text":"pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {","highlight_start":1,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":792,"byte_end":858,"line_start":48,"line_end":48,"column_start":1,"column_end":67,"is_primary":true,"text":[{"text":"pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {","highlight_start":1,"highlight_end":67}],"label":null,"suggested_replacement":"#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:48:1\n   |\nLL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1013,"byte_end":1044,"line_start":60,"line_end":60,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"pub fn rcd(_x: Rc<u32>) -> bool {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1013,"byte_end":1044,"line_start":60,"line_end":60,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"pub fn rcd(_x: Rc<u32>) -> bool {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":"#[must_use] pub fn rcd(_x: Rc<u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:60:1\n   |\nLL | pub fn rcd(_x: Rc<u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`\n\n"}
{"message":"this function could have a `#[must_use]` attribute","code":{"code":"clippy::must_use_candidate","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1112,"byte_end":1145,"line_start":68,"line_end":68,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"pub fn arcd(_x: Arc<u32>) -> bool {","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add the attribute","code":null,"level":"help","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":1112,"byte_end":1145,"line_start":68,"line_end":68,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"pub fn arcd(_x: Arc<u32>) -> bool {","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":"#[must_use] pub fn arcd(_x: Arc<u32>) -> bool","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this function could have a `#[must_use]` attribute\n  --> tests/ui/must_use_candidates.rs:68:1\n   |\nLL | pub fn arcd(_x: Arc<u32>) -> bool {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
+  --> $DIR/result_map_unit_fn_unfixable.rs:2:12
+   |
+LL | #![feature(never_type)]
+   |
+   |
+   = note: `-D stable-features` implied by `-D warnings`
+
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
    |
    = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
    |
    |
 LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`
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
    |
    |
 LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
    |     |
    |     |
    |     help: try this: `if let Ok(value) = x.field { ... }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
    |
    |
 LL |     "12".parse::<i32>().map(diverge);
    |     |
    |     |
    |     help: try this: `if let Ok(a) = "12".parse::<i32>() { diverge(a) }`
 
 error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`
    |
    |
 LL |     y.map(do_nothing);
    |     |
    |     |
    |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`
-error: aborting due to 6 previous errors
+error: aborting due to 7 previous errors
 
 
---
To only update this specific test, also pass `--test-args result_map_unit_fn_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/result_map_unit_fn_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/result_map_unit_fn_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/result_map_unit_fn_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":48,"byte_end":58,"line_start":2,"line_end":2,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(never_type)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable\n  --> tests/ui/result_map_unit_fn_unfixable.rs:2:12\n   |\nLL | #![feature(never_type)]\n   |            ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":348,"byte_end":409,"line_start":23,"line_end":23,"column_start":5,"column_end":66,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::result-map-unit-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":348,"byte_end":410,"line_start":23,"line_end":23,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:23:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n   |\n   = note: `-D clippy::result-map-unit-fn` implied by `-D warnings`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":416,"byte_end":490,"line_start":25,"line_end":25,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":416,"byte_end":491,"line_start":25,"line_end":25,"column_start":5,"column_end":80,"is_primary":true,"text":[{"text":"    x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });","highlight_start":5,"highlight_end":80}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:25:5\n   |\nLL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":627,"byte_end":708,"line_start":29,"line_end":32,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":627,"byte_end":709,"line_start":29,"line_end":32,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    x.field.map(|value| {","highlight_start":5,"highlight_end":26},{"text":"        do_nothing(value);","highlight_start":1,"highlight_end":27},{"text":"        do_nothing(value)","highlight_start":1,"highlight_end":26},{"text":"    });","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:29:5\n   |\nLL |        x.field.map(|value| {\n   |   _____^\n   |  |_____|\n   | ||\nLL | ||         do_nothing(value);\nLL | ||         do_nothing(value)\nLL | ||     });\n   | ||______^- help: try this: `if let Ok(value) = x.field { ... }`\n   | |_______|\n   | \n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":714,"byte_end":776,"line_start":33,"line_end":33,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":714,"byte_end":777,"line_start":33,"line_end":33,"column_start":5,"column_end":68,"is_primary":true,"text":[{"text":"    x.field.map(|value| { do_nothing(value); do_nothing(value); });","highlight_start":5,"highlight_end":68}],"label":null,"suggested_replacement":"if let Ok(value) = x.field { ... }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a closure that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:33:5\n   |\nLL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(value) = x.field { ... }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":958,"byte_end":990,"line_start":37,"line_end":37,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":958,"byte_end":991,"line_start":37,"line_end":37,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    \"12\".parse::<i32>().map(diverge);","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:37:5\n   |\nLL |     \"12\".parse::<i32>().map(diverge);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(a) = \"12\".parse::<i32>() { diverge(a) }`\n\n"}
{"message":"called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`","code":{"code":"clippy::result_map_unit_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1190,"byte_end":1207,"line_start":43,"line_end":43,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":1190,"byte_end":1208,"line_start":43,"line_end":43,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    y.map(do_nothing);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"if let Ok(_y) = y { do_nothing(_y) }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `map(f)` on an `Result` value where `f` is a function that returns the unit type `()`\n  --> tests/ui/result_map_unit_fn_unfixable.rs:43:5\n   |\nLL |     y.map(do_nothing);\n   |     ^^^^^^^^^^^^^^^^^-\n   |     |\n   |     help: try this: `if let Ok(_y) = y { do_nothing(_y) }`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
