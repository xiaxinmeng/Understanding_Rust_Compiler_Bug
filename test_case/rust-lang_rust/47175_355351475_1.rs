
$ target/release/bisect --preserve --test test.sh --start 9389e23a8a754097e233c7bf3ea1bb404ccf1075 --end 8e7a609e635b728eba65d471c985ab462dc4cfc7
INFO:rust_sysroot: Getting commits from the git checkout in 9389e23a8a754097e233c7bf3ea1bb404ccf1075...8e7a609e635b728eba65d471c985ab462dc4cfc7
INFO:rust_sysroot: Received 23 commits
Searching in 23 commits; about 5 steps
thread 'main' panicked at 'byte index 1747 is not a char boundary; it is inside '\u{0}' (bytes 1746..1747) of `qpassertion failed: `(left == right)`
  left: ``,
 right: ``  1.rscapacity overflowsrc/l`[...]', src/libcore/str/mod.rs:2234:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'main' panicked at 'byte index 1747 is not a char boundary; it is inside '\u{0}' (bytes 1746..1747) of `qpassertion failed: `(left == right)`
  left: ``,
 right: ``  1.rscapacity overflowsrc/l`[...]', src/libcore/str/mod.rs:2234:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
searched commits 9389e23a8a754097e233c7bf3ea1bb404ccf1075 through 8e7a609e635b728eba65d471c985ab462dc4cfc7
regression in 8; Some(Commit { sha: "b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d", date: 2018-01-01T19:04:33Z, summary: "Auto merge of #46735 - Manishearth:memchr-find, r=burntsushi" })
