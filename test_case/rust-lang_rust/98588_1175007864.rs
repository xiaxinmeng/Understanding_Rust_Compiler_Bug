plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4045ce641a9eede71cc12031a2cd71692b273890 and 15608b10b14a5fe85ccff1bc4b0257e4ce6b5167
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
......     (66/66)


/checkout/src/test/rustdoc-gui/item-info-overflow.goml An exception occured: connect ECONNRESET 127.0.0.1:44009
== STACKTRACE ==
Error
    at innerRunTestCode (/node-v14.4.0-linux-x64/lib/node_modules/browser-ui-test/src/index.js:533:16)
    at runMicrotasks (<anonymous>)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
