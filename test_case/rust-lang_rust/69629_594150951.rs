
checking nightly-2020-02-28
std for x86_64-unknown-linux-gnu: 17.52 MB / 17.52 MB [==========================================================================================================================================================================================================================] 100.00 % 15.98 MB/s uninstalling nightly-2020-02-28
verifying the start of the range does not reproduce the regression
std for x86_64-unknown-linux-gnu: 17.52 MB / 17.52 MB [==========================================================================================================================================================================================================================] 100.00 % 15.78 MB/s uninstalling nightly-2020-02-28
tested nightly-2020-02-28, got No
confirmed the start of the range does not reproduce the regression
verifying the end of the range reproduces the regression
uninstalling nightly-2020-02-29
tested nightly-2020-02-29, got Yes
confirmed the end of the range reproduces the regression
searched toolchains nightly-2020-02-28 through nightly-2020-02-29
uninstalling nightly-2020-02-29
regression in nightly-2020-02-29
fetching https://static.rust-lang.org/dist/2020-02-29/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-02-29: 40 B / 40 B [======================================================================================================================================================================================================================================] 100.00 % 176.06 KB/s converted 2020-02-29 to 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
fetching https://static.rust-lang.org/dist/2020-02-28/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-02-28: 40 B / 40 B [======================================================================================================================================================================================================================================] 100.00 % 624.42 KB/s converted 2020-02-28 to 6d69caba110c0c2fb90180df1cbc8be5033b91d4
looking for regression commit between 2020-02-29 and 2020-02-28
fetching commits from 6d69caba110c0c2fb90180df1cbc8be5033b91d4 to 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
opening existing repository at "/home/jon/dev/others/rust"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
opening existing repository at "/home/jon/dev/others/rust"
refreshing repository
validated commits found, specifying toolchains
testing commits
verifying the start of the range does not reproduce the regression
installing 6d69caba110c0c2fb90180df1cbc8be5033b91d4
std for x86_64-unknown-linux-gnu: 17.52 MB / 17.52 MB [=========================================================================================================================================================================================================================] 100.00 % 713.64 KB/s testing 6d69caba110c0c2fb90180df1cbc8be5033b91d4
tested 6d69caba110c0c2fb90180df1cbc8be5033b91d4, got No
uninstalling 6d69caba110c0c2fb90180df1cbc8be5033b91d4
confirmed the start of the range does not reproduce the regression
verifying the end of the range reproduces the regression
installing 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
std for x86_64-unknown-linux-gnu: 17.53 MB / 17.53 MB [=========================================================================================================================================================================================================================] 100.00 % 605.20 KB/s testing 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
tested 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb, got No
uninstalling 0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb
thread 'main' panicked at 'the end of the range to test must reproduce the regression', /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/cargo-bisect-rustc-0.3.0/src/least_satisfying.rs:34:14
