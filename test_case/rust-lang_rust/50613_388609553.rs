plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:15253bea
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
[00:00:23] curl: (22) The requested URL returned error: 404 Not Found
[00:00:23] The command has failed after 5 attempts.
[00:00:23] open /tmp/rustci_docker_cache: no such file or directory
[00:00:23] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:23] Sending build context to Docker daemon  498.2kB
[00:00:23] Step 1/9 : FROM ubuntu:16.04
[00:00:24] 16.04: Pulling from library/ubuntu
---
[00:03:41] Verifying status of miri...
[00:03:41] Cloning into 'rust-toolstate'...
[00:03:41] [master 76a67ab] (linux CI update)
[00:03:41]  1 file changed, 1 insertion(+)
[00:03:42] error: src refspec test does not match any.
[00:03:42] error: failed to push some refs to 'https://github.com/rust-lang-nursery/rust-toolstate.git'
27532 ./.git/modules/src/tools/lld/objects
27524 ./.git/modules/src/tools/lld/objects/pack
27088 ./src/llvm-emscripten/test/CodeGen/X86
26664 ./src/tools
---
10724 ./.git/objects/pack
10092 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9660 ./.git/modules/src/tools/clippy
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
9452 ./.git/modules/src/tools/clippy/objects
9444 ./.git/modules/src/tools/clippy/objects/pack
8744 ./src/tools/lld/test
8568 ./src/llvm/lib/CodeGen
8368 ./src/llvm/test/CodeGen/AMDGPU
8172 ./src/libcompiler_builtins/compiler-rt/lib
