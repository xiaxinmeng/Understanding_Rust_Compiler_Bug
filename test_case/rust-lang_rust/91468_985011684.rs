plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0502]: cannot borrow `*cx` as mutable because it is also borrowed as immutable
    --> src/librustdoc/html/render/mod.rs:1282:97
     |
1248 |         if let Some(impls) = cx.cache().impls.get(&did) {
     |                              ---------- immutable borrow occurs here
1249 |             for i in impls {
     |                      ----- immutable borrow later used here
...
1282 |                                 assoc_type(&mut out, it, &[], Some(&tydef.type_), src_link, "", cx);
     |                                                                                                 ^^ mutable borrow occurs here
For more information about this error, try `rustc --explain E0502`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:21
