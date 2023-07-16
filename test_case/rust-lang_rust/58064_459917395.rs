plain
travis_time:end:041ab0fa:start=1549068308862332203,finish=1549068430091833741,duration=121229501538
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:33]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:06:33] error[E0407]: method `try_rfold` is not a member of trait `Iterator`
[00:06:33]     --> src/liballoc/collections/vec_deque.rs:2190:5
[00:06:33]      |
[00:06:33] 2190 | /     fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R where
[00:06:33] 2191 | |         Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
[00:06:33] 2192 | |     {
[00:06:33] 2193 | |         let (front, back) = RingSlices::ring_slices(self.ring, self.head, self.tail);
[00:06:33] 2201 | |         accum
[00:06:33] 2202 | |     }
[00:06:33]      | |_____^ not a member of trait `Iterator`
[00:06:33] 
[00:06:33] 
[00:06:34] error[E0599]: no method named `offset` found for type `&'a [T]` in the current scope
[00:06:34]      |
[00:06:34]      |
[00:06:34] 2184 |             self.tail = unsafe { self.ring.offset(iter.as_slice().as_ptr()) };
[00:06:34] 
[00:06:34] error[E0308]: mismatched types
[00:06:34]     --> src/liballoc/collections/vec_deque.rs:2187:9
[00:06:34]      |
[00:06:34]      |
[00:06:34] 2176 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
[00:06:34]      |                                                           - expected `R` because of return type
[00:06:34] ...
[00:06:34] 2185 |             accum = res?;
[00:06:34] 2186 |         }
[00:06:34] 2187 |         accum
[00:06:34]      |         ^^^^^ expected type parameter, found a different type parameter
[00:06:34]      |
[00:06:34]      |
[00:06:34]      = note: expected type `R`
[00:06:34]                 found type `B`
[00:06:34] 
[00:06:34] error[E0599]: no method named `offset` found for type `&'a [T]` in the current scope
[00:06:34]      |
[00:06:34]      |
[00:06:34] 2198 |             self.tail = unsafe { self.ring.offset(iter.as_slice().as_ptr()) };
[00:06:34] 
[00:06:34] error[E0308]: mismatched types
[00:06:34]     --> src/liballoc/collections/vec_deque.rs:2201:9
[00:06:34]      |
[00:06:34]      |
[00:06:34] 2190 |     fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R where
[00:06:34]      |                                                            - expected `R` because of return type
[00:06:34] ...
[00:06:34] 2199 |             accum = res?;
[00:06:34] 2200 |         }
[00:06:34] 2201 |         accum
[00:06:34]      |         ^^^^^ expected type parameter, found a different type parameter
[00:06:34]      |
---
[00:06:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:36] expected success, got: exit code: 101
[00:06:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:36] Build completed unsuccessfully in 0:00:38
[00:06:36] make: *** [all] Error 1
[00:06:36] Makefile:18: recipe for target 'all' failed
Sat, 02 Feb 2019 00:53:56 GMT
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:085f7fb0
---
travis_time:end:06364963:start=1549068837530177196,finish=1549068837535200287,duration=5023091
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11da2258
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004cd3f2
travis_time:start:004cd3f2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07abceab
$ dmesg | grep -i kill
