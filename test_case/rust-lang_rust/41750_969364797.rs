shell
> export EMMAKEN_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0" 
> rustc --target=wasm32-unknown-emscripten main.rs
> 