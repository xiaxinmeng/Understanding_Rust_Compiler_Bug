plain
travis_time:end:0d7e75ea:start=1543549068915637338,finish=1543549071107749173,duration=2192111835
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
  Downloading https://files.pythonhosted.org/packages/aa/e5/ebd5896ad5ae353d23bea05ebb8edd3d49f1471784f6afa12a9cf11710de/awscli-1.16.67-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 19.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:47:52] .................................................................................................... 1100/5066
[00:47:54] .................................................................................................... 1200/5066
[00:47:56] .................................................................................................... 1300/5066
[00:47:59] .................................................................................................... 1400/5066
[00:48:01] ..............................................................................F..................... 1500/5066
[00:48:08] .................................................................................................... 1700/5066
[00:48:11] .................................................................................................... 1800/5066
[00:48:15] .................................................................................................... 1900/5066
[00:48:18] ...........................i........................................................................ 2000/5066
---
[00:48:37] .................................................................................................... 2500/5066
[00:48:41] .................................................................................................... 2600/5066
[00:48:45] .................................................................................................... 2700/5066
[00:48:48] .................................................................................................... 2800/5066
[00:48:51] ......................................................F............................................. 2900/5066
[00:48:59] ......................................................................i............................. 3100/5066
[00:49:02] .................................................................................................... 3200/5066
[00:49:05] .................................ii..i..ii.......................................................... 3300/5066
[00:49:09] .................................................................................................... 3400/5066
---
[00:49:23] .................................................................................................... 4000/5066
[00:49:26] .................................................................................................... 4100/5066
[00:49:29] .................................................................................................... 4200/5066
[00:49:33] .................................................i.................................................. 4300/5066
[00:49:39] ...................................................................................F................ 4400/5066
[00:49:45] .................................................................................................... 4600/5066
[00:49:49] ..............................i..................................................................... 4700/5066
[00:49:53] .................................................................................................... 4800/5066
[00:49:56] .................................................................................................... 4900/5066
[00:49:56] .................................................................................................... 4900/5066
:00] -    |
[00:50:00] -    = help: add #![feature(self_struct_ctor)] to the crate attributes to enable
[00:50:00] + error: aborting due to 4 previous errors
[00:50:00] 32 
[00:50:00] - error[E0658]: `Self` struct constructors are unstable (see issue #51994)
[00:50:00] -    |
[00:50:00] -    |
[00:50:00] - LL |         Self => (),
[00:50:00] -    |
[00:50:00] -    |
[00:50:00] -    = help: add #![feature(self_struct_ctor)] to the crate attributes to enable
[00:50:00] - 
[00:50:00] - error[E0658]: `Self` struct constructors are unstable (see issue #51994)
[00:50:00] -    |
[00:50:00] -    |
[00:50:00] - LL |         Foo { x: Self } => (),
[00:50:00] -    |
[00:50:00] -    |
[00:50:00] -    = help: add #![feature(self_struct_ctor)] to the crate attributes to enable
[00:50:00] - error: aborting due to 7 previous errors
[00:50:00] - 
[00:50:00] - Some errors occurred: E0432, E0531, E0658.
[00:50:00] + Some errors occurred: E0432, E0531.
[00:50:00] + Some errors occurred: E0432, E0531.
[00:50:00] 52 For more information about an error, try `rustc --explain E0432`.
[00:50:00] 53 
[00:50:00] 
[00:50:00] 
[00:50:00] The actual stderr differed from the expected stderr.
[00:50:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/self_type_keyword-2.stderr
[00:50:00] To update references, rerun the tests and pass the `--bless` flag
[00:50:00] To only update this specific test, also pass `--test-args self/self_type_keyword-2.rs`
[00:50:00] 
[00:50:00] error: 1 errors occurred comparing ode":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":745,"byte_end":749,"line_start":19,"line_end":19,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        Self => (),","highlight_start":9,"highlight_end":13}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:19:9\n   |\nLL |         Self => (),\n   |         ^^^^ not found in this scope\n\n"}
[00:50:00] {"message":"cannot find unit struct/variant or constant `Self` in this scope","code":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":937,"byte_end":941,"line_start":22,"line_end":22,"column_start":18,"column_end":22,"is_primary":true,"text":[{"text":"        Foo { x: Self } => (),","highlight_start":18,"highlight_end":22}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:22:18\n   |\nLL |         Foo { x: Self } => (),\n   |                  ^^^^ not found in this scope\n\n"}
[00:50:00] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous
