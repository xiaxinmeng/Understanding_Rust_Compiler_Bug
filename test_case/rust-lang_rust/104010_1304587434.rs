plain
[RUSTC-TIMING] getopts test:false 0.188
[RUSTC-TIMING] proc_macro test:false 0.960
 Documenting test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1.95s
[TIMING] doc::Std { stage: 2, target: i686-unknown-linux-gnu, format: HTML } -- 58.686
Building stage1 tool error_index_generator (i686-unknown-linux-gnu)
[RUSTC-TIMING] build_script_build test:false 0.407
[RUSTC-TIMING] build_script_build test:false 0.435
[RUSTC-TIMING] unicode_ident test:false 0.097
---
[RUSTC-TIMING] unicode_width test:false 0.067
[RUSTC-TIMING] getopts test:false 0.188
 Documenting test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 0.94s
[TIMING] doc::Std { stage: 2, target: i686-unknown-linux-gnu, format: JSON } -- 18.130
thread 'main' panicked at 'could not read dir "/checkout/obj/build/i686-unknown-linux-gnu/json-doc": Os { code: 2, kind: NotFound, message: "No such file or directory" }', lib.rs:1578:25
Build completed unsuccessfully in 0:16:25
