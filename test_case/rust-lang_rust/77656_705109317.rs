
>>> cargo bisect-rustc --preserve    

installing nightly-2020-10-05
testing...
RESULT: nightly-2020-10-05, ===> No

installing nightly-2020-10-06
testing...
RESULT: nightly-2020-10-06, ===> Yes

searched toolchains nightly-2020-10-05 through nightly-2020-10-06
installing nightly-2020-10-06
testing...


********************************************************************************
Regression in nightly-2020-10-06
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-10-05/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-05: 40 B / 40 B [================] 100.00 % 35.75 KB/s converted 2020-10-05 to beb5ae474d2835962ebdf7416bd1c9ad864fe101
fetching https://static.rust-lang.org/dist/2020-10-06/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-06: 40 B / 40 B [===============] 100.00 % 515.49 KB/s converted 2020-10-06 to a1dfd2490a6cb456b92e469fa550dc217e20ad6d
looking for regression commit between 2020-10-05 and 2020-10-06
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from beb5ae474d2835962ebdf7416bd1c9ad864fe101 to a1dfd2490a6cb456b92e469fa550dc217e20ad6d
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 8 bors merge commits in the specified range
  commit[0] 2020-10-04UTC: Auto merge of #77023 - HeroicKatora:len-missed-optimization, r=Mark-Simulacrum
  commit[1] 2020-10-05UTC: Auto merge of #77466 - Aaron1011:reland-drop-tree, r=matthewjasper
  commit[2] 2020-10-05UTC: Auto merge of #77557 - Dylan-DPC:rollup-aib9ptp, r=Dylan-DPC
  commit[3] 2020-10-05UTC: Auto merge of #77552 - ecstatic-morse:body-def-id, r=lcnr
  commit[4] 2020-10-05UTC: Auto merge of #77549 - tmiasko:simplify-branch-same-fix, r=oli-obk
  commit[5] 2020-10-05UTC: Auto merge of #77543 - Mark-Simulacrum:rsp-quoting, r=eddyb
  commit[6] 2020-10-05UTC: Auto merge of #77171 - VFLashM:better_sso_structures, r=oli-obk
  commit[7] 2020-10-05UTC: Auto merge of #77080 - richkadel:llvm-coverage-counters-2, r=tmandry
validated commits found, specifying toolchains

installing beb5ae474d2835962ebdf7416bd1c9ad864fe101
testing...
RESULT: beb5ae474d2835962ebdf7416bd1c9ad864fe101, ===> No

installing a1dfd2490a6cb456b92e469fa550dc217e20ad6d
testing...
RESULT: a1dfd2490a6cb456b92e469fa550dc217e20ad6d, ===> No

ERROR: the commit at the end of the range (a1dfd2490a6cb456b92e469fa550dc217e20ad6d) does not reproduce the regression
