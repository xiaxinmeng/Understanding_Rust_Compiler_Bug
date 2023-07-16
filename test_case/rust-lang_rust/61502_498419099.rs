plain
travis_time:end:25f68d56:start=1559590812698968110,finish=1559590813475907001,duration=776938891
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:26] 
[00:56:26] running 2923 tests
[00:56:37] .................................................................................................... 100/2923
[00:56:48] .............................................................................i...................... 200/2923
[00:56:57] ......F............................................................................................. 300/2923
[00:57:16] .................................................................................................... 500/2923
[00:57:27] .................................................................................................... 600/2923
[00:57:41] .................................................................................................... 700/2923
[00:57:52] .................................................................................................... 800/2923
---
[01:02:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/backtrace-debuginfo/a"
[01:02:37] stdout:
[01:02:37] ------------------------------------------
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 0
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:70:5
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f13eaeb7692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f13eaeb7329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f13eaeb7ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x56191b213284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x56191b2079db - backtrace_debuginfo::inner::h5960412c1c9c41c9
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:70
[01:02:37]    5:     0x56191b2082f4 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    6:     0x56191b208f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    7:     0x56191b1fd502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    8:     0x7f13eaeb7933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]    9:     0x7f13eaecc00a - __rust_maybe_catch_panic
[01:02:37]   10:     0x7f13eaeb8610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   11:     0x56191b1fd4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   12:     0x56191b209025 - main
[01:02:37]   13:     0x7f13ea87e830 - __libc_start_main
[01:02:37]   14:     0x56191b1f1689 - _start
[01:02:37]   15:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 1
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:71:5
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f2a031d6692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f2a031d6329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f2a031d6ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55d368a52284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55d368a46a9f - backtrace_debuginfo::inner::h5960412c1c9c41c9
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:71
[01:02:37]    5:     0x55d368a472f4 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    6:     0x55d368a47f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    7:     0x55d368a3c502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    8:     0x7f2a031d6933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]    9:     0x7f2a031eb00a - __rust_maybe_catch_panic
[01:02:37]   10:     0x7f2a031d7610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   11:     0x55d368a3c4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]   12:     0x55d368a48025 - main
[01:02:37]   12:     0x55d368a48025 - main
[01:02:37]   13:     0x7f2a02b9d830 - __libc_start_main
[01:02:37]   14:     0x55d368a30689 - _start
[01:02:37]   15:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109", "backtrace-debuginfo.rs:72", "backtrace-debuginfo-aux.rs:5"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo-aux.rs:5
[01:02:37] backtrace-debuginfo.rs:72
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 2
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:73:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7fc1421f1692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7fc1421f1329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7fc1421f1ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55ff1d784284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55ff1d771024 - backtrace_debuginfo::inner::{{closure}}::h013e19dbf8bbb66d
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:73
[01:02:37]    5:     0x55ff1d770ee9 - backtrace_debuginfo::aux::callback::h813aaf4dc912792b
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo-aux.rs:6
[01:02:37]    6:     0x55ff1d778b03 - backtrace_debuginfo::inner::h5960412c1c9c41c9
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    7:     0x55ff1d7792f4 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    8:     0x55ff1d779f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    9:     0x55ff1d76e502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]   10:     0x7fc1421f1933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   11:     0x7fc14220600a - __rust_maybe_catch_panic
[01:02:37]   12:     0x7fc1421f2610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   13:     0x55ff1d76e4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   14:     0x55ff1d77a025 - main
[01:02:37]   15:     0x7fc141bb8830 - __libc_start_main
[01:02:37]   16:     0x55ff1d762689 - _start
[01:02:37]   17:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:109", "backtrace-debuginfo.rs:75", "backtrace-debuginfo-aux.rs:12"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo-aux.rs:12
[01:02:37] backtrace-debuginfo.rs:75
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:109
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 3
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:76:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f6a29413692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f6a29413329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f6a29413ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x5614089f9284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x5614089edca4 - backtrace_debuginfo::inner::{{closure}}::h47ae4ba11ec0603d
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:76
[01:02:37]    5:     0x5614089ed6e9 - backtrace_debuginfo::aux::callback_inlined::h3c6f039a1ff39948
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo-aux.rs:13
[01:02:37]    6:     0x5614089edb59 - backtrace_debuginfo::inner::h5960412c1c9c41c9
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:78
[01:02:37]    7:     0x5614089ee2f4 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    8:     0x5614089eef7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    9:     0x5614089e3502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]   10:     0x7f6a29413933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   11:     0x7f6a2942800a - __rust_maybe_catch_panic
[01:02:37]   12:     0x7f6a29414610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   13:     0x5614089e34db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   14:     0x5614089ef025 - main
[01:02:37]   15:     0x7f6a28dda830 - __libc_start_main
[01:02:37]   16:     0x5614089d7689 - _start
[01:02:37]   17:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 4
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:84:5
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7fb33787e692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7fb33787e329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7fb33787eef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x5606751fd284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x5606751f1d9b - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:84
[01:02:37]    5:     0x5606751f2336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    6:     0x5606751f2f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    7:     0x5606751e7502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    8:     0x7fb33787e933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]    9:     0x7fb33789300a - __rust_maybe_catch_panic
[01:02:37]   10:     0x7fb33787f610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   11:     0x5606751e74db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]   12:     0x5606751f3025 - main
[01:02:37]   12:     0x5606751f3025 - main
[01:02:37]   13:     0x7fb337245830 - __libc_start_main
[01:02:37]   14:     0x5606751db689 - _start
[01:02:37]   15:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 5
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:85:5
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7ff9785ef692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7ff9785ef329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7ff9785efef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55faad6f4284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55faad6e8e5f - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:85
[01:02:37]    5:     0x55faad6e9336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    6:     0x55faad6e9f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    7:     0x55faad6de502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    8:     0x7ff9785ef933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]    9:     0x7ff97860400a - __rust_maybe_catch_panic
[01:02:37]   10:     0x7ff9785f0610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   11:     0x55faad6de4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   12:     0x55faad6ea025 - main
[01:02:37]   13:     0x7ff977fb6830 - __libc_start_main
[01:02:37]   14:     0x55faad6d2689 - _start
[01:02:37]   15:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:93"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:93
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 6
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:91:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f120fafc692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f120fafc329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f120fafcef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55f2b61c2284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55f2b61b7278 - backtrace_debuginfo::inner_inlined::inner_further_inlined::h9ea2481ab76e81ad
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:91
[01:02:37]    5:     0x55f2b61b6eee - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    6:     0x55f2b61b7336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    7:     0x55f2b61b7f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    8:     0x55f2b61ac502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    9:     0x7f120fafc933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   10:     0x7f120fb1100a - __rust_maybe_catch_panic
[01:02:37]   11:     0x7f120fafd610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   12:     0x55f2b61ac4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   13:     0x55f2b61b8025 - main
[01:02:37]   14:     0x7f120f4c3830 - __libc_start_main
[01:02:37]   15:     0x55f2b61a0689 - _start
[01:02:37]   16:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:95", "backtrace-debuginfo-aux.rs:5"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo-aux.rs:5
[01:02:37] backtrace-debuginfo.rs:95
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 7
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:96:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f5f89832692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f5f89832329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f5f89832ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x561246b2e284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x561246b1b184 - backtrace_debuginfo::inner_inlined::{{closure}}::h62023ed1a359e296
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:96
[01:02:37]    5:     0x561246b1ae79 - backtrace_debuginfo::aux::callback::h5834211fa2110840
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo-aux.rs:6
[01:02:37]    6:     0x561246b22f44 - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    7:     0x561246b23336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    8:     0x561246b23f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    9:     0x561246b18502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]   10:     0x7f5f89832933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   11:     0x7f5f8984700a - __rust_maybe_catch_panic
[01:02:37]   12:     0x7f5f89833610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   13:     0x561246b184db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   14:     0x561246b24025 - main
[01:02:37]   15:     0x7f5f891f9830 - __libc_start_main
[01:02:37]   16:     0x561246b0c689 - _start
[01:02:37]   17:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:98", "backtrace-debuginfo-aux.rs:12"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo-aux.rs:12
[01:02:37] backtrace-debuginfo.rs:98
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 8
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:99:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f9421416692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f9421416329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f9421416ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55cdb6a07284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55cdb69fc164 - backtrace_debuginfo::inner_inlined::{{closure}}::h91f8418b8ede9235
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:99
[01:02:37]    5:     0x55cdb69fb759 - backtrace_debuginfo::aux::callback_inlined::hdeb85b55da94b65a
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo-aux.rs:13
[01:02:37]    6:     0x55cdb69fbf9a - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:0
[01:02:37]    7:     0x55cdb69fc336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    8:     0x55cdb69fcf7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    9:     0x55cdb69f1502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]   10:     0x7f9421416933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   11:     0x7f942142b00a - __rust_maybe_catch_panic
[01:02:37]   12:     0x7f9421417610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   13:     0x55cdb69f14db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   14:     0x55cdb69fd025 - main
[01:02:37]   15:     0x7f9420ddd830 - __libc_start_main
[01:02:37]   16:     0x55cdb69e5689 - _start
[01:02:37]   17:                0x0 - <unknown>
[01:02:37] ---------------------------------------
[01:02:37] trace does not match position list
[01:02:37] trace does not match position list
[01:02:37] still need to find ["backtrace-debuginfo.rs:173", "backtrace-debuginfo.rs:110", "backtrace-debuginfo.rs:104"]
[01:02:37] --- stdout
[01:02:37] backtrace-debuginfo.rs:104
[01:02:37] backtrace-debuginfo.rs:110
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] backtrace-debuginfo.rs:173
[01:02:37] 
[01:02:37] --- stderr
[01:02:37] test case 9
[01:02:37] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:91:9
[01:02:37] stack backtrace:
[01:02:37]    0:     0x7f1ade8d7692 - std::panicking::default_hook::{{closure}}::h647fc01ccf61331c
[01:02:37]    1:     0x7f1ade8d7329 - std::panicking::default_hook::hdc7b137fa5bc1063
[01:02:37]    2:     0x7f1ade8d7ef7 - std::panicking::rust_panic_with_hook::hffed9450e623a9f0
[01:02:37]    3:     0x55599f9d4284 - std::panicking::begin_panic::h3c75d33182b1845b
[01:02:37]                                at /checkout/src/libstd/panicking.rs:408
[01:02:37]    4:     0x55599f9c9278 - backtrace_debuginfo::inner_inlined::inner_further_inlined::h9ea2481ab76e81ad
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:91
[01:02:37]    5:     0x55599f9c901b - backtrace_debuginfo::inner_inlined::h2aaf2e75a07e5d51
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:105
[01:02:37]    6:     0x55599f9c9336 - backtrace_debuginfo::outer::hcda8fac191911d33
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:111
[01:02:37]    7:     0x55599f9c9f7d - backtrace_debuginfo::main::h6764e0bb4e6bcac0
[01:02:37]                                at /checkout/src/test/run-pass/backtrace-debuginfo.rs:173
[01:02:37]    8:     0x55599f9be502 - std::rt::lang_start::{{closure}}::h486d64aa5a2eacf6
[01:02:37]                                at /checkout/src/libstd/rt.rs:64
[01:02:37]    9:     0x7f1ade8d7933 - std::panicking::try::do_call::h832d6c97476bdfe6
[01:02:37]   10:     0x7f1ade8ec00a - __rust_maybe_catch_panic
[01:02:37]   11:     0x7f1ade8d8610 - std::rt::lang_start_internal::h09633665d3ba56b3
[01:02:37]   12:     0x55599f9be4db - std::rt::lang_start::h9154944efa1ad5ce
[01:02:37]                                at /checkout/src/libstd/rt.rs:65
[01:02:37]   13:     0x55599f9ca025 - main
[01:02:37]   14:     0x7f1ade29e830 - __libc_start_main
[01:02:37]   15:     0x55599f9b2689 - _start
[01:02:37]   16:                0x0 - <unknown>
[01:02:37] 
[01:02:37] ------------------------------------------
[01:02:37] stderr:
[01:02:37] ------------------------------------------
[01:02:37] ------------------------------------------
[01:02:37] thread 'main' panicked at 'found some errors', /checkout/src/test/run-pass/backtrace-debuginfo.rs:163:9
[01:02:37] 
[01:02:37] ------------------------------------------
[01:02:37] 
[01:02:37] 
---
[01:02:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:02:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:37] 
[01:02:37] 
[01:02:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:37] 
[01:02:37] 
[01:02:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:37] Build completed unsuccessfully in 0:58:27
