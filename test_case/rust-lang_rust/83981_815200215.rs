plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0531]: cannot find unit struct, unit variant or constant `wasm` in module `sym`
   --> src/librustdoc/clean/cfg.rs:490:27
    |
490 |                     (sym::wasm, None) => "WebAssembly",
    |                           ^^^^ help: a constant with a similar name exists: `asm`
   ::: /checkout/compiler/rustc_span/src/symbol.rs:22:1
    |
22  | symbols! {
    | -------- similarly named constant `asm` defined here
