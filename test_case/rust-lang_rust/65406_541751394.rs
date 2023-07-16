plain
2019-10-14T15:32:51.3024642Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-14T15:32:51.4016337Z error: Could not document `syntax`.
2019-10-14T15:32:51.4021962Z 
2019-10-14T15:32:51.4022135Z Caused by:
2019-10-14T15:32:51.4035850Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-73fb4bb99d847e64.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-6f84ad327c3d363c.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f0ab1895b242ebd9.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b6bb1a98a903d0a.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-8170d64ca7802537.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-6730e724184454d8.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-76543b2e8774ccac.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-3839a06c80e7191c.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-e4f8f14b9619a15f.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-21b4f38cdbc8101b.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-5c82efb247ec812a.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-dffd278961db050f.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-14T15:32:58.2573095Z [RUSTC-TIMING] syntax test:false 10.364
2019-10-14T15:32:58.2670955Z error: build failed
2019-10-14T15:32:58.2683286Z 
2019-10-14T15:32:58.2683829Z 
2019-10-14T15:32:58.2683829Z 
2019-10-14T15:32:58.2690546Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "graphviz" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_metadata" "-p" "rustc" "-p" "arena" "-p" "fmt_macros" "-p" "rustc_driver" "-p" "rustc_llvm" "-p" "rustc_plugin" "-p" "rustc_typeck" "-p" "rustc_mir" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_apfloat" "-p" "syntax" "-p" "rustc_index" "-p" "rustc_codegen_utils" "-p" "serialize" "-p" "rustc_codegen_ssa" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "syntax_pos" "-p" "rustc_privacy" "-p" "build_helper" "-p" "rustc_resolve" "-p" "syntax_ext" "-p" "rustc_macros"
2019-10-14T15:32:58.2692516Z 
2019-10-14T15:32:58.2692577Z 
2019-10-14T15:32:58.2703996Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-14T15:32:58.2704207Z Build completed unsuccessfully in 1:29:22
2019-10-14T15:32:58.2704207Z Build completed unsuccessfully in 1:29:22
2019-10-14T15:32:58.2762404Z == clock drift check ==
2019-10-14T15:32:58.2786758Z   local time: Mon Oct 14 15:32:58 UTC 2019
2019-10-14T15:32:58.6608050Z   network time: Mon, 14 Oct 2019 15:32:58 GMT
2019-10-14T15:32:58.6611673Z == end clock drift check ==
2019-10-14T15:33:00.1780200Z ##[error]Bash exited with code '1'.
2019-10-14T15:33:00.1829779Z ##[section]Starting: Upload CPU usage statistics
2019-10-14T15:33:00.1861892Z ==============================================================================
2019-10-14T15:33:00.1862167Z Task         : Bash
2019-10-14T15:33:00.1862413Z Description  : Run a Bash script on macOS, Linux, or Windows
