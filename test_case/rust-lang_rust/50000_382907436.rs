plain

[00:49:47] travis_time:end:stage0-compiletest:start=1524179092113563188,finish=1524179176231554681,duration=84117991493

[00:49:47] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: Libtest, is_ext_tool: false, extra_features: [] } -- 84.118
[00:49:47] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `wasm32-unknown-unknown` is not configured as a host, only as a target"', libcore/result.rs:945:5
[00:49:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:49:47] Build completed unsuccessfully in 0:47:04
travis_time:end:06658078:start=1524176189111669234,finish=1524179176505719513,duration=2987394050279

