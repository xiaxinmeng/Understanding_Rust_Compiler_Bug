plain
2019-10-19T12:32:02.3764081Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T12:32:02.3962771Z ##[command]git config gc.auto 0
2019-10-19T12:32:02.4048222Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T12:32:02.4088425Z ##[command]git config --get-all http.proxy
2019-10-19T12:32:02.4275780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-19T13:35:18.8264843Z .................................................................................................... 1600/9199
2019-10-19T13:35:24.3166470Z .................................................................................................... 1700/9199
2019-10-19T13:35:37.5520934Z ..............................i...............i..................................................... 1800/9199
2019-10-19T13:35:45.2266569Z .................................................................................................... 1900/9199
2019-10-19T13:35:59.8947144Z ....................iiiii........................................................................... 2000/9199
2019-10-19T13:36:10.9555064Z .................................................................................................... 2200/9199
2019-10-19T13:36:13.6142208Z .................................................................................................... 2300/9199
2019-10-19T13:36:19.1149652Z .................................................................................................... 2400/9199
2019-10-19T13:36:41.3976149Z .................................................................................................... 2500/9199
---
2019-10-19T13:39:43.2051862Z .......................i...............i............................................................ 4800/9199
2019-10-19T13:39:55.0850587Z .................................................................................................... 4900/9199
2019-10-19T13:40:01.5867277Z .................................................................................................... 5000/9199
2019-10-19T13:40:11.9901531Z .................................................................................................... 5100/9199
2019-10-19T13:40:18.9107019Z .......................ii.ii........................................................................ 5200/9199
2019-10-19T13:40:29.7131789Z .................................................................................................... 5400/9199
2019-10-19T13:40:39.8752861Z .........................................................................................i.......... 5500/9199
2019-10-19T13:40:48.3949123Z .................................................................................................... 5600/9199
2019-10-19T13:40:53.5051918Z .................................................................................................... 5700/9199
2019-10-19T13:40:53.5051918Z .................................................................................................... 5700/9199
2019-10-19T13:41:04.4775123Z ......................................................................................ii...i..ii.... 5800/9199
2019-10-19T13:41:31.6743452Z .................................................................................................... 6000/9199
2019-10-19T13:41:41.4501127Z .................................................................................................... 6100/9199
2019-10-19T13:41:48.7442712Z .................................................................................................... 6200/9199
2019-10-19T13:41:48.7442712Z .................................................................................................... 6200/9199
2019-10-19T13:42:03.1862164Z ........i..ii....................................................................................... 6300/9199
2019-10-19T13:42:23.3616522Z ....................................................................i............................... 6500/9199
2019-10-19T13:42:25.6340822Z .................................................................................................... 6600/9199
2019-10-19T13:42:28.2497556Z ...........................................i........................................................ 6700/9199
2019-10-19T13:42:32.1620106Z .................................................................................................... 6800/9199
---
2019-10-19T13:47:11.2943410Z  finished in 5.495
2019-10-19T13:47:11.3157660Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:47:11.5111866Z 
2019-10-19T13:47:11.5114629Z running 153 tests
2019-10-19T13:47:14.7148857Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-19T13:47:16.7097656Z .i.iiii..............i.........iii.i.........ii......
2019-10-19T13:47:16.7098189Z 
2019-10-19T13:47:16.7103389Z  finished in 5.394
2019-10-19T13:47:16.7305992Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:47:16.8997137Z 
---
2019-10-19T13:47:18.9762195Z  finished in 2.245
2019-10-19T13:47:18.9998620Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:47:19.1703945Z 
2019-10-19T13:47:19.1704700Z running 9 tests
2019-10-19T13:47:19.1705483Z iiiiiiiii
2019-10-19T13:47:19.1705798Z 
2019-10-19T13:47:19.1711524Z  finished in 0.171
2019-10-19T13:47:19.1900475Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:47:19.3464814Z 
---
2019-10-19T13:47:37.5779168Z  finished in 18.387
2019-10-19T13:47:37.6009014Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:47:37.7658317Z 
2019-10-19T13:47:37.7660019Z running 123 tests
2019-10-19T13:48:02.5291101Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-19T13:48:07.4243190Z i.i.i......iii.i.....ii
2019-10-19T13:48:07.4244612Z 
2019-10-19T13:48:07.4250804Z  finished in 29.824
2019-10-19T13:48:07.4264229Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T13:48:07.4264776Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-19T13:49:06.4259941Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-19T13:49:06.4260192Z 
2019-10-19T13:49:06.4260555Z error: test compilation failed although it shouldn't!
2019-10-19T13:49:06.4262607Z status: exit code: 1
2019-10-19T13:49:06.4263691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-19T13:49:06.4264325Z ------------------------------------------
2019-10-19T13:49:06.4264489Z 
2019-10-19T13:49:06.4264870Z ------------------------------------------
2019-10-19T13:49:06.4265047Z stderr:
2019-10-19T13:49:06.4265047Z stderr:
2019-10-19T13:49:06.4265396Z ------------------------------------------
2019-10-19T13:49:06.4266133Z error[E0603]: module `attr` is private
2019-10-19T13:49:06.4268564Z    |
2019-10-19T13:49:06.4268634Z LL | use syntax_parse::parser::attr::*;
2019-10-19T13:49:06.4268676Z    |                           ^^^^
2019-10-19T13:49:06.4268703Z 
---
2019-10-19T13:49:06.4269954Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-19T13:49:06.4270000Z 
2019-10-19T13:49:06.4270185Z error: test compilation failed although it shouldn't!
2019-10-19T13:49:06.4270436Z status: exit code: 1
2019-10-19T13:49:06.4272602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-19T13:49:06.4273465Z ------------------------------------------
2019-10-19T13:49:06.4273713Z 
2019-10-19T13:49:06.4274208Z ------------------------------------------
2019-10-19T13:49:06.4274261Z stderr:
2019-10-19T13:49:06.4274261Z stderr:
2019-10-19T13:49:06.4274746Z ------------------------------------------
2019-10-19T13:49:06.4274810Z error: linking with `cc` failed: exit code: 1
2019-10-19T13:49:06.4275014Z    |
2019-10-19T13:49:06.4288578Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a.mod_dir_path_canonicalized.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_parse-be06739791c84dda.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-87b46d8c22f2b0ac.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-bbc4fdcc98b23564.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbitflags-1d20064f9d234547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-5b6c0be680fce862.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_xid-c1d8a880b7fe3396.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-0895be2268e40d45.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm_size-30c77c42a168d2eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libatty-120d7fad69be9f19.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libannotate_snippets-84964021ffe68578.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_pos-850e8faecb630da2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscoped_tls-0a371d666a6fdd4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-165c680dc720762c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-a281e14b8bad1f26.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermcolor-2d669526d62a041e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-0ea386e0d6764ed9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libena-8ba6ab99d1eb93f4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstable_deref_trait-60958989d8df91b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-2b86088911066920.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblazy_static-970c0318ef352ecd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libjobserver-3d937367686562c7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-09b5feabe090e9fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-6fe836084a8ef9dd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hash-ea0ffb4806f338da.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbyteorder-438bd23301b44145.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-8e1fbe2ef0297409.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libindexmap-e2da9d9ab81ca751.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsmallvec-ed6261e56de39d6f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-c9ef8bf1c4e01287.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-6223164d41d2b0bc.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-08978da7d5842aa0" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-e3e8064b232577ad.rlib" "-Wl,-Bdynamic" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-10-19T13:49:06.4292530Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_parse-be06739791c84dda.rlib(syntax_parse-be06739791c84dda.syntax_parse.16r14jud-cgu.0.rcgu.o): In function `syntax_parse::parser::item::_$LT$impl$u20$syntax_parse..parser..Parser$GT$::parse_item_implementation::hb069fc64288b9603':
2019-10-19T13:49:06.4293187Z            syntax_parse.16r14jud-cgu.0:(.text._ZN12syntax_parse6parser4item46_$LT$impl$u20$syntax_parse..parser..Parser$GT$25parse_item_implementation17hb069fc64288b9603E+0x4051): undefined reference to `process_configure_mod'
2019-10-19T13:49:06.4293259Z            collect2: error: ld returned 1 exit status
2019-10-19T13:49:06.4293363Z 
2019-10-19T13:49:06.4293407Z error: aborting due to previous error
2019-10-19T13:49:06.4293436Z 
2019-10-19T13:49:06.4293463Z 
2019-10-19T13:49:06.4293463Z 
2019-10-19T13:49:06.4293719Z ------------------------------------------
2019-10-19T13:49:06.4293754Z 
2019-10-19T13:49:06.4293780Z 
2019-10-19T13:49:06.4294482Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-19T13:49:06.4294524Z 
2019-10-19T13:49:06.4294773Z error: test compilation failed although it shouldn't!
2019-10-19T13:49:06.4294823Z status: exit code: 1
2019-10-19T13:49:06.4295702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-19T13:49:06.4296034Z ------------------------------------------
2019-10-19T13:49:06.4296082Z 
2019-10-19T13:49:06.4296288Z ------------------------------------------
2019-10-19T13:49:06.4296491Z stderr:
2019-10-19T13:49:06.4296491Z stderr:
2019-10-19T13:49:06.4296681Z ------------------------------------------
2019-10-19T13:49:06.4297179Z error: linking with `cc` failed: exit code: 1
2019-10-19T13:49:06.4297218Z    |
2019-10-19T13:49:06.4308589Z    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a.pprust_expr_roundtrip.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_parse-be06739791c84dda.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-87b46d8c22f2b0ac.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-bbc4fdcc98b23564.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbitflags-1d20064f9d234547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-5b6c0be680fce862.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_xid-c1d8a880b7fe3396.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-0895be2268e40d45.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm_size-30c77c42a168d2eb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libatty-120d7fad69be9f19.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libannotate_snippets-84964021ffe68578.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_pos-850e8faecb630da2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscoped_tls-0a371d666a6fdd4b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-165c680dc720762c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-a281e14b8bad1f26.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermcolor-2d669526d62a041e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-0ea386e0d6764ed9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libena-8ba6ab99d1eb93f4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstable_deref_trait-60958989d8df91b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-2b86088911066920.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblazy_static-970c0318ef352ecd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libjobserver-3d937367686562c7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-09b5feabe090e9fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-6fe836084a8ef9dd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hash-ea0ffb4806f338da.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbyteorder-438bd23301b44145.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-8e1fbe2ef0297409.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libindexmap-e2da9d9ab81ca751.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsmallvec-ed6261e56de39d6f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-c9ef8bf1c4e01287.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-6223164d41d2b0bc.rlib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-08978da7d5842aa0" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-e3e8064b232577ad.rlib" "-Wl,-Bdynamic" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
2019-10-19T13:49:06.4312726Z    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_parse-be06739791c84dda.rlib(syntax_parse-be06739791c84dda.syntax_parse.16r14jud-cgu.0.rcgu.o): In function `syntax_parse::parser::item::_$LT$impl$u20$syntax_parse..parser..Parser$GT$::parse_item_implementation::hb069fc64288b9603':
2019-10-19T13:49:06.4313480Z            syntax_parse.16r14jud-cgu.0:(.text._ZN12syntax_parse6parser4item46_$LT$impl$u20$syntax_parse..parser..Parser$GT$25parse_item_implementation17hb069fc64288b9603E+0x4051): undefined reference to `process_configure_mod'
2019-10-19T13:49:06.4314185Z            collect2: error: ld returned 1 exit status
2019-10-19T13:49:06.4314270Z 
2019-10-19T13:49:06.4314332Z error: aborting due to previous error
2019-10-19T13:49:06.4314363Z 
2019-10-19T13:49:06.4314389Z 
---
2019-10-19T13:49:06.4317281Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-19T13:49:06.4317352Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-19T13:49:06.4317382Z 
2019-10-19T13:49:06.4317406Z 
2019-10-19T13:49:06.4319228Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T13:49:06.4319532Z 
2019-10-19T13:49:06.4319560Z 
2019-10-19T13:49:06.4319603Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T13:49:06.4319667Z Build completed unsuccessfully in 1:10:07
2019-10-19T13:49:06.4319667Z Build completed unsuccessfully in 1:10:07
2019-10-19T13:49:06.4369545Z == clock drift check ==
2019-10-19T13:49:06.4390536Z   local time: Sat Oct 19 13:49:06 UTC 2019
2019-10-19T13:49:06.7300142Z   network time: Sat, 19 Oct 2019 13:49:06 GMT
2019-10-19T13:49:06.7300496Z == end clock drift check ==
2019-10-19T13:49:07.3661187Z 
2019-10-19T13:49:07.3769034Z ##[error]Bash exited with code '1'.
2019-10-19T13:49:07.3808162Z ##[section]Starting: Checkout
2019-10-19T13:49:07.3809790Z ==============================================================================
2019-10-19T13:49:07.3809855Z Task         : Get sources
2019-10-19T13:49:07.3809915Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
