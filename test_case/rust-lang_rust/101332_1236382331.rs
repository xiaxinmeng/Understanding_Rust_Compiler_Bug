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
   Compiling thiserror v1.0.33
   Compiling yaml-merge-keys v0.4.1
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
    Finished release [optimized] target(s) in 5.15s
error: .github/workflows/ci.yml is not up to date; please run `x.py run src/tools/expand-yaml-anchors`.
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different
