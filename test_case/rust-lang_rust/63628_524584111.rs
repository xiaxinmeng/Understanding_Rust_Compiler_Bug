
// These ones interest me (UPD: All are triaged below).
742 use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
 62 proc-macro derive panicked
 36 failed to resolve: could not find `rustc_serialize` in `{{root}}`
 18 cannot find macro `trace!` in this scope
 12 lifetime name `'s` declared twice in the same scope
 10 cannot find macro `flatdata_intersperse!` in this scope
  5 attributes starting with `rustc` are reserved for use by the `rustc` compiler
  2 lifetime name `'input` declared twice in the same scope
  2 custom attribute panicked
  2 cannot determine resolution for the derive macro `Debug`
  1 proc macro panicked
  1 failed to resolve: use of undeclared type or module `SliceConcatExt`
  1 expected open delimiter
  1 expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `is`
  1 expected module, found unresolved item `crate::mod`

// These ones do not interest me.
191 cannot use `state` because it was mutably borrowed
 94 cannot assign to `self.input.cached_token` because it is borrowed
 38 cannot borrow `v` as immutable because it is also borrowed as mutable
  4 cannot move out of `_` because it is borrowed
  2 cannot borrow `*value` as immutable because it is also borrowed as mutable
  2 cannot borrow `*self` as mutable more than once at a time
  2 `types::log_level_type::LogLevelType` doesn't implement `std::fmt::Debug`
  1 use of moved value: `matches`
  1 type annotations required: cannot resolve `std::boxed::Box<std::iter::Map<std::collections::hash_map::Iter<'_, &T, std::collections::HashSet<&U>>, [closure@src/weighted_rendezvous.rs:490:40: 493:10]>>: std::iter::Iterator`
  1 type annotations required: cannot resolve `std::boxed::Box<std::iter::Map<std::collections::hash_map::Iter<'_, &T, std::collections::HashSet<&U>>, [closure@src/weighted_rendezvous.rs:483:40: 486:10]>>: std::iter::Iterator`
  1 conflicting implementations of trait `std::convert::From<main::LogarithmicKnob>` for type `main::Knob`:
  1 conflicting implementations of trait `std::convert::From<main::LinearKnob>` for type `main::Knob`:
  1 conflicting implementations of trait `main::KnobControl` for type `main::Knob`:
  1 cannot borrow `self.map` as mutable more than once at a time
  1 cannot borrow `*tree` as mutable more than once at a time
  1 cannot borrow `*current` as mutable more than once at a time
  1 assign to part of possibly uninitialized variable: `xgcval`

// Infra
3 linking with `cc` failed: exit code: 1
3 ld returned 1 exit status
1 multiple matching crates for `local_encoding`
1 multiple matching crates for `docmatic`
1 can't find crate for `local_encoding`
1 can't find crate for `docmatic`
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/xi-frontend/xi-term/Cargo.lock needs to be updated but --locked was passed to prevent this
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/Prorok64b/web_crawler/Cargo.lock needs to be updated but --locked was passed to prevent this
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/mneumann/graph-similarity-cmd/Cargo.lock needs to be updated but --locked was passed to prevent this
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/MalteT/mensa/Cargo.lock needs to be updated but --locked was passed to prevent this
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/git-series/git-series/Cargo.lock needs to be updated but --locked was passed to prevent this
1 the lock file /mnt/big/crater/work/ex/beta-1.38-1/sources/1.37.0/gh/0X1A/core-utils/Cargo.lock needs to be updated but --locked was passed to prevent this
1 scenario did not finish successfully: \"2\"\nscenarios:   -> reason: job exited with non-zero exit code: 1\nscenarios: not all scenarios terminated successfully\n"`,
1 scenario did not finish successfully: \"1\"\nscenarios:   -> reason: job exited with non-zero exit code: 1\nscenarios: not all scenarios terminated successfully\n"`', tests/tests.rs:420:9
1 process didn't exit successfully: `/opt/crater/target/debug/deps/test_calls-e2efa9867cc7232e` (signal: 4, SIGILL: illegal instruction)
1 process didn't exit successfully: `/opt/crater/target/debug/deps/smalloca-4a140daaf79cea64` (signal: 4, SIGILL: illegal instruction)
1 process didn't exit successfully: `/opt/crater/target/debug/deps/mujs-5e5e5db7b26bab7e` (signal: 4, SIGILL: illegal instruction)
1 process didn't exit successfully: `/opt/crater/target/debug/deps/esr-2b38354cb0dbd2fa` (signal: 11, SIGSEGV: invalid memory reference)
1 process didn't exit successfully: `/opt/crater/target/debug/deps/consistenttime-f55c7a6e800763f5` (signal: 4, SIGILL: illegal instruction)
