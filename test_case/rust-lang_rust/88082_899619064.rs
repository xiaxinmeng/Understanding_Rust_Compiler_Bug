plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 73d96b090bb68065cd3a469b27cbd568e39bf0e7 and c1479f42520b497eeb425a18f851041398467f7d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 Documenting test_docs v0.1.0 (/checkout/src/test/rustdoc-gui/src/test_docs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
 Documenting settings v0.1.0 (/checkout/src/test/rustdoc-gui/src/settings)
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
`--jobs` option expects a positive number, found `--jobs`


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:11
