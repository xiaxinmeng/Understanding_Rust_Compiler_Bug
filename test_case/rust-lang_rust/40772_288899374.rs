
$ git clone --depth=1 https://github.com/rust-lang/rust.git rust-lang/rust
Cloning into 'rust-lang/rust'...
remote: Counting objects: 9861, done.
remote: Compressing objects: 100% (8740/8740), done.
remote: Total 9861 (delta 5307), reused 2301 (delta 1034), pack-reused 0
Receiving objects: 100% (9861/9861), 8.01 MiB | 10.89 MiB/s, done.
Resolving deltas: 100% (5307/5307), done.
$ cd rust-lang/rust
$ git fetch origin +refs/pull/40780/merge:
remote: Counting objects: 627478, done.
remote: Compressing objects: 100% (117810/117810), done.
remote: Total 627478 (delta 510690), reused 622389 (delta 505626), pack-reused 0
Receiving objects: 100% (627478/627478), 285.82 MiB | 24.97 MiB/s, done.
Resolving deltas: 100% (510690/510690), completed with 3765 local objects.
From https://github.com/rust-lang/rust
 * branch                  refs/pull/40780/merge -> FETCH_HEAD
$ git checkout -qf FETCH_HEAD
