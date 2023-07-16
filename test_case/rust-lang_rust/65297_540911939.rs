plain
2019-10-11T05:22:30.0544734Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T05:22:30.1571987Z error: Could not document `syntax`.
2019-10-11T05:22:30.1573044Z 
2019-10-11T05:22:30.1573497Z Caused by:
2019-10-11T05:22:30.1578043Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-8c129151da7679a3.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-19bb54174cc172de.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-e02597a67963bf62.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-2acfbd3010664d93.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-16584ffb0cc8c85b.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-2fe15f7b14a92e0a.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-07698344d0e4d053.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5f652e9d50229f1f.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-e6b3662bad8dcee6.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c14840a48c3730ee.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-12bcd2a8dd59a33c.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f1779b106d702e4c.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-11T05:22:36.6681792Z [RUSTC-TIMING] syntax test:false 11.920
2019-10-11T05:22:36.6826069Z error: build failed
2019-10-11T05:22:36.6849366Z 
2019-10-11T05:22:36.6849850Z 
2019-10-11T05:22:36.6849850Z 
2019-10-11T05:22:36.6859358Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_codegen_llvm" "-p" "rustc_plugin" "-p" "rustc_privacy" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "syntax" "-p" "rustc_typeck" "-p" "rustc_fs_util" "-p" "rustc_traits" "-p" "rustc_codegen_utils" "-p" "rustc_plugin_impl" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_index" "-p" "rustc_interface" "-p" "fmt_macros" "-p" "rustc_lint" "-p" "rustc_macros" "-p" "rustc_resolve" "-p" "rustc_codegen_ssa" "-p" "arena" "-p" "syntax_ext" "-p" "rustc_llvm" "-p" "rustc" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "graphviz" "-p" "rustc_metadata" "-p" "rustc_passes" "-p" "syntax_pos" "-p" "rustc_errors" "-p" "rustc_incremental" "-p" "build_helper" "-p" "serialize" "-p" "rustc_data_structures"
2019-10-11T05:22:36.6859960Z 
2019-10-11T05:22:36.6860022Z 
2019-10-11T05:22:36.6870125Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-11T05:22:36.6870333Z Build completed unsuccessfully in 1:35:42
2019-10-11T05:22:36.6870333Z Build completed unsuccessfully in 1:35:42
2019-10-11T05:22:36.6934802Z == clock drift check ==
2019-10-11T05:22:36.6955429Z   local time: Fri Oct 11 05:22:36 UTC 2019
2019-10-11T05:22:37.2649705Z   network time: Fri, 11 Oct 2019 05:22:37 GMT
2019-10-11T05:22:37.2650855Z == end clock drift check ==
2019-10-11T05:22:38.7679494Z ##[error]Bash exited with code '1'.
2019-10-11T05:22:38.7724938Z ##[section]Starting: Upload CPU usage statistics
2019-10-11T05:22:38.7742345Z ==============================================================================
2019-10-11T05:22:38.7742463Z Task         : Bash
2019-10-11T05:22:38.7742567Z Description  : Run a Bash script on macOS, Linux, or Windows
