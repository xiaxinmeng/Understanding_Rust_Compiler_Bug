bash
error: variable does not need to be mutable
   --> src/utils/async_mutex.rs:154:35
    |
154 |                         .map_err(|_| AsyncMutexError::AwakenerCanceled)?
    |                                   ^ help: remove this `mut`

...

error: variable does not need to be mutable
   --> src/timer.rs:153:19
    |
153 |             .map(|_| tm.create_client())
    |                   ^ help: remove this `mut`
    |
note: lint level defined here
   --> src/timer.rs:40:9
    |
40  | #![deny(warnings)]
    |         ^^^^^^^^
