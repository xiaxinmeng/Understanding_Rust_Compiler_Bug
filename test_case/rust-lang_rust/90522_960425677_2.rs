
$ rm -rf /tmp/pgo-data
$ RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data"     cargo build --release --target=x86_64-unknown-linux-gnu
   Compiling proc-macro2 v1.0.32
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.81
   Compiling libc v0.2.106
   Compiling autocfg v1.0.1
   Compiling pin-project-lite v0.2.7
   Compiling tokio v1.13.0
   Compiling quote v1.0.10
   Compiling num_cpus v1.13.0
   Compiling tokio-macros v1.5.1
   Compiling hello v0.1.0 (/home/user/hello)
    Finished release [optimized] target(s) in 9.38s

$ ./target/x86_64-unknown-linux-gnu/release/hello

$ alias rllvm-profdata=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata
$ rllvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

$  RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release --target=x86_64-unknown-linux-gnu

   Compiling pin-project-lite v0.2.7
   Compiling libc v0.2.106
   Compiling num_cpus v1.13.0
   Compiling tokio v1.13.0
   Compiling hello v0.1.0 (/home/user/hello)
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x4c7383)[0x7f65e7154383]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x141f0)[0x7f65e68fe1f0]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZNK4llvm7Mangler17getNameWithPrefixERNS_15SmallVectorImplIcEEPKNS_11GlobalValueEb+0x2e3)[0x7f65e3e39dc3]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZNK4llvm13TargetMachine9getSymbolEPKNS_11GlobalValueE+0x6f)[0x7f65e3e3933f]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZNK4llvm24TargetLoweringObjectFile21emitCGProfileMetadataERNS_10MCStreamerERNS_6ModuleE+0x164)[0x7f65e4a17524]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZNK4llvm27TargetLoweringObjectFileELF18emitModuleMetadataERNS_10MCStreamerERNS_6ModuleE+0xd5)[0x7f65e4a16f75]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZN4llvm10AsmPrinter14doFinalizationERNS_6ModuleE+0x25d)[0x7f65e496104d]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZN4llvm13FPPassManager14doFinalizationERNS_6ModuleE+0x2d)[0x7f65e497da0d]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.58.0-nightly.so(_ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE+0x3c5)[0x7f65e497d0e5]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x2444ca4)[0x7f65e90d1ca4]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x243fa0f)[0x7f65e90cca0f]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x2442600)[0x7f65e90cf600]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x23d51ab)[0x7f65e90621ab]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x23cf8ed)[0x7f65e905c8ed]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x24038f0)[0x7f65e90908f0]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-25ea0f19be037cbe.so(+0x24285ac)[0x7f65e90b55ac]
/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/libstd-8adcca4f1427867b.so(rust_metadata_std_fcea40badc263c8f+0xa94c3)[0x7f65e69c74c3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x9450)[0x7f65e68f3450]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x43)[0x7f65e6801d53]
error: could not compile `hello`

Caused by:
  process didn't exit successfully: `rustc --crate-name hello --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C metadata=f1308e41e83e897f -C extra-filename=-f1308e41e83e897f --out-dir /home/user/hello/target/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/user/hello/target/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/hello/target/release/deps --extern tokio=/home/user/hello/target/x86_64-unknown-linux-gnu/release/deps/libtokio-be8676721867c7d8.rlib -Cprofile-use=/tmp/pgo-data/merged.profdata`
  (signal: 11, SIGSEGV: invalid memory reference)
