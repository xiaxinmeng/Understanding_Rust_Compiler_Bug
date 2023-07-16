plain
2019-11-04T21:04:52.2582312Z [RUSTC-TIMING] backtrace test:false 0.408
2019-11-04T21:04:52.9841956Z [RUSTC-TIMING] hashbrown test:false 0.852
2019-11-04T21:04:53.1819477Z warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
2019-11-04T21:04:53.1824778Z 
2019-11-04T21:04:57.6177297Z error: `extern` fn uses type `(u64, u64)`, which is not FFI-safe
2019-11-04T21:04:57.6178398Z   --> src/libstd/sys/sgx/abi/mod.rs:57:86
2019-11-04T21:04:57.6178947Z    |
2019-11-04T21:04:57.6179612Z 57 | extern "C" fn entry(p1: u64, p2: u64, p3: u64, secondary: bool, p4: u64, p5: u64) -> (u64, u64) {
2019-11-04T21:04:57.6180386Z    |                                                                                      ^^^^^^^^^^ not FFI-safe
2019-11-04T21:04:57.6181548Z    = note: `-D improper-ctypes` implied by `-D warnings`
2019-11-04T21:04:57.6182112Z    = help: consider using a struct instead
2019-11-04T21:04:57.6182696Z    = note: tuples have unspecified layout
2019-11-04T21:04:57.6182978Z 
2019-11-04T21:04:57.6182978Z 
2019-11-04T21:04:57.6241340Z error: `extern` fn uses type `sys::sgx::rwlock::RWLock`, which is not FFI-safe
2019-11-04T21:04:57.6242103Z    --> src/libstd/sys/sgx/rwlock.rs:175:50
2019-11-04T21:04:57.6242641Z     |
2019-11-04T21:04:57.6244102Z 175 | pub unsafe extern "C" fn __rust_rwlock_rdlock(p: *mut RWLock) -> i32 {
2019-11-04T21:04:57.6244935Z     |                                                  ^^^^^^^^^^^ not FFI-safe
2019-11-04T21:04:57.6245937Z     = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-11-04T21:04:57.6246444Z     = note: this struct has unspecified layout
2019-11-04T21:04:57.6246892Z note: type defined here
2019-11-04T21:04:57.6246892Z note: type defined here
2019-11-04T21:04:57.6247330Z    --> src/libstd/sys/sgx/rwlock.rs:8:1
2019-11-04T21:04:57.6247739Z     |
2019-11-04T21:04:57.6248235Z 8   | / pub struct RWLock {
2019-11-04T21:04:57.6248990Z 9   | |     readers: SpinMutex<WaitVariable<Option<NonZeroUsize>>>,
2019-11-04T21:04:57.6249576Z 10  | |     writer: SpinMutex<WaitVariable<bool>>,
2019-11-04T21:04:57.6250520Z     | |_^
2019-11-04T21:04:57.6250699Z 
2019-11-04T21:04:57.6250699Z 
2019-11-04T21:04:57.6264901Z error: `extern` fn uses type `sys::sgx::rwlock::RWLock`, which is not FFI-safe
2019-11-04T21:04:57.6265507Z    --> src/libstd/sys/sgx/rwlock.rs:185:50
2019-11-04T21:04:57.6266173Z     |
2019-11-04T21:04:57.6267200Z 185 | pub unsafe extern "C" fn __rust_rwlock_wrlock(p: *mut RWLock) -> i32 {
2019-11-04T21:04:57.6268397Z     |                                                  ^^^^^^^^^^^ not FFI-safe
2019-11-04T21:04:57.6269066Z     = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-11-04T21:04:57.6269391Z     = note: this struct has unspecified layout
2019-11-04T21:04:57.6269674Z note: type defined here
2019-11-04T21:04:57.6269674Z note: type defined here
2019-11-04T21:04:57.6269975Z    --> src/libstd/sys/sgx/rwlock.rs:8:1
2019-11-04T21:04:57.6270214Z     |
2019-11-04T21:04:57.6270564Z 8   | / pub struct RWLock {
2019-11-04T21:04:57.6270970Z 9   | |     readers: SpinMutex<WaitVariable<Option<NonZeroUsize>>>,
2019-11-04T21:04:57.6271369Z 10  | |     writer: SpinMutex<WaitVariable<bool>>,
2019-11-04T21:04:57.6271998Z     | |_^
2019-11-04T21:04:57.6272070Z 
2019-11-04T21:04:57.6272070Z 
2019-11-04T21:04:57.6287015Z error: `extern` fn uses type `sys::sgx::rwlock::RWLock`, which is not FFI-safe
2019-11-04T21:04:57.6287400Z    --> src/libstd/sys/sgx/rwlock.rs:194:50
2019-11-04T21:04:57.6287656Z     |
2019-11-04T21:04:57.6288202Z 194 | pub unsafe extern "C" fn __rust_rwlock_unlock(p: *mut RWLock) -> i32 {
2019-11-04T21:04:57.6288688Z     |                                                  ^^^^^^^^^^^ not FFI-safe
2019-11-04T21:04:57.6289979Z     = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
2019-11-04T21:04:57.6290324Z     = note: this struct has unspecified layout
2019-11-04T21:04:57.6290626Z note: type defined here
2019-11-04T21:04:57.6290626Z note: type defined here
2019-11-04T21:04:57.6290934Z    --> src/libstd/sys/sgx/rwlock.rs:8:1
2019-11-04T21:04:57.6291186Z     |
2019-11-04T21:04:57.6291558Z 8   | / pub struct RWLock {
2019-11-04T21:04:57.6292363Z 9   | |     readers: SpinMutex<WaitVariable<Option<NonZeroUsize>>>,
2019-11-04T21:04:57.6293095Z 10  | |     writer: SpinMutex<WaitVariable<bool>>,
2019-11-04T21:04:57.6294247Z     | |_^
2019-11-04T21:04:57.6294324Z 
2019-11-04T21:04:57.8361043Z error: aborting due to 4 previous errors
2019-11-04T21:04:57.8361896Z 
---
2019-11-04T21:04:57.9118128Z   local time: Mon Nov  4 21:04:57 UTC 2019
2019-11-04T21:04:58.1794259Z   network time: Mon, 04 Nov 2019 21:04:58 GMT
2019-11-04T21:04:58.1797254Z == end clock drift check ==
2019-11-04T21:05:00.5626307Z 
2019-11-04T21:05:00.5726988Z ##[error]Bash exited with code '1'.
2019-11-04T21:05:00.5781308Z ##[section]Starting: Checkout
2019-11-04T21:05:00.5783255Z ==============================================================================
2019-11-04T21:05:00.5783348Z Task         : Get sources
2019-11-04T21:05:00.5783655Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
