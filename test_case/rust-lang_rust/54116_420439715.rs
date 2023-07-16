plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e7/f2/d85748956835ae620f2a47451365bdcd115c8da3b78d32947e1dec64e8be/awscli-1.16.12-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 14.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.1MB/s eta 0:00:01
---
[01:20:22]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2663:29
[01:20:43]      |
[01:20:43] 2663 |                 mk_set(vec![String::from("b"), String::from("c")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:43] error[E0308]: mismatched types
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2667:29
[01:20:43]      |
[01:20:43] 2667 |                 mk_set(vec![String::from("e"), String::from("f")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:43] error[E0308]: mismatched types
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2674:29
[01:20:43]      |
[01:20:43] 2674 |                 mk_set(vec![String::from("e"), String::from("f")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:43] error[E0308]: mismatched types
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2678:29
[01:20:43]      |
[01:20:43] 2678 |                 mk_set(vec![String::from("b"), String::from("c")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:43] error[E0308]: mismatched types
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2685:29
[01:20:43]      |
[01:20:43] 2685 |                 mk_set(vec![String::from("b"), String::from("c")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:43] error[E0308]: mismatched types
[01:20:43] error[E0308]: mismatched types
[01:20:43]     --> librustc/session/config.rs:2689:29
[01:20:43]      |
[01:20:43] 2689 |                 mk_set(vec![String::from("f"), String::from("e")]),
[01:20:43]      |                             ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `std::string::String`
[01:20:43]      = note: expected type `std::option::Option<std::string::String>`
[01:20:43]                 found type `std::string::String`
[01:20:43] 
[01:20:52] error: aborting due to 6 previous errors
---
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5

