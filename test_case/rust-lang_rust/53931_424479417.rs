plain
[00:53:37] ....................................................................................................
[00:53:40] ............................................................i.......................................
[00:53:42] ....................................................................................................
[00:53:45] ....................................................................................................
[00:53:48] .........iiiiiiiii..................................................................................
[00:53:53] ....................................................................................................
[00:53:57] .............................................................................................i......
[00:53:59] ....................................................................................................
[00:54:02] .....................................................i.i..ii........................................
---
[01:23:48] travis_fold:end:stage0-linkchecker

[01:23:48] travis_time:end:stage0-linkchecker:start=1537905101528242439,finish=1537905103873494024,duration=2345251585

[01:23:59] std/keyword.if.html:25: broken link - std/keyword.match.html
[01:23:59] std/keyword.crate.html:13: broken link - std/keyword.pub.html
[01:26:12] std/keyword.let.html:29: broken link - std/keyword.mut.html
[01:26:12] std/keyword.const.html:25: broken link - std/keyword.static.html
[01:26:15] std/keyword.as.html:16: broken link - std/keyword.use.html
[01:26:16] std/keyword.fn.html:29: broken link - std/keyword.self.html
[01:26:16] std/keyword.fn.html:44: broken link - std/keyword.where.html
[01:26:16] std/keyword.fn.html:47: broken link - std/keyword.pub.html
[01:26:17] std/keyword.for.html:5: broken link - std/keyword.break.html
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
gnu/stage1-rustc/x86_64-unknown-linux-gnu
143148 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
137468 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134140 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134136 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f551msp7v9-ukwhap-39xkf5uh8qqyb
133900 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125960 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
122628 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
118936 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
---
travis_time:end:157e19fc:start=1537905295520773877,finish=1537905295528493722,duration=7719845
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17ce4efe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; the
