shell
dup2rt@RNG-C-000VG:~/Temp$ cargo-bisect-rustc --test-dir=rust-bug-95825 --start=2022-01-01
checking the start range to find a passing nightly
installing nightly-2022-01-01
rust-std-nightly-x86_64-unknown-linux-gnu: 25.62 MB / 25.62 MB [======================================================================================================================] 100.00 % 5.57 MB/s testing...
RESULT: nightly-2022-01-01, ===> No
uninstalling nightly-2022-01-01

checking the end range to verify it does not pass
installing nightly-2022-03-07
testing...
RESULT: nightly-2022-03-07, ===> Yes
uninstalling nightly-2022-03-07

33 versions remaining to test after this (roughly 5 steps)
installing nightly-2022-02-02
rust-std-nightly-x86_64-unknown-linux-gnu: 25.85 MB / 25.85 MB [======================================================================================================================] 100.00 % 5.06 MB/s testing...
RESULT: nightly-2022-02-02, ===> No
uninstalling nightly-2022-02-02

17 versions remaining to test after this (roughly 4 steps)
installing nightly-2022-02-18
rust-std-nightly-x86_64-unknown-linux-gnu: 26.23 MB / 26.23 MB [======================================================================================================================] 100.00 % 5.74 MB/s testing...
RESULT: nightly-2022-02-18, ===> No
uninstalling nightly-2022-02-18

9 versions remaining to test after this (roughly 3 steps)
installing nightly-2022-02-26
rust-std-nightly-x86_64-unknown-linux-gnu: 26.42 MB / 26.42 MB [======================================================================================================================] 100.00 % 5.87 MB/s testing...
RESULT: nightly-2022-02-26, ===> Yes
uninstalling nightly-2022-02-26

4 versions remaining to test after this (roughly 2 steps)
installing nightly-2022-02-22
rust-std-nightly-x86_64-unknown-linux-gnu: 26.33 MB / 26.33 MB [======================================================================================================================] 100.00 % 5.84 MB/s testing...
RESULT: nightly-2022-02-22, ===> Yes
uninstalling nightly-2022-02-22

2 versions remaining to test after this (roughly 1 steps)
installing nightly-2022-02-20
rust-std-nightly-x86_64-unknown-linux-gnu: 26.18 MB / 26.18 MB [======================================================================================================================] 100.00 % 5.77 MB/s testing...
RESULT: nightly-2022-02-20, ===> Yes
uninstalling nightly-2022-02-20

1 versions remaining to test after this (roughly 1 steps)
installing nightly-2022-02-19
rust-std-nightly-x86_64-unknown-linux-gnu: 26.24 MB / 26.24 MB [======================================================================================================================] 100.00 % 5.89 MB/s testing...
RESULT: nightly-2022-02-19, ===> No
uninstalling nightly-2022-02-19

searched toolchains nightly-2022-01-01 through nightly-2022-03-07


********************************************************************************
Regression in nightly-2022-02-20
********************************************************************************

fetching https://static.rust-lang.org/dist/2022-02-19/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-02-19: 40 B / 40 B [==========================================================================================================================================] 100.00 % 538.82 KB/s converted 2022-02-19 to b17226fcc11587fed612631be372a5b4cb89988a
fetching https://static.rust-lang.org/dist/2022-02-20/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-02-20: 40 B / 40 B [==========================================================================================================================================] 100.00 % 483.37 KB/s converted 2022-02-20 to 3b348d932aa5c9884310d025cf7c516023fd0d9a
looking for regression commit between 2022-02-19 and 2022-02-20
opening existing repository at "/home/dup2rt/Temp/rust"
Found origin remote under name `origin`
refreshing repository at "/home/dup2rt/Temp/rust"
fetching (via local git) commits from b17226fcc11587fed612631be372a5b4cb89988a to 3b348d932aa5c9884310d025cf7c516023fd0d9a
opening existing repository at "/home/dup2rt/Temp/rust"
Found origin remote under name `origin`
refreshing repository at "/home/dup2rt/Temp/rust"
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2022-02-18UTC: Auto merge of #94121 - matthiaskrgr:rollup-6ps95da, r=matthiaskrgr
  commit[1] 2022-02-18UTC: Auto merge of #94050 - michaelwoerister:fix-unsized-tuple-debuginfo, r=pnkfelix
  commit[2] 2022-02-19UTC: Auto merge of #94134 - matthiaskrgr:rollup-b132kjz, r=matthiaskrgr
  commit[3] 2022-02-19UTC: Auto merge of #94105 - 5225225:destabilise-entry-insert, r=Mark-Simulacrum
  commit[4] 2022-02-19UTC: Auto merge of #94148 - matthiaskrgr:rollup-jgea68f, r=matthiaskrgr
  commit[5] 2022-02-19UTC: Auto merge of #94165 - Mark-Simulacrum:bump-version, r=Mark-Simulacrum
ERROR: no CI builds available between b17226fcc11587fed612631be372a5b4cb89988a and 3b348d932aa5c9884310d025cf7c516023fd0d9a within last 167 days
dup2rt@RNG-C-000VG:~/Temp$ 
