console
$ MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-check-number-validity" cargo +nightly miri test --test leak_drop
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running tests/leak_drop/main.rs (target/miri/x86_64-apple-darwin/debug/deps/leak_drop-59a0909d5e2ab85f)

running 5 tests
test bytes::dealloc_owned_data ... error: Undefined Behavior: 0x1aa85f is not a valid pointer
   --> /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/rand.rs:130:40
    |
130 |                     let ret = unsafe { f(s.as_mut_ptr() as *mut c_void, s.len()) };
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 0x1aa85f is not a valid pointer
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information

    = note: inside closure at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/rand.rs:130:40
    = note: inside `std::option::Option::<unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> i32>::map::<bool, [closure@std::sys::unix::rand::imp::getentropy_fill_bytes::{closure#0}]>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:909:29
    = note: inside `std::sys::unix::rand::imp::getentropy_fill_bytes` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/rand.rs:125:9
    = note: inside `std::sys::unix::rand::imp::fill_bytes` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/rand.rs:141:12
    = note: inside `std::sys::unix::rand::hashmap_random_keys` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/unix/rand.rs:8:9
    = note: inside `std::collections::hash_map::RandomState::new::KEYS::__init` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2952:23
    = note: inside closure at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:351:25
    = note: inside `std::thread::local::lazy::LazyKeyInner::<std::cell::Cell<(u64, u64)>>::initialize::<[closure@std::collections::hash_map::RandomState::new::KEYS::__getit::{closure#0}]>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:807:25
    = note: inside `std::thread::__FastLocalKeyInner::<std::cell::Cell<(u64, u64)>>::try_initialize::<[closure@std::collections::hash_map::RandomState::new::KEYS::__getit::{closure#0}]>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:985:31
    = note: inside `std::thread::__FastLocalKeyInner::<std::cell::Cell<(u64, u64)>>::get::<[closure@std::collections::hash_map::RandomState::new::KEYS::__getit::{closure#0}]>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:968:29
    = note: inside `std::collections::hash_map::RandomState::new::KEYS::__getit` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:343:21
    = note: inside `std::thread::LocalKey::<std::cell::Cell<(u64, u64)>>::try_with::<[closure@std::collections::hash_map::RandomState::new::{closure#0}], std::collections::hash_map::RandomState>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:442:32
    = note: inside `std::thread::LocalKey::<std::cell::Cell<(u64, u64)>>::with::<[closure@std::collections::hash_map::RandomState::new::{closure#0}], std::collections::hash_map::RandomState>` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:419:9
    = note: inside `std::collections::hash_map::RandomState::new` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2955:9
    = note: inside `<std::collections::hash_map::RandomState as std::default::Default>::default` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:3025:9
    = note: inside `std::collections::HashMap::<&[u8], intaglio::Symbol>::with_capacity` at /Users/lopopolo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:247:53
note: inside `intaglio::bytes::SymbolTable::with_capacity` at /Users/lopopolo/dev/artichoke/intaglio/src/bytes.rs:367:36
   --> /Users/lopopolo/dev/artichoke/intaglio/src/bytes.rs:367:36
    |
367 |             map: ManuallyDrop::new(HashMap::with_capacity(capacity)),
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `bytes::dealloc_owned_data` at tests/leak_drop/bytes.rs:5:21
   --> tests/leak_drop/bytes.rs:5:21
    |
5   |     let mut table = SymbolTable::with_capacity(0);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside closure at tests/leak_drop/bytes.rs:4:1
   --> tests/leak_drop/bytes.rs:4:1
    |
3   |   #[test]
    |   ------- in this procedural macro expansion
4   | / fn dealloc_owned_data() {
5   | |     let mut table = SymbolTable::with_capacity(0);
6   | |     for sym in crate::vectors::byte_symbols() {
7   | |         let symbol = sym;
...   |
18  | |     }
19  | | }
    | |_^
    = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error

error: test failed, to rerun pass '--test leak_drop'
