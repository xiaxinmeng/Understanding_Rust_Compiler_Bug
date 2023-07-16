
error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:18
    |
82  | /  macro_rules! types {
83  | |      ($(
84  | |          $(#[$doc:meta])*
85  | |          pub struct $name:ident($($fields:tt)*);
...   |
88  | |          #[derive(Copy, Clone, Debug)]
    | |                   ^^^^ in this macro invocation (#2)
...   |
93  | |      )*)
94  | |  }
    | |__- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |  / types! {
9   |  |     /// 128-bit wide integer vector type, x86-specific
10  |  |     ///
11  |  |     /// This type is the same as the `__m128i` type defined by Intel,
...    |
329 |  |     );
330 |  | }
    |  |_- in this macro invocation (#1)
    | 
   ::: library/core/src/marker.rs:393:1
    |
393 | /  pub macro Copy($item:item) {
394 | |      /* compiler built-in */
395 | |  }
    | |__- in this expansion of `#[derive(Copy)]` (#2)

error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:24
    |
82  | /   macro_rules! types {
83  | |       ($(
84  | |           $(#[$doc:meta])*
85  | |           pub struct $name:ident($($fields:tt)*);
...   |
88  | |           #[derive(Copy, Clone, Debug)]
    | |                          ^^^^^ in this macro invocation (#2)
...   |
93  | |       )*)
94  | |   }
    | |___- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |   / types! {
9   |   |     /// 128-bit wide integer vector type, x86-specific
10  |   |     ///
11  |   |     /// This type is the same as the `__m128i` type defined by Intel,
...     |
329 |   |     );
330 |   | }
    |   |_- in this macro invocation (#1)
    | 
   ::: library/core/src/clone.rs:139:1
    |
139 | /   pub macro Clone($item:item) {
140 | |       /* compiler built-in */
141 | |   }
    | |___- in this expansion of `#[derive(Clone)]` (#2)

error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:31
    |
82  | /  macro_rules! types {
83  | |      ($(
84  | |          $(#[$doc:meta])*
85  | |          pub struct $name:ident($($fields:tt)*);
...   |
88  | |          #[derive(Copy, Clone, Debug)]
    | |                                ^^^^^ in this macro invocation (#2)
...   |
93  | |      )*)
94  | |  }
    | |__- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |  / types! {
9   |  |     /// 128-bit wide integer vector type, x86-specific
10  |  |     ///
11  |  |     /// This type is the same as the `__m128i` type defined by Intel,
...    |
329 |  |     );
330 |  | }
    |  |_- in this macro invocation (#1)
    | 
   ::: library/core/src/fmt/mod.rs:618:5
    |
618 | /      pub macro Debug($item:item) {
619 | |          /* compiler built-in */
620 | |      }
    | |______- in this expansion of `#[derive(Debug)]` (#2)

error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:18
    |
82  | /  macro_rules! types {
83  | |      ($(
84  | |          $(#[$doc:meta])*
85  | |          pub struct $name:ident($($fields:tt)*);
...   |
88  | |          #[derive(Copy, Clone, Debug)]
    | |                   ^^^^ in this macro invocation (#2)
...   |
93  | |      )*)
94  | |  }
    | |__- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |  / types! {
9   |  |     /// 128-bit wide integer vector type, x86-specific
10  |  |     ///
11  |  |     /// This type is the same as the `__m128i` type defined by Intel,
...    |
329 |  |     );
330 |  | }
    |  |_- in this macro invocation (#1)
    | 
   ::: library/core/src/marker.rs:393:1
    |
393 | /  pub macro Copy($item:item) {
394 | |      /* compiler built-in */
395 | |  }
    | |__- in this expansion of `#[derive(Copy)]` (#2)

error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:24
    |
82  | /   macro_rules! types {
83  | |       ($(
84  | |           $(#[$doc:meta])*
85  | |           pub struct $name:ident($($fields:tt)*);
...   |
88  | |           #[derive(Copy, Clone, Debug)]
    | |                          ^^^^^ in this macro invocation (#2)
...   |
93  | |       )*)
94  | |   }
    | |___- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |   / types! {
9   |   |     /// 128-bit wide integer vector type, x86-specific
10  |   |     ///
11  |   |     /// This type is the same as the `__m128i` type defined by Intel,
...     |
329 |   |     );
330 |   | }
    |   |_- in this macro invocation (#1)
    | 
   ::: library/core/src/clone.rs:139:1
    |
139 | /   pub macro Clone($item:item) {
140 | |       /* compiler built-in */
141 | |   }
    | |___- in this expansion of `#[derive(Clone)]` (#2)

error: implementation has missing stability attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/macros.rs:88:31
    |
82  | /  macro_rules! types {
83  | |      ($(
84  | |          $(#[$doc:meta])*
85  | |          pub struct $name:ident($($fields:tt)*);
...   |
88  | |          #[derive(Copy, Clone, Debug)]
    | |                                ^^^^^ in this macro invocation (#2)
...   |
93  | |      )*)
94  | |  }
    | |__- in this expansion of `types!` (#1)
    | 
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/mod.rs:8:1
    |
8   |  / types! {
9   |  |     /// 128-bit wide integer vector type, x86-specific
10  |  |     ///
11  |  |     /// This type is the same as the `__m128i` type defined by Intel,
...    |
329 |  |     );
330 |  | }
    |  |_- in this macro invocation (#1)
    | 
   ::: library/core/src/fmt/mod.rs:618:5
    |
618 | /      pub macro Debug($item:item) {
619 | |          /* compiler built-in */
620 | |      }
    | |______- in this expansion of `#[derive(Debug)]` (#2)

error: associated type has missing stability attribute
  --> library/core/src/ptr/metadata.rs:57:5
   |
57 |     type Metadata: Copy + Send + Sync + Ord + Hash + Unpin;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: associated function has missing stability attribute
   --> library/core/src/ptr/metadata.rs:194:5
    |
194 | /     pub fn size_of(self) -> usize {
195 | |         self.vtable_ptr.size_of
196 | |     }
    | |_____^

error: associated function has missing stability attribute
   --> library/core/src/ptr/metadata.rs:200:5
    |
200 | /     pub fn align_of(self) -> usize {
201 | |         self.vtable_ptr.align_of
202 | |     }
    | |_____^

error: associated function has missing stability attribute
   --> library/core/src/ptr/metadata.rs:206:5
    |
206 | /     pub fn layout(self) -> crate::alloc::Layout {
207 | |         // SAFETY: the compiler emitted this vtable for a concrete Rust type which
208 | |         // is known to have a valid layout. Same rationale as in `Layout::for_value`.
209 | |         unsafe { crate::alloc::Layout::from_size_align_unchecked(self.size_of(), self.align_of()) }
210 | |     }
    | |_____^

error: aborting due to 3847 previous errors

error: could not compile `core`
