
hbina@akarin ~/g/lapce-prettier (master)> just diagnose
cargo +nightly build --target="wasm32-wasi"
   Compiling lapce_prettier v0.0.0 (/home/hbina/git/lapce-prettier)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
wasm-tools print target/wasm32-wasi/debug/lapce_prettier.wasm > ./lapce_prettier.txt
rg -w "export" lapce_prettier.txt
1262454:  (export "memory" (memory 0))
1262455:  (export "_start" (func $_start))
1262456:  (export "__main_void" (func $__main_void))
1262457:  (export "handle_rpc" (func $handle_rpc))
