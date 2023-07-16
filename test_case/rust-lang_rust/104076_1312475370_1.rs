text
  ~/devspace/personal/rust-dist/tools  $ ../../rust/x build --stage 1 rustdoc
Building rustbuild
   Compiling bootstrap v0.0.0 (/home/nimda/devspace/personal/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 2.47s

sysroot: "/home/nimda/devspace/personal/rust-dist/tools/build/x86_64-unknown-linux-gnu/stage0-sysroot"
compiler.stage: 0
builder.download_rustc() false


sysroot: "/home/nimda/devspace/personal/rust-dist/tools/build/x86_64-unknown-linux-gnu/stage1"
compiler.stage: 1
builder.download_rustc() false


sysroot: "/home/nimda/devspace/personal/rust-dist/tools/build/x86_64-unknown-linux-gnu/stage1"
compiler.stage: 1
builder.download_rustc() true


sysroot: "/home/nimda/devspace/personal/rust-dist/tools/build/x86_64-unknown-linux-gnu/stage0-sysroot"
compiler.stage: 0
builder.download_rustc() true

Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.14s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.15s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
warning: `download-rustc` does nothing when building stage1 tools; consider using `--stage 2` instead
Building rustdoc for stage1 (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.13s
Build completed successfully in 0:00:03
