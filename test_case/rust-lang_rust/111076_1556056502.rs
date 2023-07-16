plain
[RUSTC-TIMING] gimli test:false 4.718
[RUSTC-TIMING] object test:false 5.098
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error[E0364]: `accept_stream` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:13
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `accept_stream` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:13
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `alloc` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:28
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `alloc` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:28
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `async_queues` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:35
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `async_queues` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:35
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `bind_stream` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:49
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `bind_stream` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:49
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `close` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:62
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `close` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:62
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `connect_stream` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:69
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `connect_stream` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:69
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `exit` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:85
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `exit` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:85
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `flush` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:25:91
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,
   |
   |
note: consider marking `flush` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:25:91
   |
25 |             accept_stream, alloc, async_queues, bind_stream, close, connect_stream, exit, flush,


error[E0364]: `free` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:13
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `free` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:13
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `insecure_time` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:19
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `insecure_time` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:19
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `launch_thread` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:34
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `launch_thread` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:34
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `read` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:49
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `read` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:49
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `read_alloc` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:55
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `read_alloc` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:55
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `send` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:67
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `send` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:67
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `wait` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:73
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `wait` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:73
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `write` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:26:79
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,
   |
   |
note: consider marking `write` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:26:79
   |
26 |             free, insecure_time, launch_thread, read, read_alloc, send, wait, write,


error[E0364]: `do_usercall` is only public within the crate, and cannot be re-exported outside
  --> library/std/src/os/fortanix_sgx/mod.rs:28:51
   |
28 |         pub use crate::sys::abi::usercalls::raw::{do_usercall, Usercalls as UsercallNrs};
   |
   |
note: consider marking `do_usercall` as `pub` in the imported module
  --> library/std/src/os/fortanix_sgx/mod.rs:28:51
   |
28 |         pub use crate::sys::abi::usercalls::raw::{do_usercall, Usercalls as UsercallNrs};


error[E0364]: `free` is only public within the crate, and cannot be re-exported outside
   --> library/std/src/sys/sgx/abi/usercalls/mod.rs:265:9
265 | pub use self::raw::free;
    |         ^^^^^^^^^^^^^^^
    |
    |
note: consider marking `free` as `pub` in the imported module
   --> library/std/src/sys/sgx/abi/usercalls/mod.rs:265:9
265 | pub use self::raw::free;
    |         ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0364`.
