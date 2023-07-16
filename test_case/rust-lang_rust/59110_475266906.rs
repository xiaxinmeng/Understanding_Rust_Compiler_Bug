
Testing rustc_macros stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
running: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-musl" "-Zdual-proc-macros" "-j" "16" "-v" "--release" "--locked" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_macros" "--"
       Fresh unicode-xid v0.1.0
       Fresh proc-macro2 v0.4.24
       Fresh quote v0.6.10
       Fresh syn v0.15.22
       Fresh synstructure v0.10.1
       Fresh rustc_macros v0.1.0 (/checkout/src/librustc_macros)
    Finished release [optimized] target(s) in 0.23s
     Running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/rustc_macros-5dcf607750efd869`
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/rustc_macros-5dcf607750efd869: error while loading shared libraries: libstd-d301c78d889cf39d.so: cannot open shared object file: No such file or directory
error: test failed, to rerun pass '--lib'
