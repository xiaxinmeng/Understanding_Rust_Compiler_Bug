plain
travis_time:end:007f65ac:start=1542773667222586693,finish=1542773723368948278,duration=56146361585
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
    0% |▎                               | 10kB 18.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:40:22]    Compiling parking_lot_core v0.3.0
[00:40:22]    Compiling tempfile v3.0.3
[00:40:24]    Compiling parking_lot v0.6.4
[00:40:25]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:40:25] error: expected one of `.`, `;`, `?`, or an operator, found `for`
[00:40:25]     --> librustdoc/clean/mod.rs:3987:13
[00:40:25]      |
[00:40:25] 3985 |             let segment = path_it.next()?
[00:40:25]      |                                          - expected one of `.`, `;`, `?`, or an operator here
[00:40:25] 3986 | 
[00:40:25] 3987 |             for item in mem::replace(&mut items, Lrc::new(vec![])).iter() {
[00:40:25]      |             ^^^ unexpected token
travis_time:end:057e54b4:start=1542773731993765104,finish=1542776163432227864,duration=2431438462760
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11b4f58a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0788cc40:start=1542776164242627075,finish=1542776164248065482,duration=5438407
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0805d226
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:af
