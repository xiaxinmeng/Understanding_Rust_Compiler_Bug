
cargo bisect-rustc --start 2019-07-30 --end 2019-07-31 --prompt
checking nightly-2019-07-30
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [====================================] 100.00 % 17.04 MB/s 0s    Compiling bisectit v0.7.0 (/home/chris/rust-testing/bisectit)
error: Unrecognized option: 'json'

error: could not compile `bisectit`.

To learn more, run the command again with --verbose.


nightly-2019-07-30 finished with exit code Some(101).
please select an action to take:
uninstalling nightly-2019-07-30
verifying the start of the range does not reproduce the regression
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 18.00 MB/s    Compiling bisectit v0.7.0 (/home/chris/rust-testing/bisectit)
error: Unrecognized option: 'json'

error: could not compile `bisectit`.

To learn more, run the command again with --verbose.


nightly-2019-07-30 finished with exit code Some(101).
please select an action to take:
uninstalling nightly-2019-07-30
tested nightly-2019-07-30, got No
confirmed the start of the range does not reproduce the regression
verifying the end of the range reproduces the regression
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 17.82 MB/s    Compiling bisectit v0.7.0 (/home/chris/rust-testing/bisectit)
warning: field is never used: `f`
  --> src/main.rs:12:5
   |
12 |     f: F
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default


thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `bisectit`.

Caused by:
  process didn't exit successfully: `rustc --crate-name bisectit --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=681c4a9b335b4202 -C extra-filename=-681c4a9b335b4202 --out-dir /home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/deps -C incremental=/home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/incremental -L dependency=/home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/deps` (signal: 6, SIGABRT: process abort signal)


nightly-2019-07-31 finished with exit code Some(101).
please select an action to take:
uninstalling nightly-2019-07-31
tested nightly-2019-07-31, got Yes
confirmed the end of the range reproduces the regression
searched toolchains nightly-2019-07-30 through nightly-2019-07-31
std for x86_64-unknown-linux-gnu: 170.30 MB / 170.30 MB [=======================================] 100.00 % 17.69 MB/s    Compiling bisectit v0.7.0 (/home/chris/rust-testing/bisectit)
warning: field is never used: `f`
  --> src/main.rs:12:5
   |
12 |     f: F
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default


thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `bisectit`.

Caused by:
  process didn't exit successfully: `rustc --crate-name bisectit --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=681c4a9b335b4202 -C extra-filename=-681c4a9b335b4202 --out-dir /home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/deps -C incremental=/home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/incremental -L dependency=/home/chris/rust-testing/bisectit/target-bisector-nightly-2019-07-31-x86_64-unknown-linux-gnu/debug/deps` (signal: 6, SIGABRT: process abort signal)


nightly-2019-07-31 finished with exit code Some(101).
please select an action to take:
uninstalling nightly-2019-07-31
regression in nightly-2019-07-31
fetching https://static.rust-lang.org/dist/2019-07-31/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-07-31: 40 B / 40 B [=====================================================] 100.00 % 266.46 KB/s converted 2019-07-31 to dddb7fca09dc817ba275602b950bb81a9032fb6d
fetching https://static.rust-lang.org/dist/2019-07-30/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2019-07-30: 40 B / 40 B [=====================================================] 100.00 % 473.40 KB/s converted 2019-07-30 to 04b88a9eba8abbac87eddcb2998beea09589c2c9
looking for regression commit between 2019-07-31 and 2019-07-30
fetching commits from 04b88a9eba8abbac87eddcb2998beea09589c2c9 to dddb7fca09dc817ba275602b950bb81a9032fb6d
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 4 bors merge commits in the specified range
opening existing repository at "rust.git"
refreshing repository
no commits between 04b88a9eba8abbac87eddcb2998beea09589c2c9 and dddb7fca09dc817ba275602b950bb81a9032fb6d within last 167 days
