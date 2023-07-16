
$ RUST_LOG=rust_sysroot=info target/release/bisect --start b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d --end c83fa5d91c3b16459ab7b87c48ed18bd059a23af --test test.sh

INFO:rust_sysroot: Getting commits from the git checkout in b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d...c83fa5d91c3b16459ab7b87c48ed18bd059a23af
INFO:rust_sysroot: Received 159 commits
Searching in 159 commits; about 8 steps
rustc 1.25.0-nightly (97520ccb1 2018-01-21)
rustc 1.25.0-nightly (27ede5541 2018-01-10)
rustc 1.25.0-nightly (bb345a0be 2018-01-15)
rustc 1.25.0-nightly (9b2f8ac29 2018-01-13)
rustc 1.25.0-nightly (0b90e4e8c 2018-01-12)
rustc 1.25.0-nightly (619ced057 2018-01-11)
rustc 1.25.0-nightly (c9c298073 2018-01-11)
rustc 1.25.0-nightly (f62f77403 2018-01-10)
searched commits b65f0bedd2f22d9661ecb7092f07746dc2ccfb0d through c83fa5d91c3b16459ab7b87c48ed18bd059a23af
regression in 40; Some(Commit { sha: "f62f774035735a06c880c48c0b9017fcc0577e33", date: 2018-01-10T12:29:05Z, summary: "Auto merge of #47167 - ivanbakel:builtin_indexing, r=nikomatsakis" })
