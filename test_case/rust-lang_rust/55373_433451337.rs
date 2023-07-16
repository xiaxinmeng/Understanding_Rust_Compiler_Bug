plain
travis_time:end:00f6bea5:start=1540566445621560380,finish=1540566504709247763,duration=59087687383
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:23:37]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:30:17]    Compiling rustc_metadata_utils v0.0.0 (/checkout/src/librustc_metadata_utils)
[00:30:17]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:30:17]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:30:17] error: `$delegate:expr` is followed (through repetition) by `->`, which is not allowed for `expr` fragments
[00:30:17]    --> librustc_mir/interpret/snapshot.rs:107:52
[00:30:17]     |
[00:30:17] 107 |         $( $variant:ident $( ( $($field:ident $(-> $delegate:expr)*),* ) )* ),* $(,)*
[00:30:17]     |                                                 -- ^^^^^^^^^^^^^^ this fragment is followed by the first fragment in this repetition without a valid separator
[00:30:17]     |                                                 |
[00:30:17]     |                                                 this is the first fragment in the evaluated repetition
[00:30:17] 
[00:30:17] error: `$delegate:expr` is followed (through repetition) by `->`, which is not allowed for `expr` fragments
[00:30:17]    --> librustc_mir/interpret/snapshot.rs:130:54
[00:30:17]     |
[00:30:17] 130 |     (struct $struct_name:ident { $($field:ident $(-> $delegate:expr)*),*  $(,)* }) => {
[00:30:17]     |                                                   -- ^^^^^^^^^^^^^^ this fragment is followed by the first fragment in this repetition without a valid separator
[00:30:17]     |                                                   |
[00:30:17]     |                                                   this is the first fragment in the evaluated repetition
[00:30:17]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:30:20] error: unused import: `Pointer`
[00:30:20] error: unused import: `Pointer`
[00:30:20]   --> librustc_mir/interpret/snapshot.rs:14:14
[00:30:20] 14 |     AllocId, Pointer, Scalar,
[00:30:20]    |              ^^^^^^^
[00:30:20]    |
[00:30:20]    = note: `-D unused-imports` implied by `-D warnings`
[00:30:20]    = note: `-D unused-imports` implied by `-D warnings`
[00:30:20] 
[00:30:20] error: unused imports: `MemPlace`, `Operand`, `ScalarMaybeUndef`, `Value`
[00:30:20]   --> librustc_mir/interpret/snapshot.rs:28:28
[00:30:20]    |
[00:30:20] 28 | use super::{Frame, Memory, Operand, MemPlace, Place, Value, ScalarMaybeUndef};
[00:30:20] 
[00:30:20] error: unused macro definition
[00:30:20] error: unused macro definition
[00:30:20]    --> librustc_mir/interpret/snapshot.rs:97:1
[00:30:20]     |
[00:30:20] 97  | / macro_rules! __impl_snapshot_field {
[00:30:20] 98  | |     ($field:ident, $ctx:expr) => ($field.snapshot($ctx));
[00:30:20] 99  | |     ($field:ident, $ctx:expr, $delegate:expr) => ($delegate);
[00:30:20]     | |_^
[00:30:20]     |
[00:30:20]     = note: `-D unused-macros` implied by `-D warnings`
[00:30:20] 
[00:30:20] 
[00:30:34] error[E0599]: no method named `snapshot` found for type `&rustc::mir::interpret::Pointer` in the current scope
[00:30:34]    --> librustc_mir/interpret/snapshot.rs:190:45
[00:30:34]     |
[00:30:34] 190 |             Scalar::Ptr(p) => Scalar::Ptr(p.snapshot(ctx)),
[00:30:34]     |
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = note: the following trait defines an item `snapshot`, perhaps you need to implement it:
[00:30:34]             candidate #1: `interpret::snapshot::Snapshot`
[00:30:34] 
[00:30:34] error[E0599]: no method named `snapshot` found for type `&interpret::place::MemPlace` in the current scope
[00:30:34]    --> librustc_mir/interpret/snapshot.rs:244:43
[00:30:34]     |
[00:30:34] 244 |             Place::Ptr(p) => Place::Ptr(p.snapshot(ctx)),
[00:30:34]     |
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = note: the following trait defines an item `snapshot`, perhaps you need to implement it:
[00:30:34]             candidate #1: `interpret::snapshot::Snapshot`
[00:30:34] 
[00:30:34] error[E0599]: no method named `snapshot` found for type `&interpret::eval_context::LocalValue` in the current scope
[00:30:34]    --> librustc_mir/interpret/snapshot.rs:392:53
[00:30:34]     |
[00:30:34] 392 |             locals: locals.iter().map(|local| local.snapshot(ctx)).collect(),
[00:30:34]     |
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = help: items from traits can only be used if the trait is implemented and in scope
[00:30:34]     = note: the following trait defines an item `snapshot`, perhaps you need to implement it:
[00:30:34]             candidate #1: `interpret::snapshot::Snapshot`
[00:30:36] error: aborting due to 8 previous errors
[00:30:36] 
[00:30:36] For more information about this error, try `rustc --explain E0599`.
[00:30:36] error: Could not compile `rustc_mir`.
---
travis_time:end:0262f2c0:start=1540568424284202950,finish=1540568424290020583,duration=5817633
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0cc2f506
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorServitravis_time:start:0b141949
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:058bb33e
travis_time:start:058bb33e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bbe273f
$ dmesg | grep -i kill
