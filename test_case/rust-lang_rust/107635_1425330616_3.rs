
hbina@akarin ~/g/lapce-prettier (master)> just diagnose
cargo +nightly build --target="wasm32-wasi"
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
wasm-tools print target/wasm32-wasi/debug/lapce_prettier.wasm > ./lapce_prettier.txt
rg -w "export" lapce_prettier.txt
1260914:  (export "memory" (memory 0))
1260915:  (export "handle_rpc" (func $handle_rpc.command_export))
