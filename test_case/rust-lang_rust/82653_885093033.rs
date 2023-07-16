
running: /data/semarie/build-rust/build_dir/build/bootstrap/debug/bootstrap dist --jobs=4
warning: x.py has made several changes recently you may want to look at
help: consider looking at the changes in `src/bootstrap/CHANGELOG.md`
note: to silence this warning, add `changelog-seen = 2` at the top of `config.toml`
finding compilers
CC_x86_64-unknown-openbsd = "cc"
CFLAGS_x86_64-unknown-openbsd = ["-O2", "-ffunction-sections", "-fdata-sections", "-fPIC", "-m64", "-O2", "-pipe"]
CXX_x86_64-unknown-openbsd = "c++"
CXXFLAGS_x86_64-unknown-openbsd = ["-O2", "-ffunction-sections", "-fdata-sections", "-fPIC", "-m64", "-O2", "-pipe"]
AR_x86_64-unknown-openbsd = "ar"
running sanity check
learning about cargo
thread 'main' panicked at 'command did not execute successfully: "git" "config" "--file" "/data/semarie/build-rust/build_dir/rustc-nightly-src/.gitmodules" "--get-regexp" "path"
expected success, got: exit status: 1', src/bootstrap/lib.rs:568:22
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Traceback (most recent call last):
  File "/data/semarie/build-rust/build_dir/rustc-nightly-src/x.py", line 27, in <module>
    bootstrap.main()
  File "/data/semarie/build-rust/build_dir/rustc-nightly-src/src/bootstrap/bootstrap.py", line 1209, in main
    bootstrap(help_triggered)
  File "/data/semarie/build-rust/build_dir/rustc-nightly-src/src/bootstrap/bootstrap.py", line 1195, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/data/semarie/build-rust/build_dir/rustc-nightly-src/src/bootstrap/bootstrap.py", line 153, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /data/semarie/build-rust/build_dir/build/bootstrap/debug/bootstrap dist --jobs=4
Thu Jul 22 18:07:30 CEST 2021: task not finished: see build.log for detail
