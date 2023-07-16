plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/dd/2b/de181f8a22c204a832c96c0929336cc4aad128b04d63351ae21604be07c4/awscli-1.15.77-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.1MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:21:49]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:21:55] 
[00:21:55] error: internal compiler error: unexpected panic
[00:21:55] 
[00:21:55] note: the compiler unexpectedly panicked. this is a bug.
[00:21:55] 
[00:21:55] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:55] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:21:55] 
[00:21:55] 
[00:21:55] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:55] 
[00:21:55] note: some of the compiler flags provided by cargo are hidden
[00:21:55] error: Could not compile `core`.
[00:21:55] 
[00:21:55] Caused by:
[00:21:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=aae624166adf9237 -C extra-filename=-aae624166adf9237 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=aae624166adf9237 -C extra-filename=-aae624166adf9237 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:55] warning: build failed, waiting for other jobs to finish...
[00:21:56] error: build failed
[00:21:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:56] expected success, got: exit code: 101
[00:21:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:21:56] travis_fold:end:stage1-std

[00:21:56] travis_time:end:stage1-std:start=1534213000098169954,finish=1534213007387858734,duration=7289688780


[00:21:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:56] Build completed unsuccessfully in 0:17:57
[00:21:56] Makefile:28: recipe for target 'all' failed
[00:21:56] make: *** [all] Error 1
el: [    0.585232] pinctrl core: initialized pinctrl subsystem
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.586343] RTC time:  1:53:40, date: 08/14/18
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.588087] NET: Registered protocol family 16
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.598088] cpuidle: using governor ladder
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.610073] cpuidle: using governor menu
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.610997] PCCT header not found.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.611893] ACPI: bus type PCI registered
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.612493] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.613822] PCI: Using configuration type 1 for base access
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.627140] ACPI: Added _OSI(Module Device)
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.627895] ACPI: Added _OSI(Processor Device)
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.628535] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.629307] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [    0.632681] ACPI: e-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for henrik.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account henrik.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 kernel: [   11.505167] random: nonblocking pool is initialized
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for emma.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account emma.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for igor.
Aug 14 01:53:50 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account igor.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account konstantinhaase.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for aj.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account aj.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new user account for solarce.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Created user account solarce.
Aug 14 01:53:51 travis-job-cda0f1b6-a2ee-4755-9d12-da51c5378266 google-accounts: INFO Creating a new u
