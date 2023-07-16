plain
travis_time:end:228cb124:start=1542152451577409021,finish=1542152453840676678,duration=2263267657
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
  Downloading https://files.pythonhosted.org/packages/ab/4e/ae2cf04ce20a86790605a67c0a5010df912625dc826a3b1c82ea71746668/awscli-1.16.54-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 4.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▊                               | 30kB 2.0MB/s eta 0:00:01
    2% |█                               | 40kB 1.8MB/s eta 0:00:01
---
[00:50:18] .................................................................................................... 100/5017
[00:50:21] .................................................................................................... 200/5017
[00:50:23] .............................ii............................................ii...................ii.. 300/5017
[00:50:26] ..............................................................................................iii... 400/5017
[00:50:29] .....iiiiiiii.iii............................iii...........................................i........ 500/5017
[00:50:36] .................................................................................................... 700/5017
[00:50:42] .................................................................................i...........i...... 800/5017
[00:50:46] .................................................................................................... 900/5017
[00:50:49] iiiii..................ii.iiii...................................................................... 1000/5017
---
[00:51:25] .................................................................................................... 2200/5017
[00:51:29] .................................................................................................... 2300/5017
[00:51:33] .................................................................................................... 2400/5017
[00:51:37] .................................................................................................... 2500/5017
[00:51:40] .................................................................................iiiiiiiii.......... 2600/5017
[00:51:47] ..............................................ii.................................................... 2800/5017
[00:51:50] .................................................................................................... 2900/5017
[00:51:54] .................................................................................................... 3000/5017
[00:51:57] ........................................i........................................................... 3100/5017
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:27] 
[01:05:27] running 116 tests
[01:05:30] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:05:31] i.i....iiii.....
[01:05:31] 
[01:05:31]  finished in 3.432
[01:05:31] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:45] 
[01:05:45] running 118 tests
[01:06:09] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:13] ......iii.i.....ii
[01:06:13] 
[01:06:13]  finished in 28.461
[01:06:13] travis_fold:end:test_debuginfo

