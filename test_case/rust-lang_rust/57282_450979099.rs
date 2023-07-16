plain
travis_time:end:20970b74:start=1546459517418038422,finish=1546459518410024818,duration=991986396
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:25:02]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:26:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:33:38]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:33:38]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:34:11] error: `<M as interpret::machine::Machine<'_, '_, '_>>::AllocExtra` does not live long enough
[00:34:11]    --> src/librustc_mir/interpret/memory.rs:364:25
[00:34:11]     |
[00:34:11] 364 |             let alloc = Self::get_static_alloc(id, self.tcx, &self.extra).map_err(Err)?;
[00:34:11] 
[00:34:11] 
[00:34:11] error[E0309]: the parameter type `M` may not live long enough
[00:34:11]    --> src/librustc_mir/interpret/memory.rs:394:47
[00:34:11]     |
[00:34:11] 394 |           let a = self.alloc_map.get_mut_or(id, || {
[00:34:11]     |  _______________________________________________^
[00:34:11] 395 | |             // Need to make a copy, even if `get_static_alloc` is able
[00:34:11] 396 | |             // to give us a cheap reference.
[00:34:11] 397 | |             let alloc = Self::get_static_alloc(id, tcx, memory_extra)?;
[00:34:11] 404 | |             }
[00:34:11] 405 | |         });
[00:34:11]     | |_________^
[00:34:11]     |
[00:34:11]     |
[00:34:11]     = help: consider adding an explicit lifetime bound `M: 'tcx`...
[00:34:11] error: unsatisfied lifetime constraints
[00:34:11]    --> src/librustc_mir/interpret/memory.rs:394:47
[00:34:11]     |
[00:34:11]     |
[00:34:11] 304 |   impl<'a, 'mir, 'tcx, M: Machine<'a, 'mir, 'tcx>> Memory<'a, 'mir, 'tcx, M> {
[00:34:11]     |        --  ---- lifetime `'mir` defined here
[00:34:11]     |        lifetime `'a` defined here
[00:34:11] ...
[00:34:11] ...
[00:34:11] 394 |           let a = self.alloc_map.get_mut_or(id, || {
[00:34:11]     |  _______________________________________________^
[00:34:11] 395 | |             // Need to make a copy, even if `get_static_alloc` is able
[00:34:11] 396 | |             // to give us a cheap reference.
[00:34:11] 397 | |             let alloc = Self::get_static_alloc(id, tcx, memory_extra)?;
[00:34:11] 404 | |             }
[00:34:11] 405 | |         });
[00:34:11] 405 | |         });
[00:34:11]     | |_________^ requires that `'a` must outlive `'mir`
[00:34:11] error: unsatisfied lifetime constraints
[00:34:11]    --> src/librustc_mir/interpret/memory.rs:394:47
[00:34:11]     |
[00:34:11]     |
[00:34:11] 304 |   impl<'a, 'mir, 'tcx, M: Machine<'a, 'mir, 'tcx>> Memory<'a, 'mir, 'tcx, M> {
[00:34:11]     |        --  ---- lifetime `'mir` defined here
[00:34:11]     |        lifetime `'a` defined here
[00:34:11] ...
[00:34:11] ...
[00:34:11] 394 |           let a = self.alloc_map.get_mut_or(id, || {
[00:34:11]     |  _______________________________________________^
[00:34:11] 395 | |             // Need to make a copy, even if `get_static_alloc` is able
[00:34:11] 396 | |             // to give us a cheap reference.
[00:34:11] 397 | |             let alloc = Self::get_static_alloc(id, tcx, memory_extra)?;
[00:34:11] 404 | |             }
[00:34:11] 405 | |         });
[00:34:11] 405 | |         });
[00:34:11]     | |_________^ requires that `'mir` must outlive `'a`
[00:34:14] error: aborting due to 4 previous errors
[00:34:14] 
[00:34:14] For more information about this error, try `rustc --explain E0309`.
[00:34:14] error: Could not compile `rustc_mir`.
