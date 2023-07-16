plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/77285/merge:refs/remotes/pull/77285/merge
---
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
    Finished release [optimized] target(s) in 6.86s
error: .github/workflows/ci.yml is not up to date
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/expand-yaml-anchors" "check" "/checkout"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/tools/expand-yaml-anchors
Build completed unsuccessfully in 0:00:07
Build completed unsuccessfully in 0:00:07
== printing ll file ==
cat: 'build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/*.ll': No such file or directory
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6303) (node)
Terminate orphan process: pid (6312) (python)
