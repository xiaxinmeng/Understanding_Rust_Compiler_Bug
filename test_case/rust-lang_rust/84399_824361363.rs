
********************************************************************************
Regression in nightly-2020-04-10
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-04-09/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-04-09: 40 B / 40 B [=================] 100.00 % 1.13 MB/s converted 2020-04-09 to 485c5fb6e1bf12cd11a8fac5ee94962e17cff74b
fetching https://static.rust-lang.org/dist/2020-04-10/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-04-10: 40 B / 40 B [=================] 100.00 % 1.39 MB/s converted 2020-04-10 to 94d346360da50f159e0dc777dc9bc3c5b6b51a00
looking for regression commit between 2020-04-09 and 2020-04-10
cloning rust repository
fetching (via local git) commits from 485c5fb6e1bf12cd11a8fac5ee94962e17cff74b to 94d346360da50f159e0dc777dc9bc3c5b6b51a00
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2020-04-08UTC: Auto merge of #70931 - Dylan-DPC:rollup-f8orcao, r=Dylan-DPC
  commit[1] 2020-04-08UTC: Auto merge of #70721 - anyska:bless-all-mir-opt, r=oli-obk
  commit[2] 2020-04-09UTC: Auto merge of #70936 - Dylan-DPC:rollup-2ng3e5h, r=Dylan-DPC
  commit[3] 2020-04-09UTC: Auto merge of #70860 - lcnr:has_local_value, r=nikomatsakis
  commit[4] 2020-04-09UTC: Auto merge of #70943 - Centril:rollup-eowm2h3, r=Centril
  commit[5] 2020-04-09UTC: Auto merge of #70960 - Centril:rollup-9vmokvw, r=Centril
ERROR: no commits between 485c5fb6e1bf12cd11a8fac5ee94962e17cff74b and 94d346360da50f159e0dc777dc9bc3c5b6b51a00 within last 167 days
