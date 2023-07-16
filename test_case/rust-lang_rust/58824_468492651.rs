plain
travis_time:end:02e239a8:start=1551393402098862844,finish=1551393403024057781,duration=925194937
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/aa/ea/cb62728e9b38f9d8c620d60815f8dd54ca015f6b9af8f5a3d03d9b2e3c64/awscli-1.16.115-py2.py3-none-any.whl (1.4MB)
    1% |▌                               | 20kB 2.2MB/s eta 0:00:01
    2% |▊                               | 30kB 3.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.1MB/s eta 0:00:01
    3% |█▏                              | 51kB 2.6MB/s eta 0:00:01
---
    99% |████████████████████████████████| 542kB 90.1MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 27.2MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.105 (from awscli)
  Downloading https://files.pythonhosted.org/packages/cf/ce/acc9013dee20fc94c9b9ae121f5b7b342a206f0d577be1e5c6129811194a/botocore-1.12.105-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 29.9MB/s eta 0:00:01
    0% |▏                               | 30kB 36.0MB/s eta 0:00:01
    0% |▎                               | 40kB 41.5MB/s eta 0:00:01
    0% |▎                               | 51kB 45.7MB/s eta 0:00:01
---
[00:06:51]    --> src/librustdoc/passes/collect_intra_doc_links.rs:563:21
[00:06:51]     |
[00:06:51] 561 |                     Def::Method(..) | Def::Fn(..) => {
[00:06:51]     |                                                      - close delimiter possibly meant for this
[00:06:51] 562 |                         ("add parentheses", format!("{}()", path_str)
[00:06:51]     |                         - un-closed delimiter
[00:06:51]     |                     ^ incorrect close delimiter
[00:06:51] 
[00:06:55] error: aborting due to previous error
[00:06:55] 
---
travis_time:end:01170a56:start=1551393830096704322,finish=1551393830101886727,duration=5182405
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:332b24d7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15227283
travis_time:start:15227283
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09d8a1e0
$ dmesg | grep -i kill
