plain
2019-10-11T08:38:24.0983109Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T08:38:24.2072271Z error: Could not document `syntax`.
2019-10-11T08:38:24.2072419Z 
2019-10-11T08:38:24.2072497Z Caused by:
2019-10-11T08:38:24.2076432Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-0907ce1c76de7bf5.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-5a550b28fc9035d6.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7cc4409fc75c0a04.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-19a856aa96a30dd3.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-0b065de9b0a5f73b.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-a9aafc7d6489d848.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-2d423541d10003ba.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6a34b431f77dcd16.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-c6e4a4ecb321ee61.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-7d6801cddba65ca4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-3f452c2acccdb134.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bcca3f23a6477c76.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-11T08:38:32.3087844Z [RUSTC-TIMING] syntax test:false 12.002
2019-10-11T08:38:32.3208376Z error: build failed
2019-10-11T08:38:32.3227481Z 
2019-10-11T08:38:32.3227891Z 
2019-10-11T08:38:32.3227891Z 
2019-10-11T08:38:32.3231420Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_errors" "-p" "rustc_lint" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "fmt_macros" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_lexer" "-p" "rustc_plugin" "-p" "rustc_target" "-p" "rustc_codegen_llvm" "-p" "rustc_mir" "-p" "rustc_resolve" "-p" "serialize" "-p" "rustc_codegen_utils" "-p" "rustc_privacy" "-p" "graphviz" "-p" "arena" "-p" "rustc_metadata" "-p" "syntax" "-p" "rustc_macros" "-p" "rustc_save_analysis" "-p" "build_helper" "-p" "syntax_pos" "-p" "rustc_apfloat" "-p" "syntax_ext" "-p" "rustc_driver" "-p" "rustc_codegen_ssa" "-p" "rustc_plugin_impl" "-p" "rustc_passes" "-p" "rustc_interface"
2019-10-11T08:38:32.3232438Z 
2019-10-11T08:38:32.3232598Z 
2019-10-11T08:38:32.3248134Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-11T08:38:32.3248644Z Build completed unsuccessfully in 1:31:12
2019-10-11T08:38:32.3248644Z Build completed unsuccessfully in 1:31:12
2019-10-11T08:38:32.3316811Z == clock drift check ==
2019-10-11T08:38:32.3341815Z   local time: Fri Oct 11 08:38:32 UTC 2019
2019-10-11T08:38:32.6154469Z   network time: Fri, 11 Oct 2019 08:38:32 GMT
2019-10-11T08:38:32.6154728Z == end clock drift check ==
2019-10-11T08:38:34.2118530Z ##[error]Bash exited with code '1'.
2019-10-11T08:38:34.2178409Z ##[section]Starting: Upload CPU usage statistics
2019-10-11T08:38:34.2197143Z ==============================================================================
2019-10-11T08:38:34.2197243Z Task         : Bash
2019-10-11T08:38:34.2197332Z Description  : Run a Bash script on macOS, Linux, or Windows
