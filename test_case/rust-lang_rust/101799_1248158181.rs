plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
Build completed successfully in 0:03:30
+ RUSTDOCFLAGS=--document-private-items python3 ../x.py doc --stage 0 library/test
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'fs::copy(builder.src.join("src/doc/rust.css"), out.join("rust.css")) failed with No such file or directory (os error 2)', doc.rs:437:9
Build completed unsuccessfully in 0:00:00
