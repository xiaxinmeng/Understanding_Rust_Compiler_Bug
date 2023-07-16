
$ git clone --depth=1 https://github.com/rust-lang/rust.git rust-lang/rust

Cloning into 'rust-lang/rust'...

remote: Counting objects: 10175, done.

remote: Compressing objects: 100% (8953/8953), done.

remote: Total 10175 (delta 5441), reused 2470 (delta 1126), pack-reused 0

Receiving objects: 100% (10175/10175), 8.09 MiB | 4.56 MiB/s, done.

Resolving deltas: 100% (5441/5441), done.

Checking out files: 100% (9696/9696), done.

$ cd rust-lang/rust

154.86s$ git fetch origin +refs/pull/41809/merge:

remote: Counting objects: 639664, done.

remote: Compressing objects: 100% (120194/120194), done.

error: RPC failed; curl 56 SSLRead() return error -36

fatal: The remote end hung up unexpectedly

fatal: early EOF

fatal: index-pack failed

The command "eval git fetch origin +refs/pull/41809/merge: " failed. Retrying, 2 of 3.

remote: Counting objects: 639664, done.

remote: Compressing objects: 100% (120194/120194), done.

error: RPC failed; curl 56 SSLRead() return error -36

fatal: The remote end hung up unexpectedly

fatal: early EOF

fatal: index-pack failed

The command "eval git fetch origin +refs/pull/41809/merge: " failed. Retrying, 3 of 3.

remote: Counting objects: 639664, done.

remote: Compressing objects: 100% (120194/120194), done.

error: RPC failed; curl 56 SSLRead() return error -36

fatal: The remote end hung up unexpectedly

fatal: early EOF

fatal: index-pack failed

The command "eval git fetch origin +refs/pull/41809/merge: " failed 3 times.

The command "git fetch origin +refs/pull/41809/merge:" failed and exited with 128 during .
