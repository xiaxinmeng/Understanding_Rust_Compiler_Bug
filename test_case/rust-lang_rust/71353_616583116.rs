
root@ubuntu-c-32-64gib-fra1-01:~# cargo bisect-rustc --preserve --test-dir=foo --start=2020-04-07 --end=2020-04-19installing nightly-2020-04-07
testing...
RESULT: nightly-2020-04-07, ===> No

installing nightly-2020-04-19
testing...
RESULT: nightly-2020-04-19, ===> Yes

installing nightly-2020-04-13
testing...
RESULT: nightly-2020-04-13, ===> No

installing nightly-2020-04-16
testing...
RESULT: nightly-2020-04-16, ===> No

installing nightly-2020-04-17
testing...
RESULT: nightly-2020-04-17, ===> Yes

searched toolchains nightly-2020-04-07 through nightly-2020-04-19
regression in nightly-2020-04-17
fetching https://static.rust-lang.org/dist/2020-04-17/channel-rust-nightly-git-commit-hash.txt
converted 2020-04-17 to 7f3df5772439eee1c512ed2eb540beef1124d236
fetching https://static.rust-lang.org/dist/2020-04-16/channel-rust-nightly-git-commit-hash.txt
converted 2020-04-16 to d2230290f7220e740ec08f4d844bf5951e1b74b8
looking for regression commit between 2020-04-17 and 2020-04-16
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from d2230290f7220e740ec08f4d844bf5951e1b74b8 to 7f3df5772439eee1c512ed2eb540beef1124d236
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2020-04-15UTC: Auto merge of #71139 - matthiaskrgr:submodule_upd, r=Dylan-DPC
  commit[1] 2020-04-15UTC: Auto merge of #71180 - Dylan-DPC:rollup-pscpg6q, r=Dylan-DPC
  commit[2] 2020-04-16UTC: Auto merge of #71159 - topecongiro:rustfmt-1.4.14, r=Dylan-DPC
  commit[3] 2020-04-16UTC: Auto merge of #71173 - RalfJung:miri, r=RalfJung
  commit[4] 2020-04-16UTC: Auto merge of #70831 - sfackler:shrink-future-stack, r=matthewjasper
  commit[5] 2020-04-16UTC: Auto merge of #70755 - wesleywiser:simplify_locals_2_electric_boogaloo, r=oli-obk
  commit[6] 2020-04-16UTC: Auto merge of #71201 - Dylan-DPC:rollup-23202uf, r=Dylan-DPC
validated commits found, specifying toolchains
installing d2230290f7220e740ec08f4d844bf5951e1b74b8
testing...
RESULT: d2230290f7220e740ec08f4d844bf5951e1b74b8, ===> No

installing 7f3df5772439eee1c512ed2eb540beef1124d236
testing...
RESULT: 7f3df5772439eee1c512ed2eb540beef1124d236, ===> Yes

installing 534a41a32952d36ec73656357777ebbea707aeb4
testing...
RESULT: 534a41a32952d36ec73656357777ebbea707aeb4, ===> No

installing 4e4d49d60fd696c4036d438292673a2d7fd34519
testing...
RESULT: 4e4d49d60fd696c4036d438292673a2d7fd34519, ===> No

installing 7fb5187d0423f4cd0441526571b8cd61927123c9
testing...
RESULT: 7fb5187d0423f4cd0441526571b8cd61927123c9, ===> No

searched toolchains d2230290f7220e740ec08f4d844bf5951e1b74b8 through 7f3df5772439eee1c512ed2eb540beef1124d236
installing 7f3df5772439eee1c512ed2eb540beef1124d236
testing...
regression in 7f3df5772439eee1c512ed2eb540beef1124d236


searched nightlies: from nightly-2020-04-07 to nightly-2020-04-19
regressed nightly: nightly-2020-04-17
searched commits: from https://github.com/rust-lang/rust/commit/d2230290f7220e740ec08f4d844bf5951e1b74b8 to https://github.com/rust-lang/rust/commit/7f3df5772439eee1c512ed2eb540beef1124d236
regressed commit: https://github.com/rust-lang/rust/commit/7f3df5772439eee1c512ed2eb540beef1124d236
source code: URL OF A REPOSITORY THAT REPRODUCES THE ERROR
