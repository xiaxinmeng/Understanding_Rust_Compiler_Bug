rust
   Compiling serde v1.0.8
     Running `rustc --crate-name serde /Users/simon/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.8/src/lib.rs --crate-type lib --emit=dep-info,metadata -C codegen-units=4 -C debuginfo=2 --cfg 'feature="std"' --cfg 'feature="rc"' --cfg 'feature="derive"' --cfg 'feature="default"' --cfg 'feature="serde_derive"' -C metadata=b12802d89eb13761 -C extra-filename=-b12802d89eb13761 --out-dir /Users/simon/projects/servo/target/debug/deps -L dependency=/Users/simon/projects/servo/target/debug/deps --extern serde_derive=/Users/simon/projects/servo/target/debug/deps/libserde_derive-87e93d0afba53041.dylib --cap-lints allow -W unused-extern-crates`
error: dlopen(/Users/simon/projects/servo/target/debug/deps/libserde_derive-87e93d0afba53041.dylib, 1): Library not loaded: @rpath/libstd-82587804d4836e27.dylib
  Referenced from: /Users/simon/projects/servo/target/debug/deps/libserde_derive-87e93d0afba53041.dylib
  Reason: image not found
   --> /Users/simon/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.8/src/lib.rs:261:1
    |
261 | extern crate serde_derive;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `serde`.
