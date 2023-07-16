plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error[E0585]: found a documentation comment that doesn't document anything
    |
777 | ///
    | ^^^
    |
    |
    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
error[E0432]: unresolved import `crate::thread::Result`
  --> library/std/src/panic.rs:18:5
   |
18 | use crate::thread::Result;
18 | use crate::thread::Result;
   |     ^^^^^^^^^^^^^^^^^^^^^ no `Result` in `thread`
error[E0432]: unresolved import `crate::thread::Thread`
 --> library/std/src/sync/mpsc/blocking.rs:6:27
  |
  |
6 | use crate::thread::{self, Thread};
  |                           ^^^^^^ no `Thread` in `thread`
error[E0432]: unresolved import `crate::thread::Thread`
  --> library/std/src/sync/once.rs:94:27
   |
   |
94 | use crate::thread::{self, Thread};
   |                           ^^^^^^ no `Thread` in `thread`
error[E0432]: unresolved import `crate::thread::Thread`
 --> library/std/src/sys_common/thread_info.rs:5:5
  |
5 | use crate::thread::Thread;
5 | use crate::thread::Thread;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Thread` in `thread`
error[E0432]: unresolved import `crate::thread::Thread`
  --> library/std/src/rt.rs:32:9
   |
32 |     use crate::thread::Thread;
32 |     use crate::thread::Thread;
   |         ^^^^^^^^^^^^^^^^^^^^^ no `Thread` in `thread`
error[E0432]: unresolved import `crate::thread::JoinHandle`
 --> library/std/src/sys/unix/ext/thread.rs:8:5
  |
8 | use crate::thread::JoinHandle;
8 | use crate::thread::JoinHandle;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `JoinHandle` in `thread`
error: cannot find macro `thread_local` in this scope
   --> library/std/src/panicking.rs:237:5
    |
    |
237 |     thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0) }

error: cannot find macro `thread_local` in this scope
  --> library/std/src/sys_common/thread_info.rs:12:1
   |
   |
12 | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = RefCell::new(None) }

error: cannot find macro `thread_local` in this scope
  --> library/std/src/io/stdio.rs:21:1
   |
   |
21 | thread_local! {

error: cannot find macro `thread_local` in this scope
    --> library/std/src/collections/hash/map.rs:2806:9
     |
     |
2806 |         thread_local!(static KEYS: Cell<(u64, u64)> = {


error[E0425]: cannot find value `KEYS` in this scope
     |
     |
2810 |         KEYS.with(|keys| {


error[E0425]: cannot find value `OUTPUT_CAPTURE` in this scope
    |
    |
907 |     OUTPUT_CAPTURE.with(move |slot| slot.replace(sink))


error[E0425]: cannot find value `OUTPUT_CAPTURE` in this scope
    |
    |
925 |         && OUTPUT_CAPTURE.try_with(|s| {


error[E0425]: cannot find function `current` in module `thread`
  --> library/std/src/sync/mpsc/blocking.rs:31:50
   |
31 |     let inner = Arc::new(Inner { thread: thread::current(), woken: AtomicBool::new(false) });
   |                                                  ^^^^^^^ not found in `thread`
help: consider importing this function
   |
   |
3  | use crate::sys::thread::guard::current;


error[E0425]: cannot find function `park` in module `thread`
  --> library/std/src/sync/mpsc/blocking.rs:68:21
68 |             thread::park()
   |                     ^^^^ not found in `thread`


error[E0425]: cannot find function `park_timeout` in module `thread`
  --> library/std/src/sync/mpsc/blocking.rs:79:21
   |
79 |             thread::park_timeout(end - now)
   |                     ^^^^^^^^^^^^ not found in `thread`

error[E0425]: cannot find function `yield_now` in module `thread`
   --> library/std/src/sync/mpsc/shared.rs:194:63
    |
194 | ...                   mpsc::Inconsistent => thread::yield_now(),
    |                                                     ^^^^^^^^^ not found in `thread`

error[E0425]: cannot find function `yield_now` in module `thread`
   --> library/std/src/sync/mpsc/shared.rs:298:29
298 |                     thread::yield_now();
    |                             ^^^^^^^^^ not found in `thread`


error[E0425]: cannot find function `yield_now` in module `thread`
   --> library/std/src/sync/mpsc/shared.rs:470:29
470 |                     thread::yield_now();
    |                             ^^^^^^^^^ not found in `thread`


error[E0425]: cannot find function `yield_now` in module `thread`
   --> library/std/src/sync/mpsc/stream.rs:416:29
416 |                     thread::yield_now();
    |                             ^^^^^^^^^ not found in `thread`


error[E0425]: cannot find function `current` in module `thread`
    |
    |
448 |             thread: Cell::new(Some(thread::current())),
    |                                            ^^^^^^^ not found in `thread`
help: consider importing this function
    |
    |
90  | use crate::sys::thread::guard::current;


error[E0425]: cannot find function `park` in module `thread`
    |
477 |             thread::park();
    |                     ^^^^ not found in `thread`


error[E0425]: cannot find function `panicking` in module `thread`
   |
   |
31 |         let ret = Guard { panicking: thread::panicking() };
   |                                              ^^^^^^^^^ not found in `thread`
help: consider importing this function
   |
1  | use crate::panicking::panicking;
   |
   |

error[E0425]: cannot find function `panicking` in module `thread`
   |
   |
37 |         if !guard.panicking && thread::panicking() {
   |                                        ^^^^^^^^^ not found in `thread`
help: consider importing this function
   |
1  | use crate::panicking::panicking;
   |
   |

error[E0425]: cannot find value `THREAD_INFO` in this scope
   |
19 |         THREAD_INFO
   |         ^^^^^^^^^^^ not found in this scope


error[E0425]: cannot find value `THREAD_INFO` in this scope
   |
   |
40 |     THREAD_INFO.with(|c| assert!(c.borrow().is_none()));


error[E0425]: cannot find value `THREAD_INFO` in this scope
   |
   |
41 |     THREAD_INFO.with(move |c| *c.borrow_mut() = Some(ThreadInfo { stack_guard, thread }));


error[E0425]: cannot find value `THREAD_INFO` in this scope
   |
   |
45 |     THREAD_INFO.with(move |c| c.borrow_mut().as_mut().unwrap().stack_guard = stack_guard);


error[E0425]: cannot find function `current` in module `thread`
  --> library/std/src/sys_common/util.rs:26:17
   |
26 |         thread::current().name().unwrap_or("<unknown>")
   |                 ^^^^^^^ not found in `thread`
help: consider importing this function
   |
   |
1  | use crate::sys::thread::guard::current;


error[E0425]: cannot find function `panicking` in module `thread`
    |
115 |     if thread::panicking() {
    |                ^^^^^^^^^ not found in `thread`


error[E0425]: cannot find function `panicking` in module `thread`
    |
163 |     if thread::panicking() {
    |                ^^^^^^^^^ not found in `thread`


error[E0425]: cannot find value `LOCAL_PANIC_COUNT` in this scope
    |
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
    |     ------------------------------------------------------------- similarly named static `GLOBAL_PANIC_COUNT` defined here
...
248 |         LOCAL_PANIC_COUNT.with(|c| {
    |         ^^^^^^^^^^^^^^^^^ help: a static with a similar name exists: `GLOBAL_PANIC_COUNT`

error[E0425]: cannot find value `LOCAL_PANIC_COUNT` in this scope
    |
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
    |     ------------------------------------------------------------- similarly named static `GLOBAL_PANIC_COUNT` defined here
...
257 |         LOCAL_PANIC_COUNT.with(|c| {
    |         ^^^^^^^^^^^^^^^^^ help: a static with a similar name exists: `GLOBAL_PANIC_COUNT`

error[E0425]: cannot find value `LOCAL_PANIC_COUNT` in this scope
    |
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
    |     ------------------------------------------------------------- similarly named static `GLOBAL_PANIC_COUNT` defined here
...
265 |         LOCAL_PANIC_COUNT.with(|c| c.get())
    |         ^^^^^^^^^^^^^^^^^ help: a static with a similar name exists: `GLOBAL_PANIC_COUNT`

error[E0425]: cannot find value `LOCAL_PANIC_COUNT` in this scope
    |
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
244 |     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
    |     ------------------------------------------------------------- similarly named static `GLOBAL_PANIC_COUNT` defined here
...
291 |         LOCAL_PANIC_COUNT.with(|c| c.get() == 0)
    |         ^^^^^^^^^^^^^^^^^ help: a static with a similar name exists: `GLOBAL_PANIC_COUNT`
error: unused import: `crate::cell::Cell`
 --> library/std/src/collections/hash/map.rs:9:5
  |
9 | use crate::cell::Cell;
9 | use crate::cell::Cell;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::sys`
  --> library/std/src/collections/hash/map.rs:16:5
   |
16 | use crate::sys;
16 | use crate::sys;
   |     ^^^^^^^^^^

error: unused import: `Cell`
  |
  |
8 | use crate::cell::{Cell, RefCell};

error: unused import: `crate::cell::RefCell`
 --> library/std/src/sys_common/thread_info.rs:3:5
  |
---
    |
233 |     use crate::cell::Cell;
    |         ^^^^^^^^^^^^^^^^^

error: unused import: `AsInner`
 --> library/std/src/sys/unix/ext/thread.rs:7:25
  |
7 | use crate::sys_common::{AsInner, IntoInner};

error: unused import: `IntoInner`
 --> library/std/src/sys/unix/ext/thread.rs:7:34
  |
  |
7 | use crate::sys_common::{AsInner, IntoInner};

error: aborting due to 43 previous errors

Some errors have detailed explanations: E0425, E0432, E0585.
