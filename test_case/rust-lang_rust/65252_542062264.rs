plain
2019-10-15T06:36:38.9477629Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-15T06:36:39.0512108Z error: Could not document `syntax`.
2019-10-15T06:36:39.0517653Z 
2019-10-15T06:36:39.0517779Z Caused by:
2019-10-15T06:36:39.0530499Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-28fa941764cd0b5d.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-25e195a008dc650e.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-6a39080e603a7540.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8fe38918b9a2a938.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f5bdf664f2735d32.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-da81bc8f818c54ab.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-96f153401da80175.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e6bb297dc731315e.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-7c3a2b01c74539dd.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-95bcadfb6805bd96.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-b7c252bfcc11690e.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b59e470af0b21954.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-15T06:36:45.1756638Z [RUSTC-TIMING] syntax test:false 10.807
2019-10-15T06:36:45.1892085Z error: build failed
2019-10-15T06:36:45.1914488Z 
2019-10-15T06:36:45.1915587Z 
2019-10-15T06:36:45.1915587Z 
2019-10-15T06:36:45.1918195Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc" "-p" "syntax" "-p" "syntax_pos" "-p" "rustc_save_analysis" "-p" "rustc_plugin" "-p" "rustc_lint" "-p" "graphviz" "-p" "rustc_codegen_utils" "-p" "build_helper" "-p" "rustc_llvm" "-p" "rustc_metadata" "-p" "rustc_index" "-p" "serialize" "-p" "arena" "-p" "rustc_macros" "-p" "fmt_macros" "-p" "rustc_plugin_impl" "-p" "rustc_passes" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_mir" "-p" "rustc_lexer" "-p" "rustc_errors" "-p" "rustc_incremental" "-p" "syntax_ext" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_target" "-p" "rustc_typeck" "-p" "rustc_interface" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "-p" "rustc_fs_util" "-p" "rustc_driver" "-p" "rustc_traits"
2019-10-15T06:36:45.1918820Z 
2019-10-15T06:36:45.1918865Z 
2019-10-15T06:36:45.1933358Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-15T06:36:45.1933496Z Build completed unsuccessfully in 1:31:35
2019-10-15T06:36:45.1933496Z Build completed unsuccessfully in 1:31:35
2019-10-15T06:36:45.1999935Z == clock drift check ==
2019-10-15T06:36:45.2019773Z   local time: Tue Oct 15 06:36:45 UTC 2019
2019-10-15T06:36:45.4473188Z   network time: Tue, 15 Oct 2019 06:36:45 GMT
2019-10-15T06:36:45.4473545Z == end clock drift check ==
2019-10-15T06:36:46.8921931Z ##[error]Bash exited with code '1'.
2019-10-15T06:36:46.8959790Z ##[section]Starting: Upload CPU usage statistics
2019-10-15T06:36:46.8968136Z ==============================================================================
2019-10-15T06:36:46.8968255Z Task         : Bash
2019-10-15T06:36:46.8968347Z Description  : Run a Bash script on macOS, Linux, or Windows
