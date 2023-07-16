plain
travis_time:end:06b2f2ea:start=1546192567350007586,finish=1546192621798181912,duration=54448174326
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:46:58]    Compiling parking_lot_core v0.3.0
[00:46:59]    Compiling parking_lot v0.6.4
[00:47:02]    Compiling tempfile v3.0.5
[00:47:03]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:47:03] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
[00:47:03]     |
[00:47:03]     |
[00:47:03] 480 |                 "fn main() { fn _inner() -> Result<(), impl std::fmt::Debug> {",
[00:47:03]     |                                                                                ^ expected one of `.`, `;`, `?`, `}`, or an operator here
[00:47:03] 
[00:47:03] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
[00:47:03]     |
[00:47:03]     |
[00:47:03] 482 |                 "fn main() { fn _inner() -> Result<(), Box<dyn std::error::Error>> {",
[00:47:03]     |                                                                                      ^ expected one of `.`, `;`, `?`, `}`, or an operator here
[00:47:08] error: aborting due to 2 previous errors
[00:47:08] 
[00:47:08] error: Could not compile `rustdoc`.
[00:47:08] 
---
[00:47:08] 
[00:47:08] 
[00:47:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:47:08] Build completed unsuccessfully in 0:43:58
[00:47:08] make: *** [all] Error 1
[00:47:08] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07783474
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 30 18:44:18 UTC 2018
