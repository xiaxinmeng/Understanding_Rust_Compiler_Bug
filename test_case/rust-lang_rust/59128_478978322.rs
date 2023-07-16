plain
travis_time:end:015eff64:start=1554205101004927059,finish=1554205224715576853,duration=123710649794
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:03:45] travis_fold:end:stage2-rustdoc

[01:03:45] travis_time:end:stage2-rustdoc:start=1554209059593551596,finish=1554209060009576550,duration=416024954

[01:03:45] thread 'rustc' panicked at 'No option 'json-rendered' defined', /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.17/src/lib.rs:767:21
[01:03:45] 
[01:03:45] 
[01:03:45] 
[01:03:45] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/favicon.inc" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "/checkout/src/doc/guide-strings.md" "--markdown-css" "rust.css"
[01:03:45] 
[01:03:45] 
[01:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:03:45] Build completed unsuccessfully in 0:04:51
