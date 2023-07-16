`
********************************************************************************
Regression in nightly-2018-11-20
********************************************************************************

fetching https://static.rust-lang.org/dist/2018-11-19/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2018-11-19: 40 B / 40 B [============================================================================================] 100.00 % 528.10 KB/s converted 2018-11-19 to 13c9439925797cd7a65c917d047c07a500d9bfe6
fetching https://static.rust-lang.org/dist/2018-11-20/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2018-11-20: 40 B / 40 B [============================================================================================] 100.00 % 549.84 KB/s converted 2018-11-20 to 5aff30734b0e1056b1003459fbb27af199f51822
looking for regression commit between 2018-11-19 and 2018-11-20
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
fetching (via local git) commits from 13c9439925797cd7a65c917d047c07a500d9bfe6 to 5aff30734b0e1056b1003459fbb27af199f51822
opening existing repository at "rust.git"
Found origin remote under name `origin`
refreshing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2018-11-18UTC: Auto merge of #56017 - alexcrichton:debug-test, r=alexcrichton
  commit[1] 2018-11-18UTC: Auto merge of #55672 - RalfJung:miri-extern-types, r=eddyb
  commit[2] 2018-11-19UTC: Auto merge of #56042 - petrochenkov:nuni, r=petrochenkov
  commit[3] 2018-11-19UTC: Auto merge of #56051 - pietroalbini:rollup, r=pietroalbini
  commit[4] 2018-11-19UTC: Auto merge of #56060 - nrc:save-path-fallback, r=zackmdavis
  commit[5] 2018-11-19UTC: Auto merge of #55971 - SergioBenitez:skip-non-semantic, r=alexcrichton
ERROR: no CI builds available between 13c9439925797cd7a65c917d047c07a500d9bfe6 and 5aff30734b0e1056b1003459fbb27af199f51822 within last 167 days
