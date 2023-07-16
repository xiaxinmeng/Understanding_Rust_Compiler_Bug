plain
[RUSTC-TIMING] gimli test:false 6.227
[RUSTC-TIMING] object test:false 7.190
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: strict provenance disallows casting integer `usize` to pointer `*mut u8`
   --> library/std/src/sys/sgx/abi/panic.rs:14:42
    |
14  |     unsafe { UserRef::from_raw_parts_mut(1 as *mut u8, 0) }
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
   --> library/std/src/lib.rs:223:9
    |
    |
223 | #![deny(fuzzy_provenance_casts)]
    |         ^^^^^^^^^^^^^^^^^^^^^^
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
14  |     unsafe { UserRef::from_raw_parts_mut((...).with_addr(1), 0) }


error: strict provenance disallows casting integer `u64` to pointer `*const T`
 --> library/std/src/sys/sgx/abi/mem.rs:6:5
6 |     (image_base() + offset) as *const T
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  |
  = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
  |
6 |     (...).with_addr((image_base() + offset))


error: strict provenance disallows casting integer `u64` to pointer `*mut T`
  --> library/std/src/sys/sgx/abi/mem.rs:12:5
12 |     (image_base() + offset) as *mut T
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
12 |     (...).with_addr((image_base() + offset))


error: strict provenance disallows casting integer `usize` to pointer `*mut u8`
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:222:17
    |
222 |                 T::align_of() as _ // dangling pointer ok for size 0
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
222 |                 (...).with_addr(T::align_of()) // dangling pointer ok for size 0


error: strict provenance disallows casting integer `u64` to pointer `*const T`
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:83:17
78  | / macro_rules! define_ra {
78  | / macro_rules! define_ra {
79  | |     (< $i:ident > $t:ty) => {
80  | |         #[unstable(feature = "sgx_platform", issue = "56975")]
81  | |         impl<$i> RegisterArgument for $t {
82  | |             fn from_register(a: Register) -> Self {
83  | |                 a as _
...   |
111 | |     };
112 | | }
    | |_- in this expansion of `define_ra!`
    | |_- in this expansion of `define_ra!`
...
124 |   define_ra!(<T> *const T);
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
83  |                 (...).with_addr(a)


error: strict provenance disallows casting integer `u64` to pointer `*mut T`
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:83:17
78  | / macro_rules! define_ra {
78  | / macro_rules! define_ra {
79  | |     (< $i:ident > $t:ty) => {
80  | |         #[unstable(feature = "sgx_platform", issue = "56975")]
81  | |         impl<$i> RegisterArgument for $t {
82  | |             fn from_register(a: Register) -> Self {
83  | |                 a as _
...   |
111 | |     };
112 | | }
    | |_- in this expansion of `define_ra!`
    | |_- in this expansion of `define_ra!`
...
125 |   define_ra!(<T> *mut T);
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
83  |                 (...).with_addr(a)


error: strict provenance disallows casting integer `u64` to pointer `*mut T`
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:140:22
140 |         NonNull::new(a as _)
    |                      ^^^^^^
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
140 |         NonNull::new((...).with_addr(a))


error: strict provenance disallows casting integer `i32` to pointer `*mut T`
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:143:21
    |
143 |         self.map_or(0 as _, NonNull::as_ptr) as _
    |
    |
    = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
    |
143 |         self.map_or((...).with_addr(0), NonNull::as_ptr) as _


error: strict provenance disallows casting integer `u64` to pointer `*const *const u8`
  --> library/std/src/sys/sgx/abi/mod.rs:89:37
   |
89 |             let ret = main(p2 as _, p1 as _);
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
89 |             let ret = main(p2 as _, (...).with_addr(p1));
   |                                     ++++++++++++++++  ~

error: strict provenance disallows casting integer `i32` to pointer `*mut UnsafeListEntry<T>`
  --> library/std/src/sys/sgx/waitqueue/unsafe_list.rs:34:65
   |
34 |         unsafe { UnsafeList { head_tail: NonNull::new_unchecked(1 as _), head_tail_entry: None } }
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
34 |         unsafe { UnsafeList { head_tail: NonNull::new_unchecked((...).with_addr(1)), head_tail_entry: None } }


error: strict provenance disallows casting integer `usize` to pointer `*const Vec<OsString>`
  --> library/std/src/sys/sgx/args.rs:27:25
   |
27 |     let args = unsafe { (ARGS.load(Ordering::Relaxed) as *const ArgsStore).as_ref() };
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
27 |     let args = unsafe { ((...).with_addr(ARGS.load(Ordering::Relaxed))).as_ref() };


error: strict provenance disallows casting integer `usize` to pointer `*const sync::mutex::Mutex<map::HashMap<OsString, OsString>>`
  --> library/std/src/sys/sgx/os.rs:89:14
   |
89 |     unsafe { (ENV.load(Ordering::Relaxed) as *const EnvStore).as_ref() }
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
89 |     unsafe { ((...).with_addr(ENV.load(Ordering::Relaxed))).as_ref() }


error: strict provenance disallows casting integer `usize` to pointer `*const sync::mutex::Mutex<map::HashMap<OsString, OsString>>`
  --> library/std/src/sys/sgx/os.rs:96:16
   |
96 |     unsafe { &*(ENV.load(Ordering::Relaxed) as *const EnvStore) }
   |
   |
   = help: if you can't comply with strict provenance and don't have a pointer with the correct provenance you can use `std::ptr::from_exposed_addr()` instead
help: use `.with_addr()` to adjust a valid pointer in the same allocation, to this address
   |
96 |     unsafe { &*((...).with_addr(ENV.load(Ordering::Relaxed))) }

[RUSTC-TIMING] std test:false 4.107
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 13 previous errors; 1 warning emitted
