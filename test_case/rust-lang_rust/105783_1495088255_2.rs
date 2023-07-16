
searched toolchains nightly-2022-09-17 through nightly-2023-01-22


********************************************************************************
Regression in nightly-2022-10-11
********************************************************************************

fetching https://static.rust-lang.org/dist/2022-10-10/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-10-10: 40 B / 40 B [=================================================================================================================] 100.00 % 448.77 KB/s converted 2022-10-10 to 81f391930301afbc121b7c468138069daa354bf8
fetching https://static.rust-lang.org/dist/2022-10-11/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-10-11: 40 B / 40 B [=================================================================================================================] 100.00 % 507.02 KB/s converted 2022-10-11 to a6b7274a462829f8ef08a1ddcdcec7ac80dbf3e1
looking for regression commit between 2022-10-10 and 2022-10-11
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
fetching (via local git) commits from 81f391930301afbc121b7c468138069daa354bf8 to a6b7274a462829f8ef08a1ddcdcec7ac80dbf3e1
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2022-10-09: Auto merge of #102850 - JohnTitor:rollup-lze1w03, r=JohnTitor
  commit[1] 2022-10-09: Auto merge of #89123 - the8472:push_in_capacity, r=amanieu
  commit[2] 2022-10-10: Auto merge of #102867 - JohnTitor:rollup-qnwsajt, r=JohnTitor
  commit[3] 2022-10-10: Auto merge of #94381 - Kobzol:llvm-bolt, r=Mark-Simulacrum
  commit[4] 2022-10-10: Auto merge of #102875 - Dylan-DPC:rollup-zwcq8h9, r=Dylan-DPC
  commit[5] 2022-10-10: Auto merge of #96711 - emilio:inline-slice-clone, r=nikic
  commit[6] 2022-10-10: Auto merge of #102596 - scottmcm:option-bool-calloc, r=Mark-Simulacrum
ERROR: no CI builds available between 81f391930301afbc121b7c468138069daa354bf8 and a6b7274a462829f8ef08a1ddcdcec7ac80dbf3e1 within last 167 days
