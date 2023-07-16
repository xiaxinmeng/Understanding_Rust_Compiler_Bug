plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/12/92bd3655f436aa009688e7ccb8b7581554fb64278d111f5845e79da8e618/awscli-1.16.14-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:47:42] 
[00:47:42] warning: `[]` cannot be resolved, ignoring it...
[00:47:42]     --> libstd/collections/hash/map.rs:1847:40
[00:47:42]      |
[00:47:42] 1847 | /// See the [`HashMap::raw_entry_mut`][] docs for usage examples.
[00:47:42]      |
[00:47:42]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:42] 
[00:47:42] warning: `[]` cannot be resolved, ignoring it...
[00:47:42] warning: `[]` cannot be resolved, ignoring it...
[00:47:42]     --> libstd/collections/hash/map.rs:1891:36
[00:47:42]      |
[00:47:42] 1891 | /// See the [`HashMap::raw_entry`][] docs for usage examples.
[00:47:42]      |
[00:47:42]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:47:42] 
[00:47:42] warning: `[Seek::seek_relative]` cannot be resolved, ignoring it...
---
[01:27:34] travis_fold:end:stage0-linkchecker

[01:27:34] travis_time:end:stage0-linkchecker:start=1536875406008140072,finish=1536875408560221231,duration=2552081159

[01:29:29] std/collections/hash_map/enum.RawEntryMut.html:5: broken link - std/collections/hash_map/struct.Entry.html
[01:30:32] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:30:32] 
[01:30:32] 
[01:30:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:30:32] expected success, got: exit code: 101
[01:30:32] expected success, got: exit code: 101
[01:30:32] 
[01:30:32] 
[01:30:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:32] Build completed unsuccessfully in 0:42:04
[01:30:32] make: *** [check] Error 1
[01:30:32] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0acc4040
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
