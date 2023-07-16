
installing nightly-2021-10-13
cargo for x86_64-unknown-linux-gnu: 6.23 MB / 6.23 MB
testing...
RESULT: nightly-2021-10-13, ===> No

installing nightly-2021-10-14
cargo for x86_64-unknown-linux-gnu: 6.25 MB / 6.25 MB
testing...
RESULT: nightly-2021-10-14, ===> Yes

searched toolchains nightly-2021-10-13 through nightly-2021-10-14
installing nightly-2021-10-14
testing...


********************************************************************************
Regression in nightly-2021-10-14
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-10-13/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-10-13: 40 B / 40 B
converted 2021-10-13 to d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c
fetching https://static.rust-lang.org/dist/2021-10-14/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-10-14: 40 B / 40 B
converted 2021-10-14 to dfc5add915e8bf4accbb7cf4de00351a7c6126a1
looking for regression commit between 2021-10-13 and 2021-10-14
refreshing repository at "/a/b/rust"
From https://github.com/rust-lang/rust
 * branch                    HEAD       -> FETCH_HEAD
opening existing repository at "/a/b/rust"
fetching (via local git) commits from d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c to dfc5add915e8bf4accbb7cf4de00351a7c6126a1
refreshing repository at "/a/b/rust"
From https://github.com/rust-lang/rust
 * branch                    HEAD       -> FETCH_HEAD
opening existing repository at "/a/b/rust"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2021-10-12UTC: Auto merge of #89105 - DevinR528:reachable-fix, r=Nadrieril
  commit[1] 2021-10-12UTC: Auto merge of #89774 - the8472:inline-mut-iter-next, r=m-ou-se
  commit[2] 2021-10-13UTC: Auto merge of #89802 - ehuss:update-cargo, r=ehuss
  commit[3] 2021-10-13UTC: Auto merge of #89587 - camelid:all-candidates, r=petrochenkov
  commit[4] 2021-10-13UTC: Auto merge of #89847 - JohnTitor:rollup-xfymeo4, r=JohnTitor
  commit[5] 2021-10-13UTC: Auto merge of #89822 - tmiasko:overflap-duplicates, r=cjgillot
  commit[6] 2021-10-13UTC: Auto merge of #89555 - oli-obk:nll_member_constraint_diag, r=estebank
validated commits found, specifying toolchains

installing d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c
cargo for x86_64-unknown-linux-gnu: 6.23 MB / 6.23 MB
testing...
RESULT: d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c, ===> No

installing dfc5add915e8bf4accbb7cf4de00351a7c6126a1
cargo for x86_64-unknown-linux-gnu: 6.25 MB / 6.25 MB
testing...
RESULT: dfc5add915e8bf4accbb7cf4de00351a7c6126a1, ===> Yes

installing 5728bd64b49b0e78d0180efed75ef0870ae60266
cargo for x86_64-unknown-linux-gnu: 6.25 MB / 6.25 MB
testing...
RESULT: 5728bd64b49b0e78d0180efed75ef0870ae60266, ===> Yes

installing ef4b3069baafa75bc047a80b58cc130e80576a4e
cargo for x86_64-unknown-linux-gnu: 6.23 MB / 6.23 MB
testing...
RESULT: ef4b3069baafa75bc047a80b58cc130e80576a4e, ===> No

installing a16f686e4a0ea15dcd3b5aa3db7b1cba27bb9453
cargo for x86_64-unknown-linux-gnu: 6.25 MB / 6.25 MB
testing...
RESULT: a16f686e4a0ea15dcd3b5aa3db7b1cba27bb9453, ===> Yes

searched toolchains d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c through dfc5add915e8bf4accbb7cf4de00351a7c6126a1


********************************************************************************
Regression in a16f686e4a0ea15dcd3b5aa3db7b1cba27bb9453
********************************************************************************

==================================================================================
= Please file this regression report on the rust-lang/rust GitHub repository     =
=        New issue: https://github.com/rust-lang/rust/issues/new                 =
=     Known issues: https://github.com/rust-lang/rust/issues                     =
= Copy and paste the text below into the issue report thread.  Thanks!           =
==================================================================================

searched nightlies: from nightly-2021-10-13 to nightly-2021-10-14
regressed nightly: nightly-2021-10-14
searched commit range: https://github.com/rust-lang/rust/compare/d7c97a02d1215e4ef26c31cb72dbaf16fd548b2c...dfc5add915e8bf4accbb7cf4de00351a7c6126a1
regressed commit: https://github.com/rust-lang/rust/commit/a16f686e4a0ea15dcd3b5aa3db7b1cba27bb9453
