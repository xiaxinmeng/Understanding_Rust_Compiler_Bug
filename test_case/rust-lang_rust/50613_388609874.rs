plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:03b8b34c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:02:26] PageTables:         3820 kB
[00:02:26] NFS_Unstable:          0 kB
[00:02[00:02:29] From https://github.com/rust-lang-nursery/rust-toolstate
[00:02:29]  * branch            test       -> FETCH_HEAD
[00:02:29] fatal: ambiguous argument 'origin/test': unknown revision or path not in the working tree.
[00:02:29] Use '--' to separate paths from revisions, like this:
[00:02:29] 'git <command> [<revision>...] -- [<file>...]'

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 128.
travis_time:start:219eb57b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
