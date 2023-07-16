plain
[00:04:01] curl: (56) SSL read: error:00000000:lib(0):func(0):reason(0), errno 104
[00:04:02] Command failed. Attempt 2/5:
[00:06:02] error: RPC failed; curl 56 GnuTLS recv error (-9): A TLS packet with unexpected length was received.
[00:06:02] fatal: The remote end hung up unexpectedly
[00:06:02] fatal: protocol error: bad pack header
[00:06:02] Failed to clone 'src/doc/reference'. Retry scheduled
[00:06:02] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:06:02] fatal: unable to access 'https://github.com/rust-lang/libc.git/': GnuTLS recv error (-9): A TLS packet with unexpected length was received.
[00:06:02] fatal: clone of 'https://github.com/rust-lang/libc.git' into submodule path '/home/travis/build/rust-lang/rust/src/liblibc' failed
---
[00:08:21] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:08:21] Submodule 'libm' (https://github.com/japaric/libm) registered for path 'src/libcompiler_builtins/libm'
[00:08:21] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
[00:08:25] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:11:23] fatal: unable to access 'https://github.com/japaric/libm/': GnuTLS recv error (-9): A TLS packet with unexpected length was received.
[00:11:23] fatal: clone of 'https://github.com/japaric/libm' into submodule path '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm' failed
[00:11:23] Failed to clone 'libm'. Retry scheduled
[00:11:24] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '8964c7a0c4d1dc8c45b5adee6fe5ae3db057ab98'
[00:11:24] Submodule path 'src/libcompiler_builtins/libm': checked out '96e36ea2620f9fbbaa46a01694a2fa3ef6c2fb7e'
[00:11:24] Submodule path 'src/libcompiler_builtins/libm': checked out '96e36ea2620f9fbbaa46a01694a2fa3ef6c2fb7e'
[00:11:24] Submodule path 'src/liblibc': checked out '6bdbf5dc937459bd10e6bc4dc52b0adbd8cf4358'
---
[00:17:11] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-5.0/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:17:11] Sending build context to Docker daemon  501.8kB
[00:17:11] Step 1/6 : FROM ubuntu:16.04
[00:17:12] 16.04: Pulling from library/ubuntu
[00:19:43] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/sha256:da6fba872ef36657a66384ffb2c2b5b5dc1672ec5f3b310a613c0ceb6f9b299a: read tcp 10.20.2.248:33692->52.54.155.177:443: read: connection reset by peer
[00:19:44] Sending build context to Docker daemon  501.8kB
[00:19:44] Step 1/6 : FROM ubuntu:16.04
[00:19:45] 16.04: Pulling from library/ubuntu
[00:19:45] Digest: sha256:3097ac92b852f878f802c22a38f97b097b4084dbef82893ba453ba0297d76a6a
