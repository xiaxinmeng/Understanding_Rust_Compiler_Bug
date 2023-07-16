
********************************************************************************
Regression in nightly-2021-04-01
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-03-31/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-03-31: 40 B / 40 B [===========================================] 100.00 % 638.13 KB/s converted 2021-03-31 to 74874a690bc95443292496ff5df5cc5c8cb56e0b
fetching https://static.rust-lang.org/dist/2021-04-01/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-04-01: 40 B / 40 B [===========================================] 100.00 % 575.33 KB/s converted 2021-04-01 to 4fdac23f3171e2f8864d359a21da600dd3faafc9
looking for regression commit between 2021-03-31 and 2021-04-01
refreshing repository at "/home/jess/src/rust"
From https://github.com/rust-lang/rust
 * branch                    HEAD       -> FETCH_HEAD
opening existing repository at "/home/jess/src/rust"
fetching (via local git) commits from 74874a690bc95443292496ff5df5cc5c8cb56e0b to 4fdac23f3171e2f8864d359a21da600dd3faafc9
refreshing repository at "/home/jess/src/rust"
From https://github.com/rust-lang/rust
 * branch                    HEAD       -> FETCH_HEAD
opening existing repository at "/home/jess/src/rust"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2021-03-30UTC: Auto merge of #83652 - xu-cheng:ipv4-octal, r=sfackler
  commit[1] 2021-03-30UTC: Auto merge of #83692 - Dylan-DPC:rollup-2a2m3jy, r=Dylan-DPC
  commit[2] 2021-03-31UTC: Auto merge of #83666 - Amanieu:instrprof-order, r=tmandry
  commit[3] 2021-03-31UTC: Auto merge of #83681 - jyn514:blanket-impls-tweaks, r=Aaron1011
  commit[4] 2021-03-31UTC: Auto merge of #83684 - cjgillot:csp, r=petrochenkov
  commit[5] 2021-03-31UTC: Auto merge of #76814 - jackh726:binder-refactor, r=nikomatsakis
ERROR: no CI builds available between 74874a690bc95443292496ff5df5cc5c8cb56e0b and 4fdac23f3171e2f8864d359a21da600dd3faafc9 within last 167 days
