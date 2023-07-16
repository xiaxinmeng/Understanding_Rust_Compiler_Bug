
searched toolchains nightly-2021-07-12 through nightly-2021-08-10


********************************************************************************
Regression in nightly-2021-07-17
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-07-16/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-07-16: 40 B / 40 B [==========================================================================================================================] 100.00 % 1.18 MB/s converted 2021-07-16 to b1f8e27b74c541d3d555149c8efa4bfe9385cd56
fetching https://static.rust-lang.org/dist/2021-07-17/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-07-17: 40 B / 40 B [========================================================================================================================] 100.00 % 590.29 KB/s converted 2021-07-17 to 74ef0c3e404cc72c08b2d1e14506f90d9e877269
looking for regression commit between 2021-07-16 and 2021-07-17
refreshing repository at "rust.git"
opening existing repository at "rust.git"
fetching (via local git) commits from b1f8e27b74c541d3d555149c8efa4bfe9385cd56 to 74ef0c3e404cc72c08b2d1e14506f90d9e877269
refreshing repository at "rust.git"
opening existing repository at "rust.git"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 8 bors merge commits in the specified range
  commit[0] 2021-07-15UTC: Auto merge of #83319 - tmiasko:packed-aligned, r=jackh726
  commit[1] 2021-07-16UTC: Auto merge of #86993 - jackh726:project-gat-binders, r=nikomatsakis
  commit[2] 2021-07-16UTC: Auto merge of #87177 - ehuss:update-cargo, r=ehuss
  commit[3] 2021-07-16UTC: Auto merge of #86662 - mockersf:fix-86620-link-unknown-location, r=jyn514
  commit[4] 2021-07-16UTC: Auto merge of #87182 - GuillaumeGomez:rollup-whwohua, r=GuillaumeGomez
  commit[5] 2021-07-16UTC: Auto merge of #87140 - camsteffen:pat-slice-refs, r=oli-obk
  commit[6] 2021-07-16UTC: Auto merge of #84623 - jackh726:gats-incomplete, r=nikomatsakis
  commit[7] 2021-07-16UTC: Auto merge of #87201 - GuillaumeGomez:rollup-4loi2q9, r=GuillaumeGomez
validated commits found, specifying toolchains

installing b1f8e27b74c541d3d555149c8efa4bfe9385cd56
cargo for x86_64-unknown-linux-gnu: 6.06 MB / 6.06 MB [===========================================================================================================] 100.00 % 656.74 KB/s testing...
RESULT: b1f8e27b74c541d3d555149c8efa4bfe9385cd56, ===> No
uninstalling b1f8e27b74c541d3d555149c8efa4bfe9385cd56

installing 74ef0c3e404cc72c08b2d1e14506f90d9e877269
cargo for x86_64-unknown-linux-gnu: 6.07 MB / 6.07 MB [===========================================================================================================] 100.00 % 674.10 KB/s testing...
RESULT: 74ef0c3e404cc72c08b2d1e14506f90d9e877269, ===> Yes
uninstalling 74ef0c3e404cc72c08b2d1e14506f90d9e877269

installing a6470c7fa8a511cfbcf9e9d3e1ab6779ac661edb
cargo for x86_64-unknown-linux-gnu: 6.07 MB / 6.07 MB [===========================================================================================================] 100.00 % 797.24 KB/s testing...
RESULT: a6470c7fa8a511cfbcf9e9d3e1ab6779ac661edb, ===> No
uninstalling a6470c7fa8a511cfbcf9e9d3e1ab6779ac661edb

installing 2119976c492894b72287f08865c71d63cff8d471
cargo for x86_64-unknown-linux-gnu: 6.07 MB / 6.07 MB [===========================================================================================================] 100.00 % 674.55 KB/s testing...
RESULT: 2119976c492894b72287f08865c71d63cff8d471, ===> No
uninstalling 2119976c492894b72287f08865c71d63cff8d471

installing c49895d9049f67e07e297ee487836a587f69690e
cargo for x86_64-unknown-linux-gnu: 6.07 MB / 6.07 MB [===========================================================================================================] 100.00 % 692.50 KB/s testing...
RESULT: c49895d9049f67e07e297ee487836a587f69690e, ===> No
uninstalling c49895d9049f67e07e297ee487836a587f69690e

searched toolchains b1f8e27b74c541d3d555149c8efa4bfe9385cd56 through 74ef0c3e404cc72c08b2d1e14506f90d9e877269
installing 74ef0c3e404cc72c08b2d1e14506f90d9e877269
cargo for x86_64-unknown-linux-gnu: 6.07 MB / 6.07 MB [==========================================================================================================] 100.00 % 1003.23 KB/s testing...
uninstalling 74ef0c3e404cc72c08b2d1e14506f90d9e877269

********************************************************************************
Regression in 74ef0c3e404cc72c08b2d1e14506f90d9e877269
********************************************************************************

==================================================================================
= Please file this regression report on the rust-lang/rust GitHub repository     =
=        New issue: https://github.com/rust-lang/rust/issues/new                 =
=     Known issues: https://github.com/rust-lang/rust/issues                     =
= Copy and paste the text below into the issue report thread.  Thanks!           =
==================================================================================

searched nightlies: from nightly-2021-07-12 to nightly-2021-08-10
regressed nightly: nightly-2021-07-17
searched commit range: https://github.com/rust-lang/rust/compare/b1f8e27b74c541d3d555149c8efa4bfe9385cd56...74ef0c3e404cc72c08b2d1e14506f90d9e877269
regressed commit: https://github.com/rust-lang/rust/commit/74ef0c3e404cc72c08b2d1e14506f90d9e877269
