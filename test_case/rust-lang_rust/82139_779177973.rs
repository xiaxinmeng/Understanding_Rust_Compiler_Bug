
********************************************************************************
Regression in nightly-2020-02-16
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-02-15/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-02-15: 40 B / 40 B [===============] 100.00 % 664.77 KB/s converted 2020-02-15 to 433aae93e4ef866a1fdfefad136b32ed89acd3e7
fetching https://static.rust-lang.org/dist/2020-02-16/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-02-16: 40 B / 40 B [===============] 100.00 % 674.07 KB/s converted 2020-02-16 to 61d9231ff2604a0467987042d9ebf9ff9ea739b5
looking for regression commit between 2020-02-15 and 2020-02-16
cloning rust repository
fetching (via local git) commits from 433aae93e4ef866a1fdfefad136b32ed89acd3e7 to 61d9231ff2604a0467987042d9ebf9ff9ea739b5
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 5 bors merge commits in the specified range
  commit[0] 2020-02-14UTC: Auto merge of #69115 - ehuss:update-books, r=Dylan-DPC
  commit[1] 2020-02-14UTC: Auto merge of #69172 - JohnTitor:rollup-6cbmwcw, r=JohnTitor
  commit[2] 2020-02-15UTC: Auto merge of #67681 - matthewjasper:infer-regions-in-borrowck, r=nikomatsakis
  commit[3] 2020-02-15UTC: Auto merge of #69182 - Dylan-DPC:rollup-ifsa9fx, r=Dylan-DPC
  commit[4] 2020-02-15UTC: Auto merge of #69168 - brainlock:test-textrel-regression, r=Mark-Simulacrum,tmandry
ERROR: no commits between 433aae93e4ef866a1fdfefad136b32ed89acd3e7 and 61d9231ff2604a0467987042d9ebf9ff9ea739b5 within last 167 days
