
$ RUSTFLAGS='-C target-feature=+crt-static' cargo b
error: cannot produce proc-macro for `ctor v0.1.16 (/tmp/rust-ctor/ctor)` as the target `x86_64-unknown-linux-gnu` does not support these crate types

$ RUSTFLAGS='-C target-feature=+crt-static' cargo b --target x86_64-unknown-linux-gnu
   Compiling libc v0.2.79
   Compiling proc-macro2 v0.4.30
   Compiling unicode-xid v0.1.0
   Compiling proc-macro2 v1.0.23
   Compiling syn v0.15.44
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.46
   Compiling lazy_static v1.4.0
   Compiling quote v0.6.13
   Compiling quote v1.0.7
   Compiling libc-print v0.1.14
   Compiling dlopen_derive v0.1.4
   Compiling dlopen v0.1.8
   Compiling ctor v0.1.16 (/tmp/rust-ctor/ctor)
   Compiling tests v0.1.0 (/tmp/rust-ctor/tests)
    Finished dev [unoptimized + debuginfo] target(s) in 7.14s
