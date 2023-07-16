plain
configure: build.locked-deps    := True
configure: llvm.ccache          := sccache
configure: build.sanitizers     := True
configure: llvm.static-libstdcpp := True
configure: rust.codegen-backends := ['llvm', 'cranelift']
configure: rust.llvm-tools      := True
configure: build.extended       := True
configure: build.cargo-native-static := True
configure: rust.channel         := nightly
---
    Finished release [optimized] target(s) in 2m 04s
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
thread 'main' panicked at 'fs::read_to_string(&toml_file_name) failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1103:20
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:00:03
