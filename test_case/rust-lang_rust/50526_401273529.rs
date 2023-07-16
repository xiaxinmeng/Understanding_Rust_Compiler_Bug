plain
[01:10:30] [TIMING] Analysis { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 3.058
[01:10:30] Dist src
[01:10:43] [TIMING] Src -- 12.640
[01:10:43] Create plain source tarball
[01:12:52] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:14:44] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:16:37] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:16:37] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:16:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:16:37] Build completed unsuccessfully in 1:11:00
travis_time:end:0de3483f:start=1530253039615388785,finish=1530257636929458900,duration=4597314070115

---
travis_time:end:08d5687b:start=1530257638293772449,finish=1530257638303225967,duration=9453518
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1dc8c752
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
global:
global:
  _ZdaPv;
  _ZdaPvRKSt9nothrow_t;
  _ZdaPvSt11align_val_t;
  _ZdaPvSt11align_val_tRKSt9nothrow_t;
  _ZdaPvj;
  _ZdaPvjSt11align_val_t;
  _ZdlPv;
  _ZdlPvRKSt9nothrow_t;
  _ZdlPvSt11align_val_t;
  _ZdlPvSt11align_val_tRKSt9nothrow_t;
  _ZdlPvj;
  _ZdlPvjSt11align_val_t;
  _Znaj;
  _ZnajRKSt9nothrow_t;
  _ZnajSt11align_val_t;
  _ZnajSt11align_val_tRKSt9nothrow_t;
  _Znwj;
  _ZnwjRKSt9nothrow_t;
  _ZnwjSt11align_val_t;
  _ZnwjSt11align_val_tRKSt9nothrow_t;
  __asan_*;
  __cxa_atexit;
  __cxa_throw;
  __fprintf_chk;
  __getdelim;
  __interceptor___cxa_atexit;
  __interceptor___cxa_throw;
  __interceptor___fprintf_chk;
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0060de60
$ dmesg | grep -i kill
