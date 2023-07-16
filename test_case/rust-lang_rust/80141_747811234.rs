plain
GITHUB_ACTION=run6
GITHUB_ACTIONS=true
GITHUB_ACTION_REF=
GITHUB_ACTION_REPOSITORY=
GITHUB_ACTOR=dns2utf8
GITHUB_BASE_REF=master
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_752f457a-29e2-4b1a-8d42-c8033f950362
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=atomic_ops
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_752f457a-29e2-4b1a-8d42-c8033f950362
GITHUB_REF=refs/pull/80141/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=429416612
GITHUB_RUN_NUMBER=21590
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-i186godh/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpmnpxcqc4pip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 9000/11176
.................................................................................................... 9100/11176
....................................................................i......i........................ 9200/11176
.................................................................................................... 9300/11176
.......iiiiii..iiiii.ii............................................................................. 9400/11176
.................................................................................................... 9600/11176
.................................................................................................... 9700/11176
.................................................................................................... 9800/11176
.................................................................................................... 9900/11176
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.080 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

 finished in 2.479 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 2200/2811
.................................................................................................... 2300/2811
.................................................................................................... 2400/2811
.....................................i.............................................i................ 2500/2811
.i.F.....................i..F.F..................iFF....................Fi...F...................iF. 2600/2811
.F....................F.........i.F.....................i.F.F...................i...FF.............. 2700/2811
....i.F.F..................Fi..F......................F............................................. 2800/2811
failures:

---- src/sync/atomic.rs - sync::atomic::AtomicI16::add_assign (line 2212) stdout ----
---- src/sync/atomic.rs - sync::atomic::AtomicI16::add_assign (line 2212) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI16::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI16::sub_assign (line 2212) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI16::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI32::add_assign (line 2250) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI32::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI32::sub_assign (line 2250) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI32::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI64::add_assign (line 2288) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI64::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI64::sub_assign (line 2288) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI64::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI8::add_assign (line 2174) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI8::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI8::sub_assign (line 2174) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicI8::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicIsize::add_assign (line 2409) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicIsize::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicIsize::sub_assign (line 2409) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicIsize::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU16::add_assign (line 2231) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU16::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU16::sub_assign (line 2231) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU16::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU32::add_assign (line 2269) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU32::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU32::sub_assign (line 2269) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU32::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU64::add_assign (line 2307) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU64::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU64::sub_assign (line 2307) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU64::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU8::add_assign (line 2193) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU8::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU8::sub_assign (line 2193) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicU8::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicUsize::add_assign (line 2409) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicUsize::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two += 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicUsize::sub_assign (line 2409) stdout ----
error[E0596]: cannot borrow `atomic_forty_two` as mutable, as it is not declared as mutable
  |
  |
6 | let atomic_forty_two = AtomicUsize::new(42);
  |     ---------------- help: consider changing this to be mutable: `mut atomic_forty_two`
7 | atomic_forty_two -= 42;
  | ^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:42
