plain
travis_time:end:294d575b:start=1548469779857298814,finish=1548469780845276571,duration=987977757
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 57.4MB/s eta 0:00:01
    99% |████████████████████���███████████| 542kB 55.8MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 26.3MB/s 
Collecting botocore==1.12.86 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d7/af/fd9c0f1f0fdc03d3367a56f35093f8b1020ba1a97ead9fa580156895944b/botocore-1.12.86-py2.py3-none-any.whl (5.2MB)
    0% |▏                               | 20kB 28.4MB/s eta 0:00:01
    0% |▏                               | 30kB 33.8MB/s eta 0:00:01
    0% |▎                               | 40kB 38.0MB/s eta 0:00:01
    0% |▎                               | 51kB 42.1MB/s eta 0:00:01
---
[00:06:24]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:24]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:28]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:47]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:48] error[E0428]: the name `DEPRECATED_IN_FUTURE` is defined multiple times
[00:07:48]    --> src/librustc/lint/mod.rs:112:9
[00:07:48]     |
[00:07:48] 112 |           $vis static $NAME: &$crate::lint::Lint = &$crate::lint::Lint {
[00:07:48]     |           |
[00:07:48]     |           |
[00:07:48]     |           `DEPRECATED_IN_FUTURE` redefined here
[00:07:48]     |           previous definition of the value `DEPRECATED_IN_FUTURE` here
[00:07:48]    ::: src/librustc/lint/builtin.rs:349:1
[00:07:48]     |
[00:07:48]     |
[00:07:48] 349 | / declare_lint! {
[00:07:48] 350 | |     pub DEPRECATED_IN_FUTURE,
[00:07:48] 351 | |     Allow,
[00:07:48] 352 | |     "detects use of items that will be deprecated in a future version",
[00:07:48] 353 | |     report_in_external_macro: true
[00:07:48]     | |_- in this macro invocation
[00:07:48]     |
[00:07:48]     |
[00:07:48]     = note: `DEPRECATED_IN_FUTURE` must be defined only once in the value namespace of this module
[00:07:53] error: unused import: `errors::DiagnosticBuilder`
[00:07:53]  --> src/librustc/traits/util.rs:1:5
[00:07:53]   |
[00:07:53] 1 | use errors::DiagnosticBuilder;
