\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs","byte_start":354,"byte_end":364,"line_start":13,"line_end":13,"column_start":10,"column_end":20,"is_primary":true,"text":[{"text":"#[derive(HashStable)]","highlight_start":10,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs","byte_start":354,"byte_end":364,"line_start":13,"line_end":13,"column_start":10,"column_end":20,"is_primary":false,"text":[{"text":"#[derive(HashStable)]","highlight_start":10,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(HashStable)]","def_site_span":null}}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/27812","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?\n  --> /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:13:10\n   |\nLL | #[derive(HashStable)]\n   |          ^^^^^^^^^^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/27812\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:16:02] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:16:02] {"message":"Some errors occurred: E0601, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0601, E0658.\n"}
[01:16:02] 
[01:16:02] ------------------------------------------
[01:16:02] 
[01:16:02] thread '[ui] ui-fulldeps/hash-stable-is-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:16:02] thread '[ui] ui-fulldeps/hash-stable-is-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:16:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:02] 
[01:16:02] ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
[01:16:02] diff of stderr:
[01:16:02] 
[01:16:02] - error[E0658]: compiler plugins are experimental and possibly buggy (see issue #29597)
[01:16:02] + error[E0658]: compiler plugins are experimental and possibly buggy
[01:16:02] 2   --> $DIR/gated-plugin.rs:3:1
[01:16:02] 3    |
[01:16:02] 4 LL | #![plugin(attr_plugin_test)]
[01:16:02] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:16:02] 6    |
[01:16:02] +    = note: for more information, see https://github.com/rust-lang/rust/issues/29597
[01:16:02] 7    = help: add #![feature(plugin)] to the crate attributes to enable
[01:16:02] 7    = help: add #![feature(plugin)] to the crate attributes to enable
[01:16:02] 8 
[01:16:02] 9 error: aborting due to previous error
[01:16:02] 
[01:16:02] 
[01:16:02] The actual stderr differed from the expected stderr.
[01:16:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/gated-plugin.stderr
[01:16:02] To update references, rerun the tests and pass the `--bless` flag
[01:16:02] To only update this specific test, also pass `--test-args gated-plugin.rs`
[01:16:02] error: 1 errors occurred comparing output.
[01:16:02] status: exit code: 1
[01:16:02] status: exit code: 1
[01:16:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/gated-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-A" "unused"
[01:16:02] ------------------------------------------
[01:16:02] 
[01:16:02] ------------------------------------------
[01:16:02] stderr:
[01:16:02] stderr:
[01:16:02] ------------------------------------------
[01:16:02] {"message":"compiler plugins are experimental and possibly buggy","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n