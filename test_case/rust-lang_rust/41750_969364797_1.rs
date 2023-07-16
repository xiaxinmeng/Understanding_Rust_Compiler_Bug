toml
[target.wasm32-unknown-emscripten]
rustflags = [
    "-C", "link-args=--no-entry",
    "-C", "link-args=-s",
    "-C", "link-args=ERROR_ON_UNDEFINED_SYMBOLS=0" ]
