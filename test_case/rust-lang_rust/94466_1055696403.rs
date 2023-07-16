plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
     |
2922 |             let cxxflags = self
     |                 ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_cxxflags`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:01:12
