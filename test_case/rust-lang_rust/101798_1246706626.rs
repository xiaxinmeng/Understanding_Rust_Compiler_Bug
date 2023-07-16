plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0412]: cannot find type `RawWakerVTable` in this scope
   |
   |
18 |     const VOID_TABLE: RawWakerVTable = RawWakerVTable::new(|_| VOID_WAKER, |_| {}, |_| {}, |_| {});
   |
help: consider importing one of these items
   |
   |
1  | use core::task::RawWakerVTable;
   |
1  | use std::task::RawWakerVTable;


error[E0433]: failed to resolve: use of undeclared type `RawWakerVTable`
   |
   |
18 |     const VOID_TABLE: RawWakerVTable = RawWakerVTable::new(|_| VOID_WAKER, |_| {}, |_| {}, |_| {});
   |
help: consider importing one of these items
   |
   |
1  | use core::task::RawWakerVTable;
   |
1  | use std::task::RawWakerVTable;


error[E0412]: cannot find type `RawWaker` in this scope
   |
   |
20 |     const VOID_WAKER: RawWaker = RawWaker::new(&(), &VOID_TABLE);
   |
help: consider importing one of these items
   |
   |
1  | use core::task::RawWaker;
   |
1  | use std::task::RawWaker;


error[E0433]: failed to resolve: use of undeclared type `RawWaker`
   |
   |
20 |     const VOID_WAKER: RawWaker = RawWaker::new(&(), &VOID_TABLE);
   |
help: consider importing one of these items
   |
   |
1  | use core::task::RawWaker;
   |
1  | use std::task::RawWaker;

error[E0412]: cannot find type `Waker` in this scope
  --> library/core/tests/task.rs:22:18
   |
   |
22 |     const WAKER: Waker = unsafe { Waker::from_raw(VOID_WAKER) };
   |
help: consider importing one of these items
   |
1  | use core::task::Waker;
1  | use core::task::Waker;
   |
1  | use std::task::Waker;
   |

error[E0433]: failed to resolve: use of undeclared type `Waker`
  --> library/core/tests/task.rs:22:35
   |
22 |     const WAKER: Waker = unsafe { Waker::from_raw(VOID_WAKER) };
   |
help: consider importing one of these items
   |
1  | use core::task::Waker;
1  | use core::task::Waker;
   |
1  | use std::task::Waker;
   |

error[E0412]: cannot find type `Context` in this scope
  --> library/core/tests/task.rs:24:20
   |
24 |     const CONTEXT: Context<'static> = Context::from_waker(&WAKER);
   |
help: consider importing one of these items
   |
1  | use core::task::Context;
1  | use core::task::Context;
   |
1  | use std::task::Context;
   |

error[E0433]: failed to resolve: use of undeclared type `Context`
  --> library/core/tests/task.rs:24:39
   |
24 |     const CONTEXT: Context<'static> = Context::from_waker(&WAKER);
   |
help: consider importing one of these items
   |
1  | use core::task::Context;
1  | use core::task::Context;
   |
1  | use std::task::Context;
   |

error[E0412]: cannot find type `Waker` in this scope
  --> library/core/tests/task.rs:26:31
   |
26 |     const WAKER_REF: &'static Waker = CONTEXT.waker();
   |
help: consider importing one of these items
   |
1  | use core::task::Waker;
