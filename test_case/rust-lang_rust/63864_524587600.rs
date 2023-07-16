plain
2019-08-24T21:32:18.3137209Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T21:32:18.3137272Z 
2019-08-24T21:32:18.3137462Z   git checkout -b <new-branch-name>
2019-08-24T21:32:18.3137497Z 
2019-08-24T21:32:18.3137805Z HEAD is now at 560ba3b2a Auto merge of #63864 - Centril:rollup-7f5jqwp, r=Centril
2019-08-24T21:32:18.3359283Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T21:32:18.3361915Z ==============================================================================
2019-08-24T21:32:18.3361990Z Task         : Bash
2019-08-24T21:32:18.3362068Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T23:16:50.2363933Z [RUSTC-TIMING] parking_lot_core test:false 0.283
2019-08-24T23:16:50.2364221Z     Checking tempfile v3.0.5
2019-08-24T23:16:50.3010303Z [RUSTC-TIMING] rustc_rayon test:false 2.803
2019-08-24T23:16:50.3048334Z  Documenting rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-08-24T23:16:50.4311743Z error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
2019-08-24T23:16:50.4318768Z   --> src/librustc_macros/src/lib.rs:16:1
2019-08-24T23:16:50.4358377Z 16 | #[proc_macro]
2019-08-24T23:16:50.4358735Z    | ^^^^^^^^^^^^^
2019-08-24T23:16:50.4358796Z 
2019-08-24T23:16:50.4358796Z 
2019-08-24T23:16:50.4359145Z error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
2019-08-24T23:16:50.4359798Z   --> src/librustc_macros/src/lib.rs:21:1
2019-08-24T23:16:50.4360403Z 21 | #[proc_macro]
2019-08-24T23:16:50.4360641Z    | ^^^^^^^^^^^^^
2019-08-24T23:16:50.4360683Z 
2019-08-24T23:16:50.4360683Z 
2019-08-24T23:16:50.4360970Z error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
2019-08-24T23:16:50.4361215Z   --> src/librustc_macros/src/lib.rs:26:1
2019-08-24T23:16:50.4361436Z    |
2019-08-24T23:16:50.4361725Z 26 | decl_derive!([HashStable, attributes(stable_hasher)] => hash_stable::hash_stable_derive);
2019-08-24T23:16:50.4362489Z    |
2019-08-24T23:16:50.4362982Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-08-24T23:16:50.4363059Z 
2019-08-24T23:16:50.4421029Z error: Compilation failed, aborting rustdoc
2019-08-24T23:16:50.4421029Z error: Compilation failed, aborting rustdoc
2019-08-24T23:16:50.4424143Z 
2019-08-24T23:16:50.4459509Z error: aborting due to 4 previous errors
2019-08-24T23:16:50.4461729Z 
2019-08-24T23:16:50.4531701Z error: Could not document `rustc_macros`.
2019-08-24T23:16:50.4532316Z 
2019-08-24T23:16:50.4532419Z Caused by:
2019-08-24T23:16:50.4540504Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name rustc_macros src/librustc_macros/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libitertools-05c344111cebd5eb.rmeta --extern proc_macro2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro2-891be68da0739910.rmeta --extern quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libquote-9870a26fd9f55627.rmeta --extern syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsyn-99e800f212c21b8c.rmeta --extern synstructure=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsynstructure-9d2b40a17f25b016.rmeta` (exit code: 1)
2019-08-24T23:16:50.5153573Z [RUSTC-TIMING] tempfile test:false 0.274
2019-08-24T23:16:50.5232706Z error: build failed
2019-08-24T23:16:50.5246417Z 
2019-08-24T23:16:50.5246752Z 
2019-08-24T23:16:50.5246752Z 
2019-08-24T23:16:50.5252219Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "fmt_macros" "-p" "rustc_privacy" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "rustc_data_structures" "-p" "rustc_apfloat" "-p" "build_helper" "-p" "rustc_ast_borrowck" "-p" "rustc_codegen_llvm" "-p" "rustc_codegen_utils" "-p" "rustc_codegen_ssa" "-p" "rustc_target" "-p" "syntax_ext" "-p" "rustc_interface" "-p" "rustc_lint" "-p" "rustc_lexer" "-p" "rustc_errors" "-p" "rustc_passes" "-p" "syntax_pos" "-p" "rustc_traits" "-p" "graphviz" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_incremental" "-p" "syntax" "-p" "rustc_metadata" "-p" "rustc_typeck" "-p" "rustc" "-p" "serialize" "-p" "rustc_llvm" "-p" "arena" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_resolve" "-p" "rustc_plugin"
2019-08-24T23:16:50.5252799Z 
2019-08-24T23:16:50.5252841Z 
2019-08-24T23:16:50.5265674Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-08-24T23:16:50.5266071Z Build completed unsuccessfully in 1:37:32
2019-08-24T23:16:50.5266071Z Build completed unsuccessfully in 1:37:32
2019-08-24T23:16:50.5318770Z == clock drift check ==
2019-08-24T23:16:50.5341124Z   local time: Sat Aug 24 23:16:50 UTC 2019
2019-08-24T23:16:51.0089911Z   network time: Sat, 24 Aug 2019 23:16:51 GMT
2019-08-24T23:16:51.0090084Z == end clock drift check ==
2019-08-24T23:16:52.6358356Z ##[error]Bash exited with code '1'.
2019-08-24T23:16:52.6405095Z ##[section]Starting: Upload CPU usage statistics
2019-08-24T23:16:52.6414048Z ==============================================================================
2019-08-24T23:16:52.6414142Z Task         : Bash
2019-08-24T23:16:52.6414229Z Description  : Run a Bash script on macOS, Linux, or Windows
