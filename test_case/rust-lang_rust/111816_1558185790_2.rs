
searched toolchains nightly-2017-07-17 through nightly-2018-09-17


********************************************************************************
Regression in nightly-2017-10-14
********************************************************************************

fetching https://static.rust-lang.org/dist/2017-10-13/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2017-10-13: 40 B / 40 B [==============================================================================================================================] 100.00 % 913.59 KB/s converted 2017-10-13 to dcbbfb6e807fdff9c9ba80073bb755f9d9d95e31
fetching https://static.rust-lang.org/dist/2017-10-14/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2017-10-14: 40 B / 40 B [================================================================================================================================] 100.00 % 1.51 MB/s converted 2017-10-14 to 02a24dbdd8c3a5daa6578af72116020de75b5f93
looking for regression commit between 2017-10-13 and 2017-10-14
fetching (via remote github) commits from max(dcbbfb6e807fdff9c9ba80073bb755f9d9d95e31, 2017-10-11) to 02a24dbdd8c3a5daa6578af72116020de75b5f93
ending github query because we found starting sha: dcbbfb6e807fdff9c9ba80073bb755f9d9d95e31
get_commits_between returning commits, len: 10
  commit[0] 2017-10-12: Auto merge of #45007 - undecidabot:optimize-iter, r=bluss
  commit[1] 2017-10-12: Auto merge of #45233 - kennytm:rollup, r=kennytm
  commit[2] 2017-10-13: Auto merge of #45013 - chrisvittal:mir_pretty_printing_pr, r=nikomatsakis
  commit[3] 2017-10-13: Auto merge of #45025 - pnkfelix:mir-borrowck-moves-of-supporting-prefixes-invalidate-uses-too, r=arielb1
  commit[4] 2017-10-13: Auto merge of #45031 - alexcrichton:nounwind, r=arielb1
  commit[5] 2017-10-13: Auto merge of #45032 - alexcrichton:target-cfu, r=michaelwoerister
  commit[6] 2017-10-13: Auto merge of #45055 - GuillaumeGomez:search-tabs, r=QuietMisdreavus
  commit[7] 2017-10-13: Auto merge of #45063 - michaelwoerister:bring-back-incremental-info, r=nikomatsakis
  commit[8] 2017-10-13: Auto merge of #45261 - kennytm:rollup, r=kennytm
  commit[9] 2017-10-13: Auto merge of #45069 - sinkuu:tuple_arg, r=nikomatsakis
ERROR: no CI builds available between dcbbfb6e807fdff9c9ba80073bb755f9d9d95e31 and 02a24dbdd8c3a5daa6578af72116020de75b5f93 within last 167 days
