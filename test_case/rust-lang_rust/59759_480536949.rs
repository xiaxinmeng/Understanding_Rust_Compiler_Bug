plain
travis_time:end:0abad691:start=1554583822586491811,finish=1554583901692540650,duration=79106048839
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:12]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:13] error[E0433]: failed to resolve: use of undeclared type or module `std`
[00:04:13]    --> src/libstd/io/buffered.rs:592:21
[00:04:13]     |
[00:04:13] 592 |                 use std::mem::{replace, uninitialized, forget};
[00:04:13] 
[00:04:14] error[E0425]: cannot find function `replace` in this scope
[00:04:14]    --> src/libstd/io/buffered.rs:593:29
[00:04:14]     |
[00:04:14]     |
[00:04:14] 593 |                 let inner = replace(&mut self.inner, unsafe { uninitialized() });
[00:04:14] help: possible candidates are found in other modules, you can import them into scope
[00:04:14]     |
[00:04:14] 3   | use core::mem::replace;
[00:04:14]     |
---
[00:04:14] 
[00:04:14] error[E0425]: cannot find function `uninitialized` in this scope
[00:04:14]    --> src/libstd/io/buffered.rs:593:63
[00:04:14]     |
[00:04:14] 593 |                 let inner = replace(&mut self.inner, unsafe { uninitialized() });
[00:04:14] help: a tuple struct with a similar name exists
[00:04:14]     |
[00:04:14]     |
[00:04:14] 593 |                 let inner = replace(&mut self.inner, unsafe { Initializer() });
[00:04:14] help: possible candidates are found in other modules, you can import them into scope
[00:04:14]     |
[00:04:14] 3   | use core::mem::uninitialized;
[00:04:14]     |
[00:04:14]     |
[00:04:14] 3   | use core::mem::uninitialized;
[00:04:14]     |
[00:04:14] 
[00:04:14] error[E0425]: cannot find function `replace` in this scope
[00:04:14]    --> src/libstd/io/buffered.rs:594:17
[00:04:14]     |
[00:04:14] 594 |                 replace(&mut self.buf, unsafe { uninitialized() });
[00:04:14] help: possible candidates are found in other modules, you can import them into scope
[00:04:14]     |
[00:04:14] 3   | use core::mem::replace;
[00:04:14]     |
---
[00:04:14] 
[00:04:14] error[E0425]: cannot find function `uninitialized` in this scope
[00:04:14]    --> src/libstd/io/buffered.rs:594:49
[00:04:14]     |
[00:04:14] 594 |                 replace(&mut self.buf, unsafe { uninitialized() });
[00:04:14] help: a tuple struct with a similar name exists
[00:04:14]     |
[00:04:14]     |
[00:04:14] 594 |                 replace(&mut self.buf, unsafe { Initializer() });
[00:04:14] help: possible candidates are found in other modules, you can import them into scope
[00:04:14]     |
[00:04:14] 3   | use core::mem::uninitialized;
[00:04:14]     |
---
[00:04:14]     |
[00:04:14] 3   | use core::mem::forget;
[00:04:14]     |
[00:04:14] 
[00:04:14] error: unused imports: `forget`, `replace`, `uninitialized`
[00:04:14]     |
[00:04:14]     |
[00:04:14] 592 |                 use std::mem::{replace, uninitialized, forget};
[00:04:14]     |
[00:04:14]     = note: `-D unused-imports` implied by `-D warnings`
[00:04:14] 
[00:04:15] error[E0308]: mismatched types
[00:04:15] error[E0308]: mismatched types
[00:04:15]    --> src/libstd/io/buffered.rs:529:35
[00:04:15]     |
[00:04:15] 529 |     pub fn get_ref(&self) -> &W { self.inner }
[00:04:15]     |                                   |
[00:04:15]     |                                   |
[00:04:15]     |                                   expected &W, found type parameter
[00:04:15]     |                                   help: consider borrowing here: `&self.inner`
[00:04:15]     = note: expected type `&W`
[00:04:15]                found type `W`
[00:04:15] 
[00:04:15] error[E0308]: mismatched types
[00:04:15] error[E0308]: mismatched types
[00:04:15]    --> src/libstd/io/buffered.rs:547:43
[00:04:15]     |
[00:04:15] 547 |     pub fn get_mut(&mut self) -> &mut W { self.inner }
[00:04:15]     |                                           |
[00:04:15]     |                                           |
[00:04:15]     |                                           expected &mut W, found type parameter
[00:04:15]     |                                           help: consider mutably borrowing here: `&mut self.inner`
[00:04:15]     = note: expected type `&mut W`
[00:04:15]                found type `W`
[00:04:15] 
[00:04:16] error: aborting due to 9 previous errors
---
travis_time:end:27f6e2cc:start=1554584169550828654,finish=1554584169556561933,duration=5733279
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:215a192c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ec41fa
travis_time:start:04ec41fa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:034a844
