plain
[01:11:28] [TIMING] Analysis { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 2.988
[01:11:28] Dist src
[01:11:40] [TIMING] Src -- 12.744
[01:11:40] Create plain source tarball
[01:13:37] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:15:29] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:17:22] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:17:22] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[01:17:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:17:22] Build completed unsuccessfully in 1:11:19
travis_time:end:04bfa000:start=1529561613057339809,finish=1529566255330042980,duration=4642272703171

---
travis_time:end:210079ce:start=1529566256794353871,finish=1529566256801874250,duration=7520379
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b4e0685
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
travis_time:start:247b1c44
$ dmesg | grep -i kill
