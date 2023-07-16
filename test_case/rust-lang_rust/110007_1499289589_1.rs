
searched toolchains nightly-2022-01-01 through nightly-2023-04-06


********************************************************************************
Regression in nightly-2022-07-17
********************************************************************************

fetching https://static.rust-lang.org/dist/2022-07-16/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-07-16: 40 B / 40 B [===========================================================================================] 100.00 % 292.00 KB/s converted 2022-07-16 to 23e21bdd25026e2839ebe946c2a937c1904887d2
fetching https://static.rust-lang.org/dist/2022-07-17/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-07-17: 40 B / 40 B [===========================================================================================] 100.00 % 425.59 KB/s converted 2022-07-17 to d5e7f4782e4b699728d0a08200ecd1a54d56a85d
looking for regression commit between 2022-07-16 and 2022-07-17
cloning rust repository
fetching (via local git) commits from 23e21bdd25026e2839ebe946c2a937c1904887d2 to d5e7f4782e4b699728d0a08200ecd1a54d56a85d
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2022-07-15UTC: Auto merge of #99015 - lcnr:fn-ctxt-no-more, r=compiler-errors
  commit[1] 2022-07-15UTC: Auto merge of #99288 - Aaron1011:stable-intrinsics, r=yaahc
  commit[2] 2022-07-16UTC: Auto merge of #95685 - oxidecomputer:restore-static-dwarf, r=pnkfelix
  commit[3] 2022-07-16UTC: Auto merge of #96482 - willcrichton:fix-trait-suggestion-for-binops, r=estebank
  commit[4] 2022-07-16UTC: Auto merge of #99263 - compiler-errors:issue-99261, r=jyn514
  commit[5] 2022-07-16UTC: Auto merge of #99315 - JohnTitor:rollup-77wzoc1, r=JohnTitor
  commit[6] 2022-07-16UTC: Auto merge of #99346 - matthiaskrgr:rollup-p4dl1qt, r=matthiaskrgr
ERROR: no CI builds available between 23e21bdd25026e2839ebe946c2a937c1904887d2 and d5e7f4782e4b699728d0a08200ecd1a54d56a85d within last 167 days
