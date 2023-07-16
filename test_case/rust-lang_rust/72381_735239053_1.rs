
$ cargo bisect-rustc  --script=./script.sh -v --preserve --start=2019-05-01
installing nightly-2019-05-01
testing...
RESULT: nightly-2019-05-01, ===> No

installing nightly-2020-11-28
cargo for x86_64-unknown-linux-gnu: 5.25 MB / 5.25 MB [=================================================================================================================] 100.00 % 10.61 MB/s testing...
RESULT: nightly-2020-11-28, ===> Yes

installing nightly-2020-02-13
cargo for x86_64-unknown-linux-gnu: 5.03 MB / 5.03 MB [==================================================================================================================] 100.00 % 2.68 MB/s testing...
RESULT: nightly-2020-02-13, ===> Yes

installing nightly-2019-09-22
cargo for x86_64-unknown-linux-gnu: 4.68 MB / 4.68 MB [==================================================================================================================] 100.00 % 3.97 MB/s testing...
RESULT: nightly-2019-09-22, ===> No

installing nightly-2019-12-03
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 3.33 MB/s testing...
RESULT: nightly-2019-12-03, ===> Yes

installing nightly-2019-10-28
cargo for x86_64-unknown-linux-gnu: 4.69 MB / 4.69 MB [==================================================================================================================] 100.00 % 3.81 MB/s testing...
RESULT: nightly-2019-10-28, ===> No

installing nightly-2019-11-15
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 2.33 MB/s testing...
RESULT: nightly-2019-11-15, ===> Yes

installing nightly-2019-11-06
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 4.07 MB/s testing...
RESULT: nightly-2019-11-06, ===> No

installing nightly-2019-11-10
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 3.94 MB/s testing...
RESULT: nightly-2019-11-10, ===> No

installing nightly-2019-11-12
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 4.01 MB/s testing...
RESULT: nightly-2019-11-12, ===> No

installing nightly-2019-11-13
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 3.98 MB/s testing...
RESULT: nightly-2019-11-13, ===> No

installing nightly-2019-11-14
cargo for x86_64-unknown-linux-gnu: 4.70 MB / 4.70 MB [==================================================================================================================] 100.00 % 4.02 MB/s testing...
RESULT: nightly-2019-11-14, ===> No

searched toolchains nightly-2019-05-01 through nightly-2020-11-28


********************************************************************************
Regression in nightly-2019-11-15
********************************************************************************

fetching https://static.rust-lang.org/dist/2019-11-14/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-11-14: 40 B / 40 B [=============================================================================================================================] 100.00 % 490.60 KB/s converted 2019-11-14 to ded5ee0013f6126f885baf5e072c20ba8b86ee6a
fetching https://static.rust-lang.org/dist/2019-11-15/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-11-15: 40 B / 40 B [=============================================================================================================================] 100.00 % 490.24 KB/s converted 2019-11-15 to 82cf3a4486bc882207a09bf0d9e2dea4632781aa
looking for regression commit between 2019-11-14 and 2019-11-15
cloning rust repository
fetching (via local git) commits from ded5ee0013f6126f885baf5e072c20ba8b86ee6a to 82cf3a4486bc882207a09bf0d9e2dea4632781aa
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2019-11-13UTC: Auto merge of #66170 - ecstatic-morse:hir-const-check, r=Centril,oli-obk
  commit[1] 2019-11-13UTC: Auto merge of #66211 - kinnison:kinnison/fix-66159, r=GuillaumeGomez
  commit[2] 2019-11-14UTC: Auto merge of #66233 - cjgillot:constkind, r=oli-obk
  commit[3] 2019-11-14UTC: Auto merge of #66403 - JohnTitor:rollup-7obuivl, r=JohnTitor
  commit[4] 2019-11-14UTC: Auto merge of #66378 - rkruppe:revert-pr-65134, r=pnkfelix
  commit[5] 2019-11-14UTC: Auto merge of #66314 - GuillaumeGomez:move-error-codes, r=Centril
ERROR: no commits between ded5ee0013f6126f885baf5e072c20ba8b86ee6a and 82cf3a4486bc882207a09bf0d9e2dea4632781aa within last 167 days
