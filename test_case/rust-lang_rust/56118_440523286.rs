plain
travis_time:end:01857879:start=1542767386853813576,finish=1542767446914347746,duration=60060534170
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
  Downloading https://files.pythonhosted.org/packages/ef/cc/d8f288b5b450e59b92c460727012ee002925324bc655255a9945a45ab9ad/awscli-1.16.59-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 10.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:51:06] .................................................................................................... 100/5042
[00:51:09] .................................................................................................... 200/5042
[00:51:12] .............................ii............................................ii...................ii.. 300/5042
[00:51:15] ..............................................................................................iii... 400/5042
[00:51:18] .....iiiiiiii.iii............................iii...........................................i........ 500/5042
[00:51:25] .................................................................................................... 700/5042
[00:51:31] ..................................................................................i...........i..... 800/5042
[00:51:34] .................................................................................................... 900/5042
[00:51:34] .................................................................................................... 900/5042
[00:51:37] .iiiii...................iiiiii..................................................................... 1000/5042
[00:51:39] ............................................................................iiiiiiii................ 1100/5042
[00:51:44] .................................................................................................... 1300/5042
[00:51:46] .................................................................................................... 1400/5042
[00:51:49] .................................i.................................................................. 1500/5042
[00:51:51] ..i.........ii.........................................................i............................ 1600/5042
---
[00:52:12] .................................................................................................... 2200/5042
[00:52:16] .................................................................................................... 2300/5042
[00:52:20] .................................................................................................... 2400/5042
[00:52:24] .................................................................................................... 2500/5042
[00:52:27] .....................................................................................iiiiiiiii...... 2600/5042
[00:52:34] ...................................................ii............................................... 2800/5042
[00:52:37] .................................................................................................... 2900/5042
[00:52:41] .................................................................................................... 3000/5042
[00:52:44] ...............................................i.................................................... 3100/5042
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:47] 
[01:06:47] running 117 tests
[01:06:50] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:06:50] i.i.....iiii.....
[01:06:50] 
[01:06:50]  finished in 3.526
[01:06:50] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:05] 
[01:07:05] running 118 tests
[01:07:29] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:07:33] ......iii.i.....ii
[01:07:33] 
[01:07:33]  finished in 28.915
[01:07:33] travis_fold:end:test_debuginfo

---
[01:36:29] travis_fold:end:stage0-linkchecker

[01:36:29] travis_time:end:stage0-linkchecker:start=1542773243168977343,finish=1542773245517772138,duration=2348794795

[01:39:12] std/index.html:10: broken link fragment `#use-to-bring-paths-into-scope` pointing to `book/ch07-02-modules-and-use-to-control-scope-and-privacy.html`
