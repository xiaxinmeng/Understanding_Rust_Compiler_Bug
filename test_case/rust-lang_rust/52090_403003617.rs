rust
// path/to/some/crate/foo.rs
#[wasm_import_module = "./bar.js"]
extern {
    ...
}
