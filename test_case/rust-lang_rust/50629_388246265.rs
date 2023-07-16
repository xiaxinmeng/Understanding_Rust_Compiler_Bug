
    Checking rustdoc v0.0.0 (file:///home/mark/Build/rust/src/librustdoc)
     Running `/home/mark/Build/rust/build/bootstrap/debug/rustc --crate-name rustdoc librustdoc/lib.rs --error-format json --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C metadata=db2cc8d63ee60236 -C extra-filename=-db2cc8d63ee60236 --out-dir /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern tempdir=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempdir-bbbd9a1d6520a8dc.rmeta --extern pulldown_cmark=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-aabc6d2d653f0cb7.rmeta`
rustc command: "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "rustdoc" "librustdoc/lib.rs" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "opt-level=3" "-C" "metadata=db2cc8d63ee60236" "-C" "extra-filename=-db2cc8d63ee60236" "--out-dir" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps" "--extern" "tempdir=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempdir-bbbd9a1d6520a8dc.rmeta" "--extern" "pulldown_cmark=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-aabc6d2d653f0cb7.rmeta" "--cfg" "stage0" "--sysroot" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-g" "-C" "debug-assertions=y" "-C" "codegen-units=16" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "--color=always" "-Dwarnings"
sysroot: "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot"
libdir: "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0/lib"
error[E0463]: can't find crate for `rustc`
  --> librustdoc/lib.rs:30:1
   |
30 | extern crate rustc;
   | ^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `rustdoc`.

Caused by:
  process didn't exit successfully: `/home/mark/Build/rust/build/bootstrap/debug/rustc --crate-name rustdoc librustdoc/lib.rs --error-format json --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C metadata=db2cc8d63ee60236 -C extra-filename=-db2cc8d63ee60236 --out-dir /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern tempdir=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempdir-bbbd9a1d6520a8dc.rmeta --extern pulldown_cmark=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-aabc6d2d653f0cb7.rmeta` (exit code: 101)
command did not execute successfully: "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "16" "-v" "--release" "--manifest-path" "/home/mark/Build/rust/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
