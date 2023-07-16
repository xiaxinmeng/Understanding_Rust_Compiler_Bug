plain
    Finished release [optimized] target(s) in 2m 22s
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
thread 'main' panicked at 'std::fs::write(dst_libdir.join("link-type.txt"), link_type) failed with No such file or directory (os error 2)', src/bootstrap/dist.rs:1982:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:00:02
