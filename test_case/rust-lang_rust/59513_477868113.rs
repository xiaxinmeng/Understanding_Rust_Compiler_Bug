plain
[00:01:41] + OUTPUT=/usr/local
[00:01:41] + shift
[00:01:41] + export 'CFLAGS=-fPIC -Wa,-mrelax-relocations=no'
[00:01:41] + CFLAGS='-fPIC -Wa,-mrelax-relocations=no'
[00:01:41] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:41] Cloning into 'musl-cross-make'...
[00:01:41] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:41] 
[00:01:41] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:41] changes and commit them, and you can discard any commits you make in this
[00:01:41] state without impacting any branches by performing another checkout.
[00:01:41] If you want to create a new branch to retain commits you create, you may
[00:01:41] If you want to create a new branch to retain commits you create, you may
[00:01:41] do so (now or later) by using -b with the checkout command again. Example:
[00:01:41] 
[00:01:41]   git checkout -b <new-branch-name>
[00:01:41] + cd musl-cross-make
[00:01:41] ++ nproc
[00:01:41] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:41] + set +x
---
[00:11:17] + set +x
[00:11:32] /build
[00:11:32] musl-toolchain.sh: line 9: 29664 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:32] + cd -
[00:11:32] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:11:32] + echo /usr/local/x86_64-linux-musl/lib
[00:11:32] + CC=x86_64-linux-musl-gcc
[00:11:32] + export CXX=x86_64-linux-musl-g++
[00:11:32] + CXX=x86_64-linux-musl-g++
[00:11:32] + LLVM=70
