plain
Requirement already satisfied: PyYAML<=3.12,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.10.28 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2e/91/f0870d4de8eb78897ce781f3ff22fc492bbb9849b5c91f26db20b125ef36/botocore-1.10.28-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 39.5MB/s eta 0:00:01
    0% |▏                               | 20kB 41.4MB/s eta 0:00:01
    0% |▎                               | 30kB 36.1MB/s eta 0:00:01
    0% |▎                               | 40kB 17.5MB/s eta 0:00:01
---
[00:44:27] .........................................................i..........................................
[00:44:31] ..............................................................................ii....................
[00:44:37] ....................................................................................................
[00:44:43] .......................................................................................i............
[00:44:45] .....iiiiiiiii...................................................
[00:44:45] 
[00:44:45] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:45:32] .........................................................i..........................................
[00:45:36] .............................................................................ii.....................
[00:45:41] ....................................................................................................
[00:45:47] .......................................................................................i............
[00:45:49] .....iiiiiiiii...................................................
[00:45:49] 
[00:45:49]  finished in 63.515
[00:45:49] travis_fold:end:test_ui_nll

---
[01:11:52] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:52]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:11:58] error[E0599]: no method named `hash` found for type `i32` in the current scope
[01:11:58]      |
[01:11:58]      |
[01:11:58] 4266 |             1i32.hash(&mut h);
[01:11:58]      |
[01:11:58]      = help: items from traits can only be used if the trait is in scope
[01:11:58]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:58]              candidate #1: `use core::hash::Hash;`
[01:11:58]              candidate #1: `use core::hash::Hash;`
[01:11:58] 
[01:11:58] error[E0599]: no method named `finish` found for type `collections::hash::map::DefaultHasher` in the current scope
[01:11:58]      |
[01:11:58]      |
[01:11:58] 3194 | pub struct DefaultHasher(SipHasher13);
[01:11:58]      | -------------------------------------- method `finish` not found for this
[01:11:58] ...
[01:11:58] 4267 |             h.finish()
[01:11:58]      |
[01:11:58]      = help: items from traits can only be used if the trait is in scope
[01:11:58]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:58]              candidate #1: `use core::hash::Hasher;`
[01:11:58]              candidate #1: `use core::hash::Hasher;`
[01:11:58] 
[01:11:58] error[E0599]: no method named `hash` found for type `i32` in the current scope
[01:11:58]      |
[01:11:58]      |
[01:11:58] 4287 |             3i32.hash(&mut h);
[01:11:58]      |
[01:11:58]      = help: items from traits can only be used if the trait is in scope
[01:11:58]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:58]              candidate #1: `use core::hash::Hash;`
[01:11:58]              candidate #1: `use core::hash::Hash;`
[01:11:58] 
[01:11:58] error[E0599]: no method named `finish` found for type `collections::hash::map::DefaultHasher` in the current scope
[01:11:58]      |
[01:11:58]      |
[01:11:58] 3194 | pub struct DefaultHasher(SipHasher13);
[01:11:58]      | -------------------------------------- method `finish` not found for this
[01:11:58] ...
[01:11:58] 4288 |             h.finish()
[01:11:58]      |
[01:11:58]      = help: items from traits can only be used if the trait is in scope
[01:11:58]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:11:58]              candidate #1: `use core::hash::Hasher;`
[01:11:58]              candidate #1: `use core::hash::Hasher;`
[01:11:58] 
[01:11:58] error[E0599]: no method named `remove_kv` found for type `collections::hash::map::RawOccupiedEntry<'_, i32, i32>` in the current scope
[01:11:58]      |
[01:11:58]      |
[01:11:58] 1866 | pub struct RawOccupiedEntry<'a, K: 'a, V: 'a> {
[01:11:58]      | --------------------------------------------- method `remove_kv` not found for this
[01:11:58] ...
[01:11:58] 4292 |                 assert_eq!(view.remove_kv(), (3, 30));
[01:11:58]      |
[01:11:58]      |
[01:11:58]      = help: did you mean `remove`?
[01:12:06] error: aborting due to 5 previous errors
[01:12:06] 
[01:12:06] For more information about this error, try `rustc --explain E0599`.
[01:12:06] error: Could not compile `std`.
[01:12:06] error: Could not compile `std`.
[01:12:06] 
[01:12:06] To learn more, run the command again with --verbose.
[01:12:06] 
[01:12:06] 
[01:12:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:12:06] 
[01:12:06] 
[01:12:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:06] Build completed unsuccessfully in 0:29:50
[01:12:06] Build completed unsuccessfully in 0:29:50
[01:12:06] make: *** [check] Error 1
[01:12:06] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19abce3b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
