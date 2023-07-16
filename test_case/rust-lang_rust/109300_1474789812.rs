`
********************************************************************************
Regression in nightly-2022-09-30
********************************************************************************

fetching https://static.rust-lang.org/dist/2022-09-29/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-09-29: 40 B / 40 B [============================================================================================] 100.00 % 514.57 KB/s converted 2022-09-29 to ce7f0f1aa0f02c45cad0749e63f3086234b1f422
fetching https://static.rust-lang.org/dist/2022-09-30/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-09-30: 40 B / 40 B [==============================================================================================] 100.00 % 1.09 MB/s converted 2022-09-30 to 9c56d9d6fec6262bbb1549cfe466a812ae2c6523
looking for regression commit between 2022-09-29 and 2022-09-30
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
fetching (via local git) commits from ce7f0f1aa0f02c45cad0749e63f3086234b1f422 to 9c56d9d6fec6262bbb1549cfe466a812ae2c6523
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 9 bors merge commits in the specified range
  commit[0] 2022-09-28UTC: Auto merge of #100719 - CohenArthur:rust-safe-intrinsic-attribute, r=wesleywiser
  commit[1] 2022-09-28UTC: Auto merge of #102384 - camelid:extrainfo, r=GuillaumeGomez
  commit[2] 2022-09-29UTC: Auto merge of #101833 - jyn514:cross-compile-compiler-builtins, r=Mark-Simulacrum
  commit[3] 2022-09-29UTC: Auto merge of #102450 - JohnTitor:rollup-ahleg93, r=JohnTitor
  commit[4] 2022-09-29UTC: Auto merge of #102328 - cuviper:ibm-stack-probes, r=nagisa
  commit[5] 2022-09-29UTC: Auto merge of #102461 - oli-obk:split_collect_rs, r=lcnr
  commit[6] 2022-09-29UTC: Auto merge of #102471 - Dylan-DPC:rollup-ij3okjt, r=Dylan-DPC
  commit[7] 2022-09-29UTC: Auto merge of #101893 - oli-obk:lift_derive, r=lcnr
  commit[8] 2022-09-29UTC: Auto merge of #102482 - notriddle:rollup-fjm618g, r=notriddle
ERROR: no CI builds available between ce7f0f1aa0f02c45cad0749e63f3086234b1f422 and 9c56d9d6fec6262bbb1549cfe466a812ae2c6523 within last 167 days
