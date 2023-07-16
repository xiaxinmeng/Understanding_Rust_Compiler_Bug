
********************************************************************************
Regression in nightly-2021-09-02
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-09-01/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-09-01: 40 B / 40 B [=============================================================================================] 100.00 % 601.57 KB/s converted 2021-09-01 to 29ef6cf1637aa8317f8911f93f14e18d404c1b0e
fetching https://static.rust-lang.org/dist/2021-09-02/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-09-02: 40 B / 40 B [=============================================================================================] 100.00 % 345.56 KB/s converted 2021-09-02 to 50171c310cd15e1b2d3723766ce64e2e4d6696fc
looking for regression commit between 2021-09-01 and 2021-09-02
cloning rust repository
fetching (via local git) commits from 29ef6cf1637aa8317f8911f93f14e18d404c1b0e to 50171c310cd15e1b2d3723766ce64e2e4d6696fc
refreshing repository at "rust.git"
From https://github.com/rust-lang/rust
 * branch                    HEAD       -> FETCH_HEAD
opening existing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 9 bors merge commits in the specified range
  commit[0] 2021-08-31UTC: Auto merge of #88506 - Mark-Simulacrum:fix-rlibs, r=ehuss
  commit[1] 2021-08-31UTC: Auto merge of #88533 - oli-obk:tait_🧊, r=spastorino
  commit[2] 2021-09-01UTC: Auto merge of #87688 - camsteffen:let-else, r=cjgillot
  commit[3] 2021-09-01UTC: Auto merge of #88121 - camelid:better-recursive-alias-error, r=estebank
  commit[4] 2021-09-01UTC: Auto merge of #88272 - willcrichton:mutable-sparse-matrix, r=ecstatic-morse
  commit[5] 2021-09-01UTC: Auto merge of #88556 - m-ou-se:rollup-q636wyd, r=m-ou-se
  commit[6] 2021-09-01UTC: Auto merge of #88395 - ricky26:llvm-submodule, r=nikic
  commit[7] 2021-09-01UTC: Auto merge of #88269 - prconrad:doctest-persist-binaries, r=jyn514
  commit[8] 2021-09-01UTC: Auto merge of #88563 - ehuss:update-cargo-books, r=ehuss
