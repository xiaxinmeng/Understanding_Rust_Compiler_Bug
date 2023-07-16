plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c5d82ed7a4ad94a538bb87e5016e7d5ce0bd434b and 7d9c6e8ae6d959b64d81770143ba9ea9b7801530
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
+ /tmp/checktools.sh ../x.py
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
cat: /tmp/toolstate/toolstates.json: No such file or directory
