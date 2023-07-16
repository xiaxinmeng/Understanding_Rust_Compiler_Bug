plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d7dd01fe8b071602510eaac9f676acc0e3cf8e4a and 9487252b32a7f8c1c6a55c9e10e5cb14946f0ec7
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (80/81)
.          (81/81)


/checkout/src/test/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (line 22) Error: Evaluation failed: The following errors happened: [`Constants` isn't equal to `Traits`]: for command `assert-text: (".sidebar-elems section ul > li:nth-child(6)", "Traits")`
[ERROR] (line 23) Error: Evaluation failed: The following errors happened: [`Traits` isn't equal to `Functions`]: for command `assert-text: (".sidebar-elems section ul > li:nth-child(7)", "Functions")`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [`Functions` isn't equal to `Type Definitions`]: for command `assert-text: (".sidebar-elems section ul > li:nth-child(8)", "Type Definitions")`
[ERROR] (line 25) Error: Evaluation failed: The following errors happened: [`Type Definitions` isn't equal to `Unions`]: for command `assert-text: (".sidebar-elems section ul > li:nth-child(9)", "Unions")`
[ERROR] (line 26) Error: Evaluation failed: The following errors happened: [`Unions` isn't equal to `Keywords`]: for command `assert-text: (".sidebar-elems section ul > li:nth-child(10)", "Keywords")`
Build completed unsuccessfully in 0:01:39
