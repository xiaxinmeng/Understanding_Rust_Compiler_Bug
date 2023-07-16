plain
travis_time:end:000ff1e0:start=1557579584667717974,finish=1557579671563465249,duration=86895747275
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:37] 34 |     Nan,
[00:27:37]    |     ^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]   --> src/libcore/num/dec2flt/parse.rs:40:5
[00:27:37] 40 |     ShortcutToInf,
[00:27:37]    |     ^^^^^^^^^^^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]    --> src/libcore/ops/range.rs:696:5
[00:27:37]     |
[00:27:37] 696 |     Unbounded,
[00:27:37] 696 |     Unbounded,
[00:27:37]     |     ^^^^^^^^^
[00:27:37]     |
[00:27:37]     = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]     = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]    --> src/libcore/char/mod.rs:246:5
[00:27:37]     |
[00:27:37] 246 |     Done,
[00:27:37] 246 |     Done,
[00:27:37]     |     ^^^^
[00:27:37]     |
[00:27:37]     = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]     = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]    --> src/libcore/char/mod.rs:436:5
[00:27:37]     |
[00:27:37] 436 |     Zero,
[00:27:37] 436 |     Zero,
[00:27:37]     |     ^^^^
[00:27:37]     |
[00:27:37]     = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]     = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]    --> src/libcore/option.rs:153:5
[00:27:37]     |
[00:27:37] 153 |     None,
[00:27:37] 153 |     None,
[00:27:37]     |     ^^^^
[00:27:37]     |
[00:27:37]     = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]     = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]   --> src/libcore/str/pattern.rs:79:5
[00:27:37]    |
[00:27:37] 79 |     Done
[00:27:37] 79 |     Done
[00:27:37]    |     ^^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]   --> src/libcore/fmt/rt/v1.rs:40:5
[00:27:37] 40 |     NextParam,
[00:27:37]    |     ^^^^^^^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]   --> src/libcore/fmt/rt/v1.rs:46:5
[00:27:37] 46 |     Next,
[00:27:37]    |     ^^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37] error[E0658]: discriminator values can only be used with a field-less enum
[00:27:37]   --> src/libcore/task/poll.rs:25:5
[00:27:37] 25 |     Pending,
[00:27:37]    |     ^^^^^^^
[00:27:37]    |
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = note: for more information, see https://github.com/rust-lang/rust/issues/60553
[00:27:37]    = help: add #![feature(arbitrary_enum_discriminant)] to the crate attributes to enable
[00:27:39]    Compiling compiler_builtins v0.1.12
[00:27:39]    Compiling cmake v0.1.38
[00:27:39]    Compiling backtrace-sys v0.1.27
[00:27:41]    Compiling std v0.0.0 (/checkout/src/libstd)
---
travis_time:end:0d073066:start=1557581358570226031,finish=1557581358576864611,duration=6638580
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:149f940c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:015bde90
travis_time:start:015bde90
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:186ba8ce
$ dmesg | grep -i kill
