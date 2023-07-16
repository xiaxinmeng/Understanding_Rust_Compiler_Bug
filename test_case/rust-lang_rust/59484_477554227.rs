plain
travis_time:end:00980bcd:start=1553770370576068829,finish=1553770371562128940,duration=986060111
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:27:37]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:27:42]    Compiling compiler_builtins v0.1.5
[00:27:42]    Compiling cmake v0.1.33
[00:27:42]    Compiling backtrace-sys v0.1.27
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 3924 |             b'A'...b'Z' | b'a'...b'z' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42]      |
[00:27:42]      = note: #[deny(ellipsis_inclusive_range_patterns)] on by default
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 3924 |             b'A'...b'Z' | b'a'...b'z' => true,
[00:27:42]      |                               ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 3959 |             b'A'...b'Z' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 3994 |             b'a'...b'z' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4032 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4032 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:27:42]      |                               ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4032 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:27:42]      |                                             ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4067 |             b'0'...b'9' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4105 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4105 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:27:42]      |                               ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4105 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:27:42]      |                                             ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4144 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4144 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:27:42]      |                               ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4144 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:27:42]      |                                             ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4144 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:27:42]      |                                                           ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4179 |             b'!'...b'~' => true,
[00:27:42]      |                 ^^^ help: use `..=` for an inclusive range
[00:27:42] 
[00:27:42] error: `...` range patterns are deprecated
[00:27:42]      |
[00:27:42]      |
[00:27:42] 4268 |             b'\0'...b'\x1F' | b'\x7F' => true,
[00:27:42]      |                  ^^^ help: use `..=` for an inclusive range
[00:27:45]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:27:45]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:46]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:46]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
