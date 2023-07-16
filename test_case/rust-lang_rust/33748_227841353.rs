
> 
> failures:
> 
> ---- sync::mpsc::Receiver<T>::recv_timeout_0 stdout ----
>   <anon>:4:33: 4:49 error: use of unstable library feature 'mpsc_recv_timeout' (see issue #34029)
> <anon>:4     use std::sync::mpsc::{self, RecvTimeoutError};
>                                          ^~~~~~~~~~~~~~~~
> <anon>:4:33: 4:49 help: add #![feature(mpsc_recv_timeout)] to the crate attributes to enable
> <anon>:10:16: 10:41 error: use of unstable library feature 'mpsc_recv_timeout' (see issue #34029)
> <anon>:10 assert_eq!(Err(RecvTimeoutError::Timeout), recv.recv_timeout(timeout));
>                          ^~~~~~~~~~~~~~~~~~~~~~~~~
> <anon>:10:1: 10:72 note: in this expansion of assert_eq! (defined in <std macros>)
> <anon>:10:16: 10:41 help: add #![feature(mpsc_recv_timeout)] to the crate attributes to enable
> <anon>:10:49: 10:61 error: use of unstable library feature 'mpsc_recv_timeout' (see issue #34029)
> <anon>:10 assert_eq!(Err(RecvTimeoutError::Timeout), recv.recv_timeout(timeout));
>                                                           ^~~~~~~~~~~~
> <anon>:10:1: 10:72 note: in this expansion of assert_eq! (defined in <std macros>)
> <anon>:10:49: 10:61 help: add #![feature(mpsc_recv_timeout)] to the crate attributes to enable
> error: aborting due to previous error(s)
> thread 'sync::mpsc::Receiver<T>::recv_timeout_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:171
> note: Run with `RUST_BACKTRACE=1` for a backtrace.
> 
> 
> failures:
>     sync::mpsc::Receiver<T>::recv_timeout_0
> 