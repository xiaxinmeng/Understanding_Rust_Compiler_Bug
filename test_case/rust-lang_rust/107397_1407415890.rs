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
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.05s
Warning: --all-targets is now on by default and does not need to be passed explicitly.
Warning: Unable to find the stamp file, did you try to keep a nonexistent build stage?
