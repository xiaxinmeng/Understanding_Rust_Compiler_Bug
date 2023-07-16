\n\n`#[inline]` hints the compiler whether or not to attempt to inline a method or\nfunction. By default, the compiler does a pretty good job of figuring this out\nitself, but if you feel the need for annotations, `#[inline(always)]` and\n`#[inline(never)]` can override or force the compiler's decision.\n\nIf you wish to apply this attribute to all methods in an impl, manually annotate\neach method; it is not possible to annotate the entire impl with an `#[inline]`\nattribute.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs","byte_start":1299,"byte_end":1309,"line_start":35,"line_end":35,"column_start":24,"column_end":34,"is_primary":false,"text":[{"text":"    #[inline = \"2100\"] impl S { }","highlight_start":24,"highlight_end":34}],"label":"not a function or closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs","byte_start":1280,"byte_end":1298,"line_start":35,"line_end":35,"column_start":5,"column_end":23,"is_primary":true,"text":[{"label":"not a function or closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs","byte_start":952,"byte_end":969,"line_start":24,"line_end":24,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"    mod inner { #![inline=\"2100\"] }","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0518]: attribute should be applied to function or closure\n  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs:24:17\n   |\nLL |     mod inner { #![inline=\"2100\"] }\n   |     ------------^^^^^^^^^^^^^^^^^-- not a function or closure\n\n"}
[00:50:19] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:50:19] {"message":"For more information about this error, try `rustc --explain E0518`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0518`.\n"}
[00:50:19] ------------------------------------------
[00:50:19] 
[00:50:19] thread '[ui] ui/feature-gate/issue-43106-gating-of-inline.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:19] 
[00:50:19] ---- [ui] ui/issue-12511.rs stdout ----
[00:50:19] diff of stderr:
[00:50:19] 
[00:50:19] 10 LL | trait t2 : t1 {
[00:50:19] 11    | ^^^^^^^^^^^^^
[00:50:19] 12    = note: ...which again requires computing the supertraits of `t1`, completing the cycle
[00:50:19] + note: cycle used when processing ``
[00:50:19] +   --> $DIR/issue-12511.rs:11:1
[00:50:19] +    |
[00:50:19] + LL | trait t1 : t2 {
[00:50:19] 13 
[00:50:19] 14 error: aborting due to previous error
[00:50:19] 15 
[00:50:19] 
[00:50:19] 
[00:50:19] 
[00:50:19] The actual stderr differed from the expected stderr.
[00:50:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511/issue-12511.stderr
[00:50:19] To update references, rerun the tests and pass the `--bless` flag
[00:50:19] To only update this specific test, also pass `--test-args issue-12511.rs`
[00:50:19] error: 1 errors occurred comparing output.
[00:50:19] status: exit code: 101
[00:50:19] status: exit code: 101
[00:50:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-12511.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511/auxiliary" "-A" "unused"
[00:50:19] ------------------------------------------
[00:50:19] 
[00:50:19] ------------------------------------------
[00:50:19] stderr:
[00:50:19] stderr:
[00:50:19] ------------------------------------------
[00:50:19] {"message":"cycle detected when computing the supertraits of `:20>`, completing the cycle
[00:50:19] + note: cycle used when processing ``
[00:50:19] +   --> $DIR/issue-23305.rs:11:1
[00:50:19] +    |
[00:50:19] + LL | pub trait ToNbt<T> {
[00:50:19] 8 
[00:50:19] 9 error: aborting due to previous error
[00:50:19] 10 
[00:50:19] 
[00:50:19] 
[00:50:19] 
[00:50:19] The actual stderr differed from the expected stderr.
[00:50:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/issue-23305.stderr
[00:50:19] To update references, rerun the tests and pass the `--bless` flag
[00:50:19] To only update this specific test, also pass `--test-args resolve/issue-23305.rs`
[00:50:19] error: 1 errors occurred comparing output.
[00:50:19] status: exit code: 101
[00:50:19] status: exit code: 101
[00:50:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-23305.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/auxiliary" "-A" "unused"
[00:50:19] ------------------------------------------
[00:50:19] 
[00:50:19] ------------------------------------------
[00:50:19] stderr:
[00:50:19] stderr:
[00:50:19] ------------------------------------------
[00:50:19] {"message":"cycle detected when processing `<impl at /checkout/src/test/ui/resolve/issue-23305.note: ...which again requires processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/resolve/issue-23305.rs:11:1\n   |\nLL | pub trait ToNbt<T> {\n   | ^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:19] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:50:19] ------------------------------------------
[00:50:19] 
[00:50:19] thread '[ui] ui/resolve/issue-23305.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:19] 
[00:50:19] 
[00:50:19] ---- [ui] ui/target-feature-wrong.rs stdout ----
[00:50:19] diff of stderr:
[00:50:19] 
[00:50:19] 28 LL | #[target_feature(enable = "sse2")]
[00:50:19] 30 
[00:50:19] 30 
[00:50:19] + error: cannot use #[inline(always)] with #[target_feature]
[00:50:19] +   --> $DIR/target-feature-wrong.rs:41:1
[00:50:19] +    |
[00:50:19] + LL | #[inline(always)]
[00:50:19] + 
[00:50:19] 31 error: attribute should be applied to a function
[00:50:19] 32   --> $DIR/target-feature-wrong.rs:37:1
[00:50:19] 33    |
[00:50:19] 33    |
[00:50:19] 
[00:50:19] 36 LL | //~^ ERROR: should be applied to a function
[00:50:19] 37 LL | mod another {}
[00:50:19] 38    | -------------- not a fu form #[target_feature(..)]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":648,"byte_end":675,"line_start":23,"line_end":23,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"#[target_feature = \"+sse2\"]","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature] attribute must be of the form #[target_feature(..)]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:23:1\n   |\nLL | #[target_feature = \"+sse2\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"the feature named `foo` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":725,"byte_end":739,"line_start":25,"line_end":25,"column_start":18,"column_end":32,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"foo\")]","highlight_start":18,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the feature named `foo` is not valid for this target\n  --> /checkout/src/test/ui/target-feature-wrong.rs:25:18\n   |\nLL | #[target_feature(enable = \"foo\")]\n   |                  ^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":797,"byte_end":800,"line_start":27,"line_end":27,"column_start":18,"cosion":null}],"children":[],"rendered":"error: #[target_feature(..)] can only be applied to `unsafe` function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:33:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"cannot use #[inline(always)] with #[target_feature]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1122,"byte_end":1139,"line_start":41,"line_end":41,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"#[inline(always)]","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot use #[inline(always)] with #[target_feature]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:41:1\n   |\nLL | #[inline(always)]\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"attribute should be applied to a function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1106,"byte_end":1120,"line_start":39,"line_end":39,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"mod another {}","highlight_start":1,"highlight_end":15}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1027,"byte_end":1061,"line_start":37,"line_end":37,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":3rs:19:1\n   |\nLL | #[wasm_custom_section = \"foo\"] //~ ERROR: only allowed on consts\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"only allowed on consts","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/wasm-custom-section/not-const.rs","byte_start":749,"byte_end":779,"line_start":22,"line_end":22,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"#[wasm_custom_section = \"foo\"] //~ ERROR: only allowed on consts","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: only allowed on consts\n  --> /checkout/src/test/ui/wasm-custom-section/not-const.rs:22:1\n   |\nLL | #[wasm_custom_section = \"foo\"] //~ ERROR: only allowed on consts\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"only allowed on consts","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/wasm-custom-section/not-const.rs","byte_start":900,"byte_end":930,"line_start":28,"line_end":28,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"#[wasm_custom_section = \"foo\"] //~ ERROR: only allowed on consts","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: only allowed on consts\n  --> /checkout/src/test/ui/wasm-custom-section/not-const.rs:28:1\n   |\nLL | #[wasm_custom_section = \"foo\"] //~ ERROR: only allowed on consts\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:19] {"message":"only allowed on consts","code":null/obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103928 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1voluf6r1-1fnqm4h-no75lmxs5rt9
102904 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
102900 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
99520 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
91736 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
