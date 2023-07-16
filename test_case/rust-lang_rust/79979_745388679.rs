plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +38dfb0954de378c4d730b43268cd5e043217d0af:refs/remotes/pull/79979/merge
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
    Finished release [optimized] target(s) in 7.04s
error: .github/workflows/ci.yml is not up to date


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/expand-yaml-anchors" "check" "/checkout"
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/tools/expand-yaml-anchors
Build completed unsuccessfully in 0:00:07
---
Entering 'src/tools/miri'
Entering 'src/tools/rls'
Entering 'src/tools/rust-analyzer'
Entering 'src/tools/rust-installer'
Entering 'src/tools/rustdoc-gui/browser-UI-test'
[command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
http.https://github.com/.extraheader
[command]/usr/bin/git config --local --unset-all http.https://github.com/.extraheader
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
---
Entering 'src/tools/miri'
Entering 'src/tools/rls'
Entering 'src/tools/rust-analyzer'
Entering 'src/tools/rust-installer'
Entering 'src/tools/rustdoc-gui/browser-UI-test'
Cleaning up orphan processes
Terminate orphan process: pid (6162) (node)
Terminate orphan process: pid (6171) (python)
