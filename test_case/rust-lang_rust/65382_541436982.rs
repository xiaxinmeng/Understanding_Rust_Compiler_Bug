plain
2019-10-13T17:10:06.8108568Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-13T17:10:06.9097198Z error: Could not document `syntax`.
2019-10-13T17:10:06.9099670Z 
2019-10-13T17:10:06.9101222Z Caused by:
2019-10-13T17:10:06.9107408Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-435dbeebdef092cb.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a6b66cd822ecd2b7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-10998367f7a2cdfc.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f2a014a0db031991.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b3159432e1221100.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-71dcfe9e43d142fc.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-0c2de7c77980d275.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a83e0850fdaf203d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-39457f4bb57354dd.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2acc33d0854bc403.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bb804521efc09831.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-841e94e5e9965c08.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-13T17:10:13.2559711Z [RUSTC-TIMING] syntax test:false 11.438
2019-10-13T17:10:13.2676044Z error: build failed
2019-10-13T17:10:13.2694814Z 
2019-10-13T17:10:13.2695203Z 
2019-10-13T17:10:13.2695203Z 
2019-10-13T17:10:13.2701894Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_lexer" "-p" "syntax" "-p" "rustc_driver" "-p" "rustc_traits" "-p" "rustc_lint" "-p" "rustc_target" "-p" "fmt_macros" "-p" "rustc_incremental" "-p" "rustc_llvm" "-p" "rustc_mir" "-p" "rustc_plugin_impl" "-p" "serialize" "-p" "rustc_errors" "-p" "rustc_privacy" "-p" "syntax_ext" "-p" "rustc_codegen_llvm" "-p" "rustc_macros" "-p" "graphviz" "-p" "rustc_apfloat" "-p" "rustc" "-p" "rustc_fs_util" "-p" "rustc_plugin" "-p" "rustc_codegen_ssa" "-p" "syntax_pos" "-p" "rustc_save_analysis" "-p" "rustc_typeck" "-p" "rustc_index" "-p" "arena" "-p" "rustc_resolve" "-p" "build_helper" "-p" "rustc_metadata" "-p" "rustc_data_structures" "-p" "rustc_passes" "-p" "rustc_codegen_utils" "-p" "rustc_interface"
2019-10-13T17:10:13.2702498Z 
2019-10-13T17:10:13.2702558Z 
2019-10-13T17:10:13.2718699Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-13T17:10:13.2719426Z Build completed unsuccessfully in 1:40:36
2019-10-13T17:10:13.2719426Z Build completed unsuccessfully in 1:40:36
2019-10-13T17:10:13.2783102Z == clock drift check ==
2019-10-13T17:10:13.2809722Z   local time: Sun Oct 13 17:10:13 UTC 2019
2019-10-13T17:10:13.5785122Z   network time: Sun, 13 Oct 2019 17:10:13 GMT
2019-10-13T17:10:13.5785280Z == end clock drift check ==
2019-10-13T17:10:15.2194179Z ##[error]Bash exited with code '1'.
2019-10-13T17:10:15.2248925Z ##[section]Starting: Upload CPU usage statistics
2019-10-13T17:10:15.2262673Z ==============================================================================
2019-10-13T17:10:15.2263026Z Task         : Bash
2019-10-13T17:10:15.2263218Z Description  : Run a Bash script on macOS, Linux, or Windows
