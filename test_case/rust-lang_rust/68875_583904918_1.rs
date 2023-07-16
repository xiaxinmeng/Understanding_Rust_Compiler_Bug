
checking nightly-2019-07-30
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 19.06 MB/s uninstalling nightly-2019-07-30
verifying the start of the range does not reproduce the regression
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 18.76 MB/s uninstalling nightly-2019-07-30
tested nightly-2019-07-30, got No
confirmed the start of the range does not reproduce the regression
verifying the end of the range reproduces the regression
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 19.19 MB/s uninstalling nightly-2019-07-31
tested nightly-2019-07-31, got Yes
confirmed the end of the range reproduces the regression
searched toolchains nightly-2019-07-30 through nightly-2019-07-31
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 18.37 MB/s uninstalling nightly-2019-07-31
regression in nightly-2019-07-31
fetching https://static.rust-lang.org/dist/2019-07-31/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-07-31: 40 B / 40 B [=====================================================] 100.00 % 319.54 KB/s converted 2019-07-31 to dddb7fca09dc817ba275602b950bb81a9032fb6d
fetching https://static.rust-lang.org/dist/2019-07-30/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-07-30: 40 B / 40 B [=====================================================] 100.00 % 480.83 KB/s converted 2019-07-30 to 04b88a9eba8abbac87eddcb2998beea09589c2c9
looking for regression commit between 2019-07-31 and 2019-07-30
fetching commits from 04b88a9eba8abbac87eddcb2998beea09589c2c9 to dddb7fca09dc817ba275602b950bb81a9032fb6d
cloning rust repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 4 bors merge commits in the specified range
opening existing repository at "rust.git"
refreshing repository
no commits between 04b88a9eba8abbac87eddcb2998beea09589c2c9 and dddb7fca09dc817ba275602b950bb81a9032fb6d within last 167 days
