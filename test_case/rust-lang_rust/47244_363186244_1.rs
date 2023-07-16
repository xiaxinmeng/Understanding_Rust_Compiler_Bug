
$ RUST_LOG=rust_sysroot=info target/release/bisect --preserve --test test.sh --start 33374fa9d09e2a790979b31e61100dfed4b44139 --end b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d
INFO:rust_sysroot: Getting commits from the git checkout in 33374fa9d09e2a790979b31e61100dfed4b44139...b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d
INFO:rust_sysroot: Received 272 commits
Searching in 272 commits; about 9 steps
rustc 1.24.0-nightly (830599b19 2017-12-11)
rustc 1.24.0-nightly (315fbf751 2017-12-01)
rustc 1.23.0-nightly (2f84fb5cc 2017-11-26)
rustc 1.23.0-nightly (0916bbc00 2017-11-23)
rustc 1.23.0-nightly (96e9cee77 2017-11-22)
rustc 1.23.0-nightly (ebda7662d 2017-11-21)
error[E0593]: closure is expected to take a single tuple as argument, but it takes 2 distinct arguments
rustc 1.23.0-nightly (63739ab7b 2017-11-21)
error[E0593]: closure is expected to take a single tuple as argument, but it takes 2 distinct arguments
rustc 1.23.0-nightly (d6d09e0b4 2017-11-21)
searched commits 33374fa9d09e2a790979b31e61100dfed4b44139 through b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d
regression in 7; Some(Commit { sha: "d6d09e0b4dac93ae07dae6206bf95e7cea0124a2", date: 2017-11-21T22:52:19Z, summary: "Auto merge of #45879 - nikomatsakis:nll-kill-cyclic-closures, r=arielb1" })
