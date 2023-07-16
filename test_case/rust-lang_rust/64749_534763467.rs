plain
2019-09-24T21:12:07.1854886Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T21:12:07.2094534Z ##[command]git config gc.auto 0
2019-09-24T21:12:07.2194518Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T21:12:07.2232140Z ##[command]git config --get-all http.proxy
2019-09-24T21:12:07.2419111Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64749/merge:refs/remotes/pull/64749/merge
---
2019-09-24T21:17:46.5809076Z  ---> 5a9c4155b013
2019-09-24T21:17:46.5856808Z Successfully built 5a9c4155b013
2019-09-24T21:17:46.6923270Z Successfully tagged rust-ci:latest
2019-09-24T21:17:46.8072642Z Built container sha256:5a9c4155b01378641558d1b5ccac3f8603bfb435b79e65afffd9d532e91397cd
2019-09-24T21:17:46.8096663Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/d75f4159e94a655bec830f29c217435ec5e0ac916ddc4020533e7b87fe06a0623bf7377f6d5cd1ca2fd627e09081e1dfacce244379f5b5e14a8d0757d0290bdc
2019-09-24T21:18:50.3500241Z upload failed: - to s3://rust-lang-ci-sccache2/docker/d75f4159e94a655bec830f29c217435ec5e0ac916ddc4020533e7b87fe06a0623bf7377f6d5cd1ca2fd627e09081e1dfacce244379f5b5e14a8d0757d0290bdc An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-09-24T21:18:51.8368295Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-09-24T21:18:51.8398324Z == clock drift check ==
2019-09-24T21:18:51.8409057Z   local time: Tue Sep 24 21:18:51 UTC 2019
2019-09-24T21:18:52.0020551Z   network time: Tue, 24 Sep 2019 21:18:51 GMT
---
2019-09-24T21:51:31.0118114Z    Compiling tempfile v3.1.0
2019-09-24T21:51:33.2300113Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-09-24T21:51:34.6162289Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-09-24T21:51:45.1567748Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-24T21:51:46.1810241Z thread 'rustc' panicked at 'assertion failed: elem.index() < self.domain_size', src/librustc_data_structures/bit_set.rs:369:9
2019-09-24T21:51:46.1810516Z 
2019-09-24T21:51:46.1810567Z error: internal compiler error: unexpected panic
2019-09-24T21:51:46.1810599Z 
2019-09-24T21:51:46.1810667Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-24T21:51:46.1810667Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-24T21:51:46.1810700Z 
2019-09-24T21:51:46.1811331Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-24T21:51:46.1811672Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-09-24T21:51:46.1811705Z 
2019-09-24T21:51:46.1811705Z 
2019-09-24T21:51:46.1812123Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-09-24T21:51:46.1812239Z note: some of the compiler flags provided by cargo are hidden
2019-09-24T21:51:46.1812272Z 
2019-09-24T21:51:46.2089431Z error: Could not compile `syntax_pos`.
2019-09-24T21:51:46.2089572Z 
---
2019-09-24T21:51:46.2200580Z == clock drift check ==
2019-09-24T21:51:46.2200654Z   local time: Tue Sep 24 21:51:46 UTC 2019
2019-09-24T21:51:46.3698524Z   network time: Tue, 24 Sep 2019 21:51:46 GMT
2019-09-24T21:51:46.3701672Z == end clock drift check ==
2019-09-24T21:51:47.6917034Z ##[error]Bash exited with code '1'.
2019-09-24T21:51:47.6958134Z ##[section]Starting: Checkout
2019-09-24T21:51:47.6961632Z ==============================================================================
2019-09-24T21:51:47.6961691Z Task         : Get sources
2019-09-24T21:51:47.6961737Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
