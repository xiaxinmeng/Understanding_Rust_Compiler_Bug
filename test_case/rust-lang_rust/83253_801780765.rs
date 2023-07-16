
********************************************************************************
Regression in nightly-2021-03-17
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-03-16/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-03-16: 40 B / 40 B [===============] 100.00 % 697.91 KB/s converted 2021-03-16 to 107896c32d5dda4db508968ff34997a39d286966
fetching https://static.rust-lang.org/dist/2021-03-17/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-03-17: 40 B / 40 B [===============] 100.00 % 705.85 KB/s converted 2021-03-17 to f5d8117c338a788bd24abec733fd143dfceb25a0
looking for regression commit between 2021-03-16 and 2021-03-17
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from 107896c32d5dda4db508968ff34997a39d286966 to f5d8117c338a788bd24abec733fd143dfceb25a0
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2021-03-15UTC: Auto merge of #83121 - the8472:env-rwlock-2, r=joshtriplett
  commit[1] 2021-03-16UTC: Auto merge of #83153 - Aaron1011:eval-always-extern_mod_stmt_cnum, r=michaelwoerister
  commit[2] 2021-03-16UTC: Auto merge of #82898 - oli-obk:tait_🧊, r=nikomatsakis
  commit[3] 2021-03-16UTC: Auto merge of #82838 - Amanieu:rustdoc_asm, r=nagisa
  commit[4] 2021-03-16UTC: Auto merge of #83199 - JohnTitor:rollup-zrfk94a, r=JohnTitor
  commit[5] 2021-03-16UTC: Auto merge of #82536 - sexxi-goose:handle-patterns-take-2, r=nikomatsakis
validated commits found, specifying toolchains

installing 107896c32d5dda4db508968ff34997a39d286966
rustc for x86_64-unknown-linux-gnu: 1003.49 KB / 49.11 MB  2.00 % 881.96 KB/s 56rustc for x86_64-unknown-linux-gnu: 1004.49 KB / 49.11 MB  2.00 % 882.53 KB/s 56rustc for x86_64-unknown-linux-gnu: 1012.49 KB / 49.11 MB  2.01 % 889.51 KB/s 55rustc for x86_64-unknown-linux-gnu: 1020.49 KB / 49.11 MB  2.03 % 896.23 KB/s 55rustc for x86_64-unknown-linux-gnu: 1021.49 KB / 49.11 MB  2.03 % 896.79 KB/s 55cargo for x86_64-unknown-linux-gnu: 5.96 MB / 5.96 MB [====] 100.00 % 7.31 MB/s testing...
RESULT: 107896c32d5dda4db508968ff34997a39d286966, ===> No
uninstalling 107896c32d5dda4db508968ff34997a39d286966

installing f5d8117c338a788bd24abec733fd143dfceb25a0
cargo for x86_64-unknown-linux-gnu: 5.96 MB / 5.96 MB [====] 100.00 % 8.68 MB/s testing...
RESULT: f5d8117c338a788bd24abec733fd143dfceb25a0, ===> No
uninstalling f5d8117c338a788bd24abec733fd143dfceb25a0

ERROR: the commit at the end of the range (f5d8117c338a788bd24abec733fd143dfceb25a0) does not reproduce the regression
