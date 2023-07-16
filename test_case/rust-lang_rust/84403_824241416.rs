plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling thiserror v1.0.20
   Compiling yaml-merge-keys v0.4.1
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
    Finished release [optimized] target(s) in 7.36s
error: .github/workflows/ci.yml is not up to date; please run `x.py run src/tools/expand-yaml-anchors`.
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/expand-yaml-anchors" "check" "/checkout"
expected success, got: exit code: 1

