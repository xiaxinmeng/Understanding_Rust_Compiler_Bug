
$ diff -r tmp-failure tmp-failure2
diff -r tmp-failure/.rustc_info.json tmp-failure2/.rustc_info.json
1c1
< {"rustc_fingerprint":4601125552553856084,"outputs":{"6217262102979750783":["___.wasm\nlib___.rlib\n___.wasm\nlib___.a\n/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu\ndebug_assertions\nproc_macro\ntarget_arch=\"wasm32\"\ntarget_endian=\"little\"\ntarget_env=\"\"\ntarget_has_atomic=\"16\"\ntarget_has_atomic=\"32\"\ntarget_has_atomic=\"64\"\ntarget_has_atomic=\"8\"\ntarget_has_atomic=\"cas\"\ntarget_has_atomic=\"ptr\"\ntarget_os=\"unknown\"\ntarget_pointer_width=\"32\"\ntarget_vendor=\"unknown\"\n","warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`\n\nwarning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`\n\n"],"1617349019360157463":["___\nlib___.rlib\nlib___.so\nlib___.so\nlib___.a\nlib___.so\n/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu\ndebug_assertions\nproc_macro\ntarget_arch=\"x86_64\"\ntarget_endian=\"little\"\ntarget_env=\"gnu\"\ntarget_family=\"unix\"\ntarget_feature=\"fxsr\"\ntarget_feature=\"mmx\"\ntarget_feature=\"sse\"\ntarget_feature=\"sse2\"\ntarget_has_atomic=\"16\"\ntarget_has_atomic=\"32\"\ntarget_has_atomic=\"64\"\ntarget_has_atomic=\"8\"\ntarget_has_atomic=\"cas\"\ntarget_has_atomic=\"ptr\"\ntarget_os=\"linux\"\ntarget_pointer_width=\"64\"\ntarget_thread_local\ntarget_vendor=\"unknown\"\nunix\n",""],"1164083562126845933":["rustc 1.33.0-nightly (2fadb0a16 2019-01-13)\nbinary: rustc\ncommit-hash: 2fadb0a16c8737a45746e95df9138912590ed8ad\ncommit-date: 2019-01-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.33.0-nightly\nLLVM version: 8.0\n",""]},"successes":{}}
\ No newline at end of file
---
> {"rustc_fingerprint":4601125552553856084,"outputs":{"1164083562126845933":["rustc 1.33.0-nightly (2fadb0a16 2019-01-13)\nbinary: rustc\ncommit-hash: 2fadb0a16c8737a45746e95df9138912590ed8ad\ncommit-date: 2019-01-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.33.0-nightly\nLLVM version: 8.0\n",""],"6217262102979750783":["___.wasm\nlib___.rlib\n___.wasm\nlib___.a\n/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu\ndebug_assertions\nproc_macro\ntarget_arch=\"wasm32\"\ntarget_endian=\"little\"\ntarget_env=\"\"\ntarget_has_atomic=\"16\"\ntarget_has_atomic=\"32\"\ntarget_has_atomic=\"64\"\ntarget_has_atomic=\"8\"\ntarget_has_atomic=\"cas\"\ntarget_has_atomic=\"ptr\"\ntarget_os=\"unknown\"\ntarget_pointer_width=\"32\"\ntarget_vendor=\"unknown\"\n","warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`\n\nwarning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`\n\n"],"1617349019360157463":["___\nlib___.rlib\nlib___.so\nlib___.so\nlib___.a\nlib___.so\n/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu\ndebug_assertions\nproc_macro\ntarget_arch=\"x86_64\"\ntarget_endian=\"little\"\ntarget_env=\"gnu\"\ntarget_family=\"unix\"\ntarget_feature=\"fxsr\"\ntarget_feature=\"mmx\"\ntarget_feature=\"sse\"\ntarget_feature=\"sse2\"\ntarget_has_atomic=\"16\"\ntarget_has_atomic=\"32\"\ntarget_has_atomic=\"64\"\ntarget_has_atomic=\"8\"\ntarget_has_atomic=\"cas\"\ntarget_has_atomic=\"ptr\"\ntarget_os=\"linux\"\ntarget_pointer_width=\"64\"\ntarget_thread_local\ntarget_vendor=\"unknown\"\nunix\n",""]},"successes":{}}
\ No newline at end of file
diff -r tmp-failure/release/.fingerprint/bit-io-derive-7588f3d609919d59/lib-bit_io_derive-7588f3d609919d59 tmp-failure2/release/.fingerprint/bit-io-derive-7588f3d609919d59/lib-bit_io_derive-7588f3d609919d59
1c1
< 13c80cb41b35bcc5
\ No newline at end of file
---
> 82d7e55a7525a356
\ No newline at end of file
diff -r tmp-failure/release/.fingerprint/bit-io-derive-7588f3d609919d59/lib-bit_io_derive-7588f3d609919d59.json tmp-failure2/release/.fingerprint/bit-io-derive-7588f3d609919d59/lib-bit_io_derive-7588f3d609919d59.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":14880731055498822660,"profile":4056442569674558749,"path":4160252336531197319,"deps":[["proc-macro2 v0.4.23","proc_macro2",6299910923903654121],["quote v0.6.10","quote",18326760302049624648],["syn v0.15.20","syn",11401365078351496994]],"local":[{"MtimeBased":[[1547500213,0],".fingerprint/bit-io-derive-7588f3d609919d59/dep-lib-bit_io_derive-7588f3d609919d59"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":14880731055498822660,"profile":4056442569674558749,"path":4160252336531197319,"deps":[["proc-macro2 v0.4.23","proc_macro2",6299910923903654121],["quote v0.6.10","quote",18326760302049624648],["syn v0.15.20","syn",11401365078351496994]],"local":[{"MtimeBased":[[1547500728,0],".fingerprint/bit-io-derive-7588f3d609919d59/dep-lib-bit_io_derive-7588f3d609919d59"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/release/.fingerprint/proc-macro2-714b8af757bd42e8/build tmp-failure2/release/.fingerprint/proc-macro2-714b8af757bd42e8/build
1c1
< ac622a558c9e48d2
\ No newline at end of file
---
> d31d896b2a56df75
\ No newline at end of file
diff -r tmp-failure/release/.fingerprint/proc-macro2-714b8af757bd42e8/build.json tmp-failure2/release/.fingerprint/proc-macro2-714b8af757bd42e8/build.json
1c1
< {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500125,0],"build/proc-macro2-714b8af757bd42e8/output"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500642,0],"build/proc-macro2-714b8af757bd42e8/output"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/lib-ans-8d251b1eddb4815c tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/lib-ans-8d251b1eddb4815c
1c1
< 0f55abc15378d276
\ No newline at end of file
---
> 0fc6f79fe977d04e
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/lib-ans-8d251b1eddb4815c.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/lib-ans-8d251b1eddb4815c.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":11143812123863548133,"profile":4056442569674558749,"path":6579037944379086315,"deps":[["bit-io v0.1.0 (/src/bit-io)","bit_io",9509097644641225333],["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028]],"local":[{"MtimeBased":[[1547500248,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/dep-lib-ans-8d251b1eddb4815c"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":11143812123863548133,"profile":4056442569674558749,"path":6579037944379086315,"deps":[["bit-io v0.1.0 (/src/bit-io)","bit_io",577165927740480094],["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028]],"local":[{"MtimeBased":[[1547500764,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/ans-8d251b1eddb4815c/dep-lib-ans-8d251b1eddb4815c"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/lib-bit_io-a7981cad456dda77 tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/lib-bit_io-a7981cad456dda77
1c1
< 750a582bed19f783
\ No newline at end of file
---
> 5e1a6e735a810208
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/lib-bit_io-a7981cad456dda77.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/lib-bit_io-a7981cad456dda77.json
1c1
< {"rustc":1915399918710576637,"features":"[\"default\", \"failure\", \"std\"]","target":17289339218366542335,"profile":4056442569674558749,"path":16849838035947673793,"deps":[["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028]],"local":[{"MtimeBased":[[1547500245,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/dep-lib-bit_io-a7981cad456dda77"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[\"default\", \"failure\", \"std\"]","target":17289339218366542335,"profile":4056442569674558749,"path":16849838035947673793,"deps":[["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028]],"local":[{"MtimeBased":[[1547500762,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/bit-io-a7981cad456dda77/dep-lib-bit_io-a7981cad456dda77"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/lib-client_wasm tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/lib-client_wasm
1c1
< 3c24a1c9b0ac3e1f
\ No newline at end of file
---
> 7ce45976130c71e2
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/lib-client_wasm.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/lib-client_wasm.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":7186147792127477003,"profile":4056442569674558749,"path":10872709659218687626,"deps":[["console_error_panic_hook v0.1.5","console_error_panic_hook",12439448065548290498],["game_core v0.1.0 (/src/game_core)","game_core",6407263028888969704],["host-native v0.1.0 (/src/host-native)","host_native",8489734473147781728],["js-sys v0.3.5","js_sys",7716729700818979661],["log v0.4.6","log",7302143067765734024],["utils v0.1.0 (/src/utils)","utils",7328506546309553957],["wasm-bindgen v0.2.28","wasm_bindgen",7117114712198101345],["web-console-logger v0.1.2","web_console_logger",6383578908039250069],["web-sys v0.3.5","web_sys",4340538738106084048]],"local":[{"MtimeBased":[[1547500378,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/dep-lib-client_wasm"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":7186147792127477003,"profile":4056442569674558749,"path":10872709659218687626,"deps":[["console_error_panic_hook v0.1.5","console_error_panic_hook",12439448065548290498],["game_core v0.1.0 (/src/game_core)","game_core",6462254084054765742],["host-native v0.1.0 (/src/host-native)","host_native",15716344863745358646],["js-sys v0.3.5","js_sys",7716729700818979661],["log v0.4.6","log",7302143067765734024],["utils v0.1.0 (/src/utils)","utils",16198687384152772183],["wasm-bindgen v0.2.28","wasm_bindgen",7117114712198101345],["web-console-logger v0.1.2","web_console_logger",6383578908039250069],["web-sys v0.3.5","web_sys",4340538738106084048]],"local":[{"MtimeBased":[[1547500894,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/client-wasm-1cd319b8fd8d3430/dep-lib-client_wasm"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/lib-game_core tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/lib-game_core
1c1
< e8a16b13c72beb58
\ No newline at end of file
---
> aed066d8d989ae59
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/lib-game_core.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/lib-game_core.json
1c1
< {"rustc":1915399918710576637,"features":"[\"windowing\"]","target":8199923762460833936,"profile":4056442569674558749,"path":2989653428462243033,"deps":[["ans v0.1.0 (/src/ans)","ans",8562038142711584015],["arraydeque v0.4.3","arraydeque",5532183644959529063],["arrayvec v0.4.7","arrayvec",6484549932760244449],["bit-io v0.1.0 (/src/bit-io)","bit_io",9509097644641225333],["bit-io-derive v0.1.0 (/src/bit-io-derive)","bit_io_derive",14248321714194532371],["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028],["hashers v1.0.1","hashers",2521454862387702790],["lazy_static v1.2.0","lazy_static",11658794385023691174],["log v0.4.6","log",7302143067765734024],["rand v0.6.0","rand",11426553532401451830],["rand_xorshift v0.1.0","rand_xorshift",4623798773986329789],["seq-buf v0.1.0 (/src/seq-buf)","seq_buf",6545764656016881710],["static_assertions v0.3.0","static_assertions",163316590932619041],["utils v0.1.0 (/src/utils)","utils",7328506546309553957]],"local":[{"MtimeBased":[[1547500268,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/dep-lib-game_core"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[\"windowing\"]","target":8199923762460833936,"profile":4056442569674558749,"path":2989653428462243033,"deps":[["ans v0.1.0 (/src/ans)","ans",5679170975409096207],["arraydeque v0.4.3","arraydeque",5532183644959529063],["arrayvec v0.4.7","arrayvec",6484549932760244449],["bit-io v0.1.0 (/src/bit-io)","bit_io",577165927740480094],["bit-io-derive v0.1.0 (/src/bit-io-derive)","bit_io_derive",6242874694432053122],["failure v0.1.3","failure",1789378418897756605],["failure_derive v0.1.3","failure_derive",1466114861772967028],["hashers v1.0.1","hashers",2521454862387702790],["lazy_static v1.2.0","lazy_static",11658794385023691174],["log v0.4.6","log",7302143067765734024],["rand v0.6.0","rand",11426553532401451830],["rand_xorshift v0.1.0","rand_xorshift",4623798773986329789],["seq-buf v0.1.0 (/src/seq-buf)","seq_buf",4823353591751167402],["static_assertions v0.3.0","static_assertions",163316590932619041],["utils v0.1.0 (/src/utils)","utils",16198687384152772183]],"local":[{"MtimeBased":[[1547500785,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/game_core-938eb93e42d2f528/dep-lib-game_core"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/lib-host_native-373ed1d511ca5ab7 tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/lib-host_native-373ed1d511ca5ab7
1c1
< 60925cd58598d175
\ No newline at end of file
---
> 36bb138362ac1bda
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/lib-host_native-373ed1d511ca5ab7.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/lib-host_native-373ed1d511ca5ab7.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":79871543664929709,"profile":4056442569674558749,"path":4912911018244300858,"deps":[["log v0.4.6","log",7302143067765734024],["utils v0.1.0 (/src/utils)","utils",7328506546309553957]],"local":[{"MtimeBased":[[1547500141,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/dep-lib-host_native-373ed1d511ca5ab7"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":79871543664929709,"profile":4056442569674558749,"path":4912911018244300858,"deps":[["log v0.4.6","log",7302143067765734024],["utils v0.1.0 (/src/utils)","utils",16198687384152772183]],"local":[{"MtimeBased":[[1547500657,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/host-native-373ed1d511ca5ab7/dep-lib-host_native-373ed1d511ca5ab7"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/lib-seq_buf-cf7f9aee0dc5108e tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/lib-seq_buf-cf7f9aee0dc5108e
1c1
< 2e783111463ad75a
\ No newline at end of file
---
> aacd6e5689feef42
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/lib-seq_buf-cf7f9aee0dc5108e.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/lib-seq_buf-cf7f9aee0dc5108e.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":9575394518290745232,"profile":4056442569674558749,"path":5266886399031786112,"deps":[["bit-io v0.1.0 (/src/bit-io)","bit_io",9509097644641225333],["bit-io-derive v0.1.0 (/src/bit-io-derive)","bit_io_derive",14248321714194532371],["static_assertions v0.2.5","static_assertions",17771686059335003230]],"local":[{"MtimeBased":[[1547500246,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/dep-lib-seq_buf-cf7f9aee0dc5108e"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":9575394518290745232,"profile":4056442569674558749,"path":5266886399031786112,"deps":[["bit-io v0.1.0 (/src/bit-io)","bit_io",577165927740480094],["bit-io-derive v0.1.0 (/src/bit-io-derive)","bit_io_derive",6242874694432053122],["static_assertions v0.2.5","static_assertions",17771686059335003230]],"local":[{"MtimeBased":[[1547500763,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/seq-buf-cf7f9aee0dc5108e/dep-lib-seq_buf-cf7f9aee0dc5108e"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/lib-utils-a98c4116e973a416 tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/lib-utils-a98c4116e973a416
1c1
< 25dfe732e015b465
\ No newline at end of file
---
> 570e07005a4ccde0
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/lib-utils-a98c4116e973a416.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/lib-utils-a98c4116e973a416.json
1c1
< {"rustc":1915399918710576637,"features":"[]","target":244887208638610850,"profile":4056442569674558749,"path":64702511458511907,"deps":[],"local":[{"MtimeBased":[[1547500125,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/dep-lib-utils-a98c4116e973a416"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"[]","target":244887208638610850,"profile":4056442569674558749,"path":64702511458511907,"deps":[],"local":[{"MtimeBased":[[1547500642,0],"/src/tmp/wasm32-unknown-unknown/release/.fingerprint/utils-a98c4116e973a416/dep-lib-utils-a98c4116e973a416"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/wasm-bindgen-cd8817de3844541a/build tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/wasm-bindgen-cd8817de3844541a/build
1c1
< 542b186c83b11803
\ No newline at end of file
---
> f1bb52f0568fa0e7
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/wasm-bindgen-cd8817de3844541a/build.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/wasm-bindgen-cd8817de3844541a/build.json
1c1
< {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500130,0],"/src/tmp/wasm32-unknown-unknown/release/build/wasm-bindgen-cd8817de3844541a/output"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500647,0],"/src/tmp/wasm32-unknown-unknown/release/build/wasm-bindgen-cd8817de3844541a/output"]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/web-sys-d998b76b14e5a5b6/build tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/web-sys-d998b76b14e5a5b6/build
1c1
< f187db61c343d178
\ No newline at end of file
---
> 2ccd70777184698d
\ No newline at end of file
diff -r tmp-failure/wasm32-unknown-unknown/release/.fingerprint/web-sys-d998b76b14e5a5b6/build.json tmp-failure2/wasm32-unknown-unknown/release/.fingerprint/web-sys-d998b76b14e5a5b6/build.json
1c1
< {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500385,0],"/src/tmp/wasm32-unknown-unknown/release/build/web-sys-d998b76b14e5a5b6/output"]},{"EnvBased":["__WASM_BINDGEN_DUMP_FEATURES",null]},{"EnvBased":["WEBIDL_RUSTFMT_BINDINGS",null]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
---
> {"rustc":1915399918710576637,"features":"","target":0,"profile":0,"path":0,"deps":[],"local":[{"MtimeBased":[[1547500902,0],"/src/tmp/wasm32-unknown-unknown/release/build/web-sys-d998b76b14e5a5b6/output"]},{"EnvBased":["__WASM_BINDGEN_DUMP_FEATURES",null]},{"EnvBased":["WEBIDL_RUSTFMT_BINDINGS",null]}],"rustflags":[],"edition":"Edition2015"}
\ No newline at end of file
Binary files tmp-failure/wasm32-unknown-unknown/release/client_wasm.wasm and tmp-failure2/wasm32-unknown-unknown/release/client_wasm.wasm differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.0.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.0.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.1.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.1.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.2.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.2.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.3.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.3.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.4.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.4.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.5.rcgu.o and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.client_wasm.8tlfxq5h-cgu.5.rcgu.o differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/client_wasm.wasm and tmp-failure2/wasm32-unknown-unknown/release/deps/client_wasm.wasm differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/game_core.wasm and tmp-failure2/wasm32-unknown-unknown/release/deps/game_core.wasm differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libans-8d251b1eddb4815c.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libans-8d251b1eddb4815c.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libconsole_error_panic_hook-7b2ca8fe1ff1170c.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libconsole_error_panic_hook-7b2ca8fe1ff1170c.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libgame_core.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libgame_core.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libhost_native-373ed1d511ca5ab7.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libhost_native-373ed1d511ca5ab7.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/liblog-00f7f036ccc616bf.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/liblog-00f7f036ccc616bf.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/librustc_demangle-a1dffddb8b02b09a.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/librustc_demangle-a1dffddb8b02b09a.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libweb_console_logger-13c9ac993d7ea9e5.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libweb_console_logger-13c9ac993d7ea9e5.rlib differ
Binary files tmp-failure/wasm32-unknown-unknown/release/deps/libweb_sys-b68829c5e399f1e0.rlib and tmp-failure2/wasm32-unknown-unknown/release/deps/libweb_sys-b68829c5e399f1e0.rlib differ
