cmd
@set NAME=rust_wasm_sample
cargo build --target=wasm32-unknown-unknown
wasm-dwarf ^
       target/wasm32-unknown-unknown/debug/%NAME%.wasm ^
    -p /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/=file://C:/Users/%USERNAME%/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/ ^
    -p /cargo/registry/=file://C:/Users/%USERNAME%/.cargo/registry/ ^
    -p src/liballoc/=file://C:/Users/%USERNAME%/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/src/liballoc/ ^
    -p src/libcore/=file://C:/Users/%USERNAME%/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/src/libcore/ ^
    -p src/libstd/=file://C:/Users/%USERNAME%/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/src/libstd/ ^
    -p src/libpanic_abort/=file://C:/Users/%USERNAME%/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/src/libpanic_abort/ ^
    -p src/=../../../src/ ^
    -w target/wasm32-unknown-unknown/debug/%NAME%_dbg.wasm ^
    -o target/wasm32-unknown-unknown/debug/%NAME%_dbg.wasm.map ^
    -m %NAME%_dbg.wasm.map
