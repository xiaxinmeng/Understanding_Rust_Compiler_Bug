compile_fail
2019-11-17T15:22:18.5077191Z   |   ^
2019-11-17T15:22:18.5077461Z   |
2019-11-17T15:22:18.5078052Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-11-17T15:22:18.5078431Z   |
2019-11-17T15:22:18.5078952Z 1 | ``'compile_fail
2019-11-17T15:22:18.5080361Z 
2019-11-17T15:22:18.5080683Z error: unknown start of token: `
2019-11-17T15:22:18.5081407Z  --> <doctest>:8:5
2019-11-17T15:22:18.5081777Z   |
---
2019-11-17T15:23:59.5925426Z     |                                 ^^^^^^^^ cannot be resolved, ignoring
2019-11-17T15:23:59.5925831Z     |
2019-11-17T15:23:59.5926205Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-11-17T15:23:59.5926281Z 
2019-11-17T15:23:59.6192737Z thread 'rustc' panicked at 'byte index 334 is out of bounds of `A doc comment (e.g. `/// ...`, `//! ...`, `/** ... */`, `/*! ... */`).
2019-11-17T15:23:59.6192945Z Doc attributes (e.g. `#[doc="..."]`) are represented with the `Normal`
2019-11-17T15:23:59.6193031Z variant (which is much less compact and thus more expensive).
2019-11-17T15:23:59.6193097Z 
2019-11-17T15:23:59.6193404Z Note: `self.has_name(sym::doc)` and `self.check_nam`[...]', src/libcore/macros/mod.rs:23:13
2019-11-17T15:23:59.7271182Z error: Could not document `syntax`.
2019-11-17T15:23:59.7271323Z 
2019-11-17T15:23:59.7271384Z Caused by:
2019-11-17T15:23:59.7271384Z Caused by:
2019-11-17T15:23:59.7274713Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name syntax src/libsyntax/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-4832b73a59cc0f66.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-5da31b467d36a336.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b71cb8d96ddad860.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dea507b75c596083.rmeta --extern rustc_error_codes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_codes-fb2a6d59a52a3b6f.rmeta --extern errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-ebacc70065319d20.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-07a3b9e8780801c6.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-9f91b4ca008ddea4.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-d39e9d1a030ad0d2.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-351ce27a546fb3a8.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-50b6da5c023b0a74.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-d3ea3842cfe923a7.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-11-17T15:24:01.8523670Z [RUSTC-TIMING] syntax test:false 6.582
2019-11-17T15:24:01.8670526Z error: build failed
2019-11-17T15:24:01.8700744Z 
2019-11-17T15:24:01.8700888Z 
2019-11-17T15:24:01.8700888Z 
2019-11-17T15:24:01.8703253Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_metadata" "-p" "rustc_codegen_llvm" "-p" "rustc_macros" "-p" "rustc_llvm" "-p" "rustc_error_codes" "-p" "rustc_privacy" "-p" "rustc_errors" "-p" "rustc_interface" "-p" "syntax_ext" "-p" "rustc_plugin" "-p" "build_helper" "-p" "rustc_codegen_ssa" "-p" "syntax" "-p" "rustc_traits" "-p" "syntax_pos" "-p" "rustc_index" "-p" "rustc_lint" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_codegen_utils" "-p" "rustc" "-p" "rustc_save_analysis" "-p" "rustc_driver" "-p" "serialize" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_parse" "-p" "rustc_fs_util" "-p" "arena" "-p" "rustc_plugin_impl" "-p" "rustc_resolve" "-p" "graphviz" "-p" "rustc_target" "-p" "syntax_expand" "-p" "rustc_typeck" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "fmt_macros"
2019-11-17T15:24:01.8703908Z 
2019-11-17T15:24:01.8703968Z 
2019-11-17T15:24:01.8704462Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-11-17T15:24:01.8704613Z Build completed unsuccessfully in 1:27:09
2019-11-17T15:24:01.8704613Z Build completed unsuccessfully in 1:27:09
2019-11-17T15:24:01.8749504Z == clock drift check ==
2019-11-17T15:24:01.8771550Z   local time: Sun Nov 17 15:24:01 UTC 2019
2019-11-17T15:24:02.4645875Z   network time: Sun, 17 Nov 2019 15:24:02 GMT
2019-11-17T15:24:02.4654357Z == end clock drift check ==
2019-11-17T15:24:05.8082040Z 
2019-11-17T15:24:05.8206621Z ##[error]Bash exited with code '1'.
2019-11-17T15:24:05.8244122Z ##[section]Starting: Checkout
2019-11-17T15:24:05.8246514Z ==============================================================================
2019-11-17T15:24:05.8246759Z Task         : Get sources
2019-11-17T15:24:05.8246853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
