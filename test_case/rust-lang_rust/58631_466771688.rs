plain
[01:19:36]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:19:39] error[E0531]: cannot find tuple struct/variant `Local` in this scope
[01:19:39]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:298:13
[01:19:39]     |
[01:19:39] 298 |             Local(local) => return Some((*local, deref || field)),
[01:19:39] help: possible candidates are found in other modules, you can import them into scope
[01:19:39]     |
[01:19:39] 1   | use rustc::hir::Node::Local;
[01:19:39]     |
---
[01:19:39] 
[01:19:40] error[E0531]: cannot find tuple struct/variant `Local` in this scope
[01:19:40]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:298:13
[01:19:40]     |
[01:19:40] 298 |             Local(local) => return Some((*local, deref || field)),
[01:19:40] help: possible candidates are found in other modules, you can import them into scope
[01:19:40]     |
[01:19:40] 1   | use rustc::hir::Node::Local;
[01:19:40]     |
---
[01:19:40] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:148:44
[01:19:43]     |
[01:19:43] 148 |                     if *res == mir::Place::Local(cloned);
[01:19:43]     |                                |
[01:19:43]     |                                variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                help: did you mean: `local`
[01:19:43] 
[01:19:43] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:234:47
[01:19:43]     |
[01:19:43] 234 |         if let mir::Operand::Move(mir::Place::Local(local)) = &args[0];
[01:19:43]     |                                   |
[01:19:43]     |                                   variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                   help: did you mean: `local`
[01:19:43] 
[01:19:43] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:260:59
[01:19:43]     |
[01:19:43] 260 |             if let mir::StatementKind::Assign(mir::Place::Local(local), v) = &stmt.kind {
[01:19:43]     |                                               |
[01:19:43]     |                                               variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                               help: did you mean: `local`
[01:19:43] 
[01:19:43] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:148:44
[01:19:43]     |
[01:19:43] 148 |                     if *res == mir::Place::Local(cloned);
[01:19:43]     |                                |
[01:19:43]     |                                variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                help: did you mean: `local`
[01:19:43] 
[01:19:43] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:234:47
[01:19:43]     |
[01:19:43] 234 |         if let mir::Operand::Move(mir::Place::Local(local)) = &args[0];
[01:19:43]     |                                   |
[01:19:43]     |                                   variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                   help: did you mean: `local`
[01:19:43] 
[01:19:43] 
[01:19:43] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:19:43]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:260:59
[01:19:43]     |
[01:19:43] 260 |             if let mir::StatementKind::Assign(mir::Place::Local(local), v) = &stmt.kind {
[01:19:43]     |                                               |
[01:19:43]     |                                               variant not found in `rustc::mir::Place<'_>`
[01:19:43]     |                                               help: did you mean: `local`
[01:19:43] 
---
[01:23:58]    Compiling rls-analysis v0.16.12
[01:24:01] error[E0531]: cannot find tuple struct/variant `Local` in this scope
[01:24:01]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:298:13
[01:24:01]     |
[01:24:01] 298 |             Local(local) => return Some((*local, deref || field)),
[01:24:01] help: possible candidates are found in other modules, you can import them into scope
[01:24:01]     |
[01:24:01] 1   | use rustc::hir::Node::Local;
[01:24:01]     |
---
[01:24:01] 
[01:24:06] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:24:06]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:148:44
[01:24:06]     |
[01:24:06] 148 |                     if *res == mir::Place::Local(cloned);
[01:24:06]     |                                |
[01:24:06]     |                                variant not found in `rustc::mir::Place<'_>`
[01:24:06]     |                                help: did you mean: `local`
[01:24:06] 
[01:24:06] 
[01:24:06] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:24:06]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:234:47
[01:24:06]     |
[01:24:06] 234 |         if let mir::Operand::Move(mir::Place::Local(local)) = &args[0];
[01:24:06]     |                                   |
[01:24:06]     |                                   variant not found in `rustc::mir::Place<'_>`
[01:24:06]     |                                   help: did you mean: `local`
[01:24:06] 
[01:24:06] 
[01:24:06] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:24:06]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:260:59
[01:24:06]     |
[01:24:06] 260 |             if let mir::StatementKind::Assign(mir::Place::Local(local), v) = &stmt.kind {
[01:24:06]     |                                               |
[01:24:06]     |                                               variant not found in `rustc::mir::Place<'_>`
[01:24:06]     |                                               help: did you mean: `local`
[01:24:06] 
---
[01:30:05]    Compiling miri v0.1.0 (/checkout/src/tools/miri)
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/lib.rs:125:44
[01:30:07]     |
[01:30:07] 125 |     let dest = ecx.eval_place(&mir::Place::Local(args.next().unwrap()))?;
[01:30:07]     |                                |
[01:30:07]     |                                variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/lib.rs:129:44
[01:30:07]     |
[01:30:07] 129 |     let dest = ecx.eval_place(&mir::Place::Local(args.next().unwrap()))?;
[01:30:07]     |                                |
[01:30:07]     |                                variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/lib.rs:141:44
[01:30:07]     |
[01:30:07] 141 |     let dest = ecx.eval_place(&mir::Place::Local(args.next().unwrap()))?;
[01:30:07]     |                                |
[01:30:07]     |                                variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/lib.rs:440:47
[01:30:07]     |
[01:30:07] 440 |         let arg = ecx.eval_place(&mir::Place::Local(args.next().unwrap()))?;
[01:30:07]     |                                   |
[01:30:07]     |                                   variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                   help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/lib.rs:445:47
[01:30:07]     |
[01:30:07] 445 |         let arg = ecx.eval_place(&mir::Place::Local(args.next().unwrap()))?;
[01:30:07]     |                                   |
[01:30:07]     |                                   variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                   help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:07] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:07]    --> src/tools/miri/src/fn_call.rs:271:61
[01:30:07]     |
[01:30:07] 271 |                 let arg_dest = this.eval_place(&mir::Place::Local(arg_local))?;
[01:30:07]     |                                                 |
[01:30:07]     |                                                 variant not found in `rustc::mir::Place<'_>`
[01:30:07]     |                                                 help: did you mean: `local`
[01:30:07] 
[01:30:07] 
[01:30:08] error[E0599]: no variant named `Local` found for type `rustc::mir::Place<'_>` in the current scope
[01:30:08]    --> src/tools/miri/src/tls.rs:154:53
[01:30:08]     |
[01:30:08] 154 |             let dest = this.eval_place(&mir::Place::Local(arg_local))?;
[01:30:08]     |                                         |
[01:30:08]     |                                         variant not found in `rustc::mir::Place<'_>`
[01:30:08]     |                                         help: did you mean: `local`
[01:30:08] 
---
travis_time:end:028bd270:start=1551012905615709395,finish=1551012905623469110,duration=7759715
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:070c78b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02e8fc68
travis_time:start:02e8fc68
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2705f757
$ dmesg | grep -i kill
