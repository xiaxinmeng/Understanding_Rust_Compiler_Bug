plain
travis_time:end:076ceb0a:start=1559079351708603131,finish=1559079481639803353,duration=129931200222
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
DEPRECATION: Python 2.7 will reach the end of its life on January 1st, 2020. Please upgrade your Python as Python 2.7 won't be maintained after that date. A future version of pip will drop support for Python 2.7.
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/1b/ea/62e3e65da5016f9641b9edee0f459e87eb813ed276f996aceb76f3ddd140/awscli-1.16.167-py2.py3-none-any.whl (1.6MB)
    1% |▍                               | 20kB 2.0MB/s eta 0:00:01
    1% |▋                               | 30kB 2.8MB/s eta 0:00:01
    2% |▉                               | 40kB 1.9MB/s eta 0:00:01
    3% |█                               | 51kB 2.3MB/s eta 0:00:01
---
[00:24:46]    Compiling rustc-demangle v0.1.10
[00:24:49]    Compiling num_cpus v1.8.0
[00:24:50]    Compiling memmap v0.6.2
[00:24:53]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:24:57] error[E0407]: method `unchecked_uadd` is not a member of trait `BuilderMethods`
[00:24:57]     |
[00:24:57]     |
[00:24:57] 409 | /     fn unchecked_uadd(&mut self, lhs: &'ll Value, rhs: &'ll Value) -> &'ll Value {
[00:24:57] 410 | |         self.count_insn("unchecked_uadd");
[00:24:57] 411 | |         unsafe {
[00:24:57] 412 | |             llvm::LLVMBuildNUWAdd(self.llbuilder, lhs, rhs, noname())
[00:24:57] 414 | |     }
[00:24:57] 414 | |     }
[00:24:57]     | |_____^ not a member of trait `BuilderMethods`
[00:24:57] 
[00:24:58] error[E0046]: not all trait items implemented, missing: `unchecked_uwadd`
[00:24:58]     |
[00:24:58]     |
[00:24:58] 103 | impl BuilderMethods<'a, 'tcx> for Builder<'a, 'll, 'tcx> {
[00:24:58]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `unchecked_uwadd` in implementation
[00:24:58]     |
[00:24:58]     = note: `unchecked_uwadd` from trait: `fn(&mut Self, <Self as rustc_codegen_ssa::traits::BackendTypes>::Value, <Self as rustc_codegen_ssa::traits::BackendTypes>::Value) -> <Self as rustc_codegen_ssa::traits::BackendTypes>::Value`
[00:24:58] 
[00:24:59] error[E0599]: no method named `unchecked_uadd` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:24:59]     |
[00:24:59]     |
[00:24:59] 439 |                                     self.unchecked_uadd(args[0].immediate(), args[1].immediate())
[00:24:59] 
[00:25:00] error: aborting due to 3 previous errors
[00:25:00] 
[00:25:00] Some errors have detailed explanations: E0046, E0407, E0599.
---
travis_time:end:02a65658:start=1559080992209544024,finish=1559080992214521311,duration=4977287
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2507cee0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:117a5360
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib
