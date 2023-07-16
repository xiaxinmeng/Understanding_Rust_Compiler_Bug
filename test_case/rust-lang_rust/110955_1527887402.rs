plain
   Compiling askama_derive v0.12.1
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling askama v0.12.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: using `.clone()` on a double reference, which returns `&clean::types::GenericBound` instead of cloning the inner type
   --> src/librustdoc/html/format.rs:170:71
    |
170 |         for (i, bound) in bounds.iter().filter(|b| bounds_dup.insert(b.clone())).enumerate() {
    |
    |
    = note: `-D suspicious-double-ref-op` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:10:20
cat: /tmp/toolstate/toolstates.json: No such file or directory
