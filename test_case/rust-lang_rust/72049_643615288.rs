
// Nightly with LD, not exported
$ cargo build --target i686-pc-windows-gnu; llvm-nm target/i686-pc-windows-gnu/debug/cdylib.dll | rg SomeFunction && echo implib: && llvm-nm target/i686-pc-windows-gnu/debug/libcdylib.dll.a | rg SomeFunction
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
69a814b0 T _SomeFunction@0
implib:

// Nightly with LLD, doesn't support linker script
$ RUSTFLAGS='-C link-arg=-fuse-ld=lld' cargo build --target i686-pc-windows-gnu; llvm-nm target/i686-pc-windows-gnu/debug/cdylib.dll | rg SomeFunction && echo implib: && llvm-nm target/i686-pc-windows-gnu/debug/libcdylib.dll.a | rg SomeFunction
[..]
  = note: lld: error: unknown argument: --version-script=C:\Users\mateusz\AppData\Local\Temp\rustc7TPSJT\list
[..]

// This PR with LD, almost good but it should have exported _SomeFunction@0 not _SomeFunction
$ cargo +stage2 build --target i686-pc-windows-gnu; llvm-nm target/i686-pc-windows-gnu/debug/cdylib.dll | rg SomeFunction && echo implib: && llvm-nm target/i686-pc-windows-gnu/debug/libcdylib.dll.a | rg SomeFunction
   Compiling cdylib v0.1.0 (D:\msys64\tmp\cdylib)
    Finished dev [unoptimized + debuginfo] target(s) in 0.83s
69a81460 T _SomeFunction
69a81460 T _SomeFunction@0
implib:
00000000 T _SomeFunction
00000000 I __imp__SomeFunction

// This PR with LLD, perfect, the test would pass (if there was any)
$ RUSTFLAGS='-C link-arg=-fuse-ld=lld' cargo +stage2 build --target i686-pc-windows-gnu; llvm-nm target/i686-pc-windows-gnu/debug/cdylib.dll | rg SomeFunction && echo implib: && llvm-nm target/i686-pc-windows-gnu/debug/libcdylib.dll.a | rg SomeFunction
   Compiling cdylib v0.1.0 (D:\msys64\tmp\cdylib)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
10001460 T _SomeFunction@0
implib:
00000000 T _SomeFunction@0
00000000 T __imp__SomeFunction@0
