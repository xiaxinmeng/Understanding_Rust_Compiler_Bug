plain
2019-08-20T10:09:45.0039514Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T10:09:45.0252794Z ##[command]git config gc.auto 0
2019-08-20T10:09:45.0312309Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T10:09:45.0375760Z ##[command]git config --get-all http.proxy
2019-08-20T10:09:45.0526383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63521/merge:refs/remotes/pull/63521/merge
---
2019-08-20T10:10:20.9623488Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T10:10:20.9625895Z 
2019-08-20T10:10:20.9628208Z   git checkout -b <new-branch-name>
2019-08-20T10:10:20.9629644Z 
2019-08-20T10:10:20.9630644Z HEAD is now at 0f15ce8d9 Merge e500fc3171a899f713a269ccee6f480b9db4859b into 7858dc237d70fc0c5a31eb528dfab1ad0baf6a27
2019-08-20T10:10:20.9781401Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T10:10:20.9783942Z ==============================================================================
2019-08-20T10:10:20.9784004Z Task         : Bash
2019-08-20T10:10:20.9784059Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T10:13:55.7473728Z 
2019-08-20T10:13:55.7475283Z ######################################################################## 100.0%
2019-08-20T10:13:56.1084341Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-20T10:13:56.1843328Z     Updating crates.io index
2019-08-20T10:14:28.2085061Z error: checksum for `libc v0.2.61` changed between lock files
2019-08-20T10:14:28.2086167Z this could be indicative of a few possible errors:
2019-08-20T10:14:28.2086871Z 
2019-08-20T10:14:28.2087094Z     * the lock file is corrupt
2019-08-20T10:14:28.2087094Z     * the lock file is corrupt
2019-08-20T10:14:28.2087317Z     * a replacement source in use (e.g., a mirror) returned a different checksum
2019-08-20T10:14:28.2087604Z     * the source itself may be corrupt in one way or another
2019-08-20T10:14:28.2088012Z unable to verify that `libc v0.2.61` is the same as when the lockfile was generated
2019-08-20T10:14:28.2088259Z 
2019-08-20T10:14:28.2088259Z 
2019-08-20T10:14:28.2172258Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-20T10:14:28.2175382Z == clock drift check ==
2019-08-20T10:14:28.2185357Z   local time: Tue Aug 20 10:14:28 UTC 2019
2019-08-20T10:14:28.2535840Z   network time: Tue, 20 Aug 2019 10:14:28 GMT
2019-08-20T10:14:28.2537766Z == end clock drift check ==
2019-08-20T10:14:28.2537766Z == end clock drift check ==
2019-08-20T10:14:34.4128967Z ##[error]Bash exited with code '1'.
2019-08-20T10:14:34.4167694Z ##[section]Starting: Checkout
2019-08-20T10:14:34.4169281Z ==============================================================================
2019-08-20T10:14:34.4169323Z Task         : Get sources
2019-08-20T10:14:34.4169359Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
