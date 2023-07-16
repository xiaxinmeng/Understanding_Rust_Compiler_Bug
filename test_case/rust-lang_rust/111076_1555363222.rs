plain
[RUSTC-TIMING] gimli test:false 3.998
[RUSTC-TIMING] object test:false 4.504
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: type `fortanix_sgx_abi::ByteBuffer` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:839:5
839 |     pub fn copy_user_buffer(&self) -> Vec<u8> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: `-D exported-private-dependencies` implied by `-D warnings`

error: type `fortanix_sgx_abi::ByteBuffer` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:227:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
227 | |         pub unsafe fn $f($n1: $t1, $n2: $t2) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::ByteBuffer` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::ByteBuffer` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:195:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
195 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3, $n4: $t4) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::FifoDescriptor<fortanix_sgx_abi::Usercall>` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::Usercall` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::FifoDescriptor<fortanix_sgx_abi::Return>` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::Return` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::FifoDescriptor<fortanix_sgx_abi::Cancel>` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }


error: type `fortanix_sgx_abi::Cancel` from private dependency 'fortanix_sgx_abi' in public interface
   --> library/std/src/sys/sgx/abi/usercalls/raw.rs:211:9
60  | / macro_rules! define_usercalls {
60  | / macro_rules! define_usercalls {
61  | |     ($(fn $f:ident($($n:ident: $t:ty),*) $(-> $r:tt)*; )*) => {
62  | |         /// Usercall numbers as per the ABI.
63  | |         #[repr(u64)]
...   |
74  | |         $(enclave_usercalls_internal_define_usercalls!(def fn $f($($n: $t),*) $(-> $r)*);)*
75  | |     };
76  | | }
    | |_- in this expansion of `define_usercalls!` (#2)
...
...
188 | / macro_rules! enclave_usercalls_internal_define_usercalls {
189 | |     (def fn $f:ident($n1:ident: $t1:ty, $n2:ident: $t2:ty,
190 | |                      $n3:ident: $t3:ty, $n4:ident: $t4:ty) -> $r:tt) => (
191 | |         /// This is the raw function definition, see the ABI documentation for
...   |
211 | |         pub unsafe fn $f($n1: $t1, $n2: $t2, $n3: $t3) -> $r {
...   |
266 | |     );
267 | | }
267 | | }
    | |_- in this expansion of `enclave_usercalls_internal_define_usercalls!` (#3)
268 |
269 |   invoke_with_usercalls!(define_usercalls);
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/fortanix-sgx-abi-0.5.0/src/lib.rs:873:43
    |
    |
873 |           { define_invoke_with_usercalls!(@ [$($accumulated)* $(fn $f($($n: $t),*) $(-> $r)*;)*] $($remainder)*); };
874 | |     // visit modules
874 | |     // visit modules
875 | |     (@ $accumulated:tt $(#[$meta:meta])* pub mod $modname:ident { $($contents:tt)* } $($remainder:tt)*) =>
876 | |         { define_invoke_with_usercalls!(@ $accumulated $($contents)* $($remainder)*); };
892 | |         macro_rules! invoke_with_usercalls {
892 | |         macro_rules! invoke_with_usercalls {
    | |         ---------------------------------- in this expansion of `invoke_with_usercalls!` (#1)
893 | |             ($m:ident) => { $m! $accumulated; }

[RUSTC-TIMING] std test:false 2.292
warning: `std` (lib) generated 1 warning
error: could not compile `std` (lib) due to 11 previous errors; 1 warning emitted
