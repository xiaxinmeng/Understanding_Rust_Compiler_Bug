
error[E0425]: cannot find function `forget` in this scope
--> library/std/src/sys/unix/locks/pthread_rwlock.rs:27:13
|
27|             forget(rwlock);
| ^^^^^^not found in this scope
|
help: consider importing one of these items
|
1| use core::intrinsics::forget;
|
1| use core::mem::forget;
|
1| use crate::intrinsics::forget;
|
1| use crate::mem::forget;
|
