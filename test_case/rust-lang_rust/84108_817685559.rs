
********************************************************************************
Regression in nightly-2020-10-27
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-10-26/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-26: 40 B / 40 B [===============] 100.00 % 814.97 KB/s converted 2020-10-26 to 4760b8fb886a3702ae11bfa7868d495b2675b5ed
fetching https://static.rust-lang.org/dist/2020-10-27/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-27: 40 B / 40 B [=================] 100.00 % 1.02 MB/s converted 2020-10-27 to fd542592f08ca0d1f7255600115c2eafdf6b5da7
looking for regression commit between 2020-10-26 and 2020-10-27
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from 4760b8fb886a3702ae11bfa7868d495b2675b5ed to fd542592f08ca0d1f7255600115c2eafdf6b5da7
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 11 bors merge commits in the specified range
  commit[0] 2020-10-25UTC: Auto merge of #78179 - RalfJung:miri-comments, r=oli-obk
  commit[1] 2020-10-25UTC: Auto merge of #78015 - ssomers:btree_merge_mergers, r=Mark-Simulacrum
  commit[2] 2020-10-26UTC: Auto merge of #77283 - estebank:if-let-sugg, r=Mark-Simulacrum
  commit[3] 2020-10-26UTC: Auto merge of #78387 - Dylan-DPC:rollup-ch0st6z, r=Dylan-DPC
  commit[4] 2020-10-26UTC: Auto merge of #75728 - nagisa:improve_align_offset_2, r=Mark-Simulacrum
  commit[5] 2020-10-26UTC: Auto merge of #78324 - RalfJung:uninhabited-statics, r=oli-obk
  commit[6] 2020-10-26UTC: Auto merge of #78196 - pietroalbini:shipped-files, r=Mark-Simulacrum
  commit[7] 2020-10-26UTC: Auto merge of #78395 - RalfJung:miri, r=RalfJung
  commit[8] 2020-10-26UTC: Auto merge of #77975 - bjorn3:cg_clif_subtree3, r=Mark-Simulacrum
  commit[9] 2020-10-26UTC: Auto merge of #68965 - eddyb:mir-inline-scope, r=nagisa,oli-obk
  commit[10] 2020-10-26UTC: Auto merge of #77187 - TimDiekmann:box-alloc, r=Amanieu
ERROR: no commits between 4760b8fb886a3702ae11bfa7868d495b2675b5ed and fd542592f08ca0d1f7255600115c2eafdf6b5da7 within last 167 days
