
$ git clone --depth=1 https://github.com/rust-lang/rust.git rust-lang/rust
Cloning into 'rust-lang/rust'...
remote: Counting objects: 10010, done.
remote: Compressing objects: 100% (8825/8825), done.
remote: Total 10010 (delta 5369), reused 2433 (delta 1096), pack-reused 0
Receiving objects: 100% (10010/10010), 8.04 MiB | 10.93 MiB/s, done.
Resolving deltas: 100% (5369/5369), done.
$ cd rust-lang/rust
$ git fetch origin +refs/pull/41074/merge:
remote: Counting objects: 630073, done.
remote: Compressing objects: 100% (118309/118309), done.
remote: Total 630073 (delta 512723), reused 625020 (delta 507693), pack-reused 0
Receiving objects: 100% (630073/630073), 286.78 MiB | 25.92 MiB/s, done.
Resolving deltas: 100% (512723/512723), completed with 3750 local objects.
From https://github.com/rust-lang/rust
 * branch                  refs/pull/41074/merge -> FETCH_HEAD
