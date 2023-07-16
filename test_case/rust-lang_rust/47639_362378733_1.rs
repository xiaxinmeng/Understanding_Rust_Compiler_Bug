
$ RUST_LOG=rust_sysroot=info target/release/bisect --preserve --test test.sh --start 33374fa9d09e2a790979b31e61100dfed4b44139 --end 7d6e5b9da0865fbc9fa54edb324fefe80f358da7
INFO:rust_sysroot: Getting commits from the git checkout in 33374fa9d09e2a790979b31e61100dfed4b44139...7d6e5b9da0865fbc9fa54edb324fefe80f358da7
INFO:rust_sysroot: Received 375 commits
Searching in 375 commits; about 9 steps
<snip>
searched commits 33374fa9d09e2a790979b31e61100dfed4b44139 through 7d6e5b9da0865fbc9fa54edb324fefe80f358da7
regression in 31; Some(Commit { sha: "e97ba83287a6f0f85cc9cc7a51ab309487e17038", date: 2017-11-25T19:00:45Z, summary: "Auto merge of #46115 - alexcrichton:add-wasm-target, r=kennytm" })
