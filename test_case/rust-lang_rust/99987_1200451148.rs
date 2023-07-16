plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 76822a28780a9a93be04409e52c5df21663aab97 and 727c6f680719862474526a9bea174e7aafa260d4
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/67)
.......    (67/67)


/checkout/src/test/rustdoc-gui/default-settings.goml An exception occured: connect ECONNREFUSED 127.0.0.1:41411
== STACKTRACE ==
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:533:16)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async main (/checkout/src/tools/rustdoc-gui/tester.js:242:13)
