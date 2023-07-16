plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0dd7bb8f
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
################################################################          89.9%
######################################################################## 100.0%
[00:04:05] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:05]     Updating crates.io index
[00:04:12] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:12] Build completed unsuccessfully in 0:00:22
[00:04:12] make: *** [prepare] Error 1
[00:04:13] Command failed. Attempt 2/5:
[00:04:13] Command failed. Attempt 2/5:
[00:04:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:14] Build completed unsuccessfully in 0:00:00
[00:04:14] make: *** [prepare] Error 1
[00:04:16] Command failed. Attempt 3/5:
[00:04:16] Command failed. Attempt 3/5:
[00:04:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:16] Build completed unsuccessfully in 0:00:00
[00:04:16] make: *** [prepare] Error 1
[00:04:19] Command failed. Attempt 4/5:
[00:04:19] Command failed. Attempt 4/5:
[00:04:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:19] Build completed unsuccessfully in 0:00:00
[00:04:19] make: *** [prepare] Error 1
[00:04:23] Command failed. Attempt 5/5:
[00:04:23] Command failed. Attempt 5/5:
[00:04:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:24] Build completed unsuccessfully in 0:00:00
[00:04:24] make: *** [prepare] Error 1
[00:04:24] The command has failed after 5 attempts.
mpiler_builtins/modules/compiler-rt
---
10956 ./src/llvm/test/MC/Disassembler/AMDGPU
10820 ./src/tools/lldb/unittests
10508 ./src/llvm/test/MC/AMDGPU
10332 ./src/tools/clang/include
10140 ./src/tools/lldb/packages/Python/lldbsuite/test/functionalities/postmortem
10012 ./src/llvm-emscripten/test/MC/AMDGPU
9816 ./.git/modules/src/doc
9708 ./.git/modules/src/tools/rustfmt
9648 ./src/llvm-emscripten/test/MC/Disassembler/AMDGPU
