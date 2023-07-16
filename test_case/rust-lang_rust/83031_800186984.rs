
********************************************************************************
Regression in nightly-2019-05-09
********************************************************************************

fetching https://static.rust-lang.org/dist/2019-05-08/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-05-08: 40 B / 40 B [===============] 100.00 % 652.99 KB/s converted 2019-05-08 to cfdc84a009020c59e53e4039beae22eb59e41685
fetching https://static.rust-lang.org/dist/2019-05-09/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-05-09: 40 B / 40 B [===============] 100.00 % 683.02 KB/s converted 2019-05-09 to 3f5152e200c0c02dfe0f79367948c98053d35855
looking for regression commit between 2019-05-08 and 2019-05-09
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from cfdc84a009020c59e53e4039beae22eb59e41685 to 3f5152e200c0c02dfe0f79367948c98053d35855
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2019-05-07UTC: Auto merge of #60612 - Centril:rollup-61drhqt, r=Centril
  commit[1] 2019-05-07UTC: Auto merge of #60586 - cramertj:await, r=oli-obk
  commit[2] 2019-05-08UTC: Auto merge of #60378 - froydnj:apple-target-modifications, r=michaelwoerister
  commit[3] 2019-05-08UTC: Auto merge of #60246 - Zoxc:hir-map-vec, r=eddyb
  commit[4] 2019-05-08UTC: Auto merge of #60626 - matthiaskrgr:submodule_upd, r=oli-obk
  commit[5] 2019-05-08UTC: Auto merge of #60402 - michaelwoerister:update-profiler-rt-build, r=alexcrichton
ERROR: no commits between cfdc84a009020c59e53e4039beae22eb59e41685 and 3f5152e200c0c02dfe0f79367948c98053d35855 within last 167 days
