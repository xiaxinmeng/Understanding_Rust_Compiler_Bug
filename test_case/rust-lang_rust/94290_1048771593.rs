plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c651ba8a542c7d89b271efbf024a31091c824f4b and 9505fe35c0d4690471bd9627729e659b28c7bdbb
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.12.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
Incorrect number of arguments passed to called function!
  tail call void @_ZN4core9panicking15panic_no_unwind17heb5bd7bc5de639ecE(%"core::panic::location::Location"* bitcast (<{ i8*, [16 x i8] }>* @10 to %"core::panic::location::Location"*)) #15
in function _ZN12panic_unwind8real_imp5panic17exception_cleanup17h7b2c07bcac8db7c5E
LLVM ERROR: Broken function found, compilation aborted!
error: could not compile `panic_unwind`
Incorrect number of arguments passed to called function!
Incorrect number of arguments passed to called function!
  call void @_ZN4core9panicking15panic_no_unwind17heb5bd7bc5de639ecE(%"panic::location::Location"* bitcast (<{ i8*, [16 x i8] }>* @364 to %"panic::location::Location"*)) #26
in function _ZN4core9panicking15panic_no_unwind17heb5bd7bc5de639ecE
LLVM ERROR: Broken function found, compilation aborted!
Build completed unsuccessfully in 0:00:15
cat: /tmp/toolstate/toolstates.json: No such file or directory
