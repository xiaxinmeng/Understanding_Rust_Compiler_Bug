plain
2019-10-03T20:59:16.7292508Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-03T20:59:17.1747966Z [RUSTC-TIMING] arena test:false 0.440
2019-10-03T20:59:17.1792999Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-03T20:59:17.2366833Z [RUSTC-TIMING] tempfile test:false 3.615
2019-10-03T20:59:17.2399764Z error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-f9c41f60c3e440f1.so: undefined symbol: __rustc_proc_macro_decls_7b3f997c3f27a6e9835a42bc866723ac__
2019-10-03T20:59:17.2400170Z  --> src/libsyntax_pos/symbol.rs:8:5
2019-10-03T20:59:17.2400682Z 8 | use rustc_macros::symbols;
2019-10-03T20:59:17.2401305Z   |     ^^^^^^^^^^^^
2019-10-03T20:59:17.2401359Z 
2019-10-03T20:59:17.2417103Z error: aborting due to previous error
---
2019-10-03T20:59:17.2617578Z == clock drift check ==
2019-10-03T20:59:17.2630921Z   local time: Thu Oct  3 20:59:17 UTC 2019
2019-10-03T20:59:17.4106937Z   network time: Thu, 03 Oct 2019 20:59:17 GMT
2019-10-03T20:59:17.4114088Z == end clock drift check ==
2019-10-03T20:59:19.0974127Z ##[error]Bash exited with code '1'.
2019-10-03T20:59:19.1018285Z ##[section]Starting: Upload CPU usage statistics
2019-10-03T20:59:19.1025990Z ==============================================================================
2019-10-03T20:59:19.1026123Z Task         : Bash
2019-10-03T20:59:19.1026221Z Description  : Run a Bash script on macOS, Linux, or Windows
