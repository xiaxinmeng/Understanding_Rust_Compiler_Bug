plain
travis_time:end:044c472c:start=1542422760185936867,finish=1542422818761646237,duration=58575709370
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a6/da/c99b10bfc509cbbea520886d2e8fe0e738e3ce22e2f528381f3bb2229433/awscli-1.16.57-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 16.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:49:44] .................................................................................................... 100/5019
[00:49:47] .................................................................................................... 200/5019
[00:49:50] .............................ii............................................ii...................ii.. 300/5019
[00:49:53] ..............................................................................................iii... 400/5019
[00:49:55] .....iiiiiiii.iii............................iii...........................................i........ 500/5019
[00:50:02] .................................................................................................... 700/5019
[00:50:07] .................................................................................i...........i...... 800/5019
[00:50:10] .................................................................................................... 900/5019
[00:50:13] iiiii..................ii.iiii...................................................................... 1000/5019
---
[00:50:48] .................................................................................................... 2200/5019
[00:50:52] .................................................................................................... 2300/5019
[00:50:56] .................................................................................................... 2400/5019
[00:51:00] .................................................................................................... 2500/5019
[00:51:03] ..............................................................................iiiiiiiii............. 2600/5019
[00:51:10] ............................................ii...................................................... 2800/5019
[00:51:13] .................................................................................................... 2900/5019
[00:51:17] .................................................................................................... 3000/5019
[00:51:20] .......................................i............................................................ 3100/5019
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:54] 
[01:04:54] running 116 tests
[01:04:57] i..ii....iii.iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:04:57] i.i....iiii.....
[01:04:57] 
[01:04:57]  finished in 3.457
[01:04:57] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:11] 
[01:05:11] running 118 tests
[01:05:36] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:40] ......iii.i.....ii
[01:05:40] 
[01:05:40]  finished in 28.568
[01:05:40] travis_fold:end:test_debuginfo

