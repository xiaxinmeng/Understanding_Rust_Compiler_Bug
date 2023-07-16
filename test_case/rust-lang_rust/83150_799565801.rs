
********************************************************************************
Regression in nightly-2020-09-19
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-09-18/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-09-18: 40 B / 40 B [===============] 100.00 % 696.29 KB/s converted 2020-09-18 to f3c923a13a458c35ee26b3513533fce8a15c9c05
fetching https://static.rust-lang.org/dist/2020-09-19/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-09-19: 40 B / 40 B [===============] 100.00 % 768.78 KB/s converted 2020-09-19 to bbc677480db8da85ea302e1e89d3df1f00e435bf
looking for regression commit between 2020-09-18 and 2020-09-19
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from f3c923a13a458c35ee26b3513533fce8a15c9c05 to bbc677480db8da85ea302e1e89d3df1f00e435bf
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 8 bors merge commits in the specified range
  commit[0] 2020-09-17UTC: Auto merge of #76645 - fusion-engineering-forks:windows-lock, r=kennytm
  commit[1] 2020-09-18UTC: Auto merge of #76790 - ssomers:btree_slice_slasher_returns, r=Mark-Simulacrum
  commit[2] 2020-09-18UTC: Auto merge of #76854 - aDotInTheVoid:mir-opt-32-64-diff-name, r=oli-obk
  commit[3] 2020-09-18UTC: Auto merge of #76837 - wesleywiser:disable_consideredequal, r=oli-obk
  commit[4] 2020-09-18UTC: Auto merge of #72412 - VFLashM:issue-72408-nested-closures-exponential, r=tmandry
  commit[5] 2020-09-18UTC: Auto merge of #76575 - lcnr:abstract-const, r=oli-obk
  commit[6] 2020-09-18UTC: Auto merge of #76884 - Mark-Simulacrum:fix-macos-ci, r=pietroalbini
  commit[7] 2020-09-18UTC: Auto merge of #76782 - lzutao:rd-cap, r=jyn514
ERROR: no commits between f3c923a13a458c35ee26b3513533fce8a15c9c05 and bbc677480db8da85ea302e1e89d3df1f00e435bf within last 167 days
