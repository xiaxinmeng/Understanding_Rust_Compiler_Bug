plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0412]: cannot find type `Bytes` in this scope
   --> library/core/src/ffi/c_str.rs:608:28
    |
608 |     pub fn bytes(&self) -> Bytes<'_> {
    |
help: consider importing this struct
    |
1   | use crate::str::Bytes;
---

error[E0308]: mismatched types
   --> library/core/src/ffi/c_str.rs:710:50
    |
710 |             ptr: unsafe { NonNull::new_unchecked(s.inner.as_ptr()) },
    |                           ---------------------- ^^^^^^^^^^^^^^^^ types differ in mutability
    |                           arguments to this function are incorrect
    |
    |
    = note: expected raw pointer `*mut u8`
               found raw pointer `*const i8`
   --> library/core/src/ptr/non_null.rs:197:25
    |
    |
197 |     pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {


error[E0560]: struct `CStrBytes<'a>` has no field named `phantom`
    |
    |
711 |             phantom: PhantomData,
    |             ^^^^^^^ `CStrBytes<'a>` does not have this field
    |
    = note: available fields are: `ptr`, `marker`

error[E0614]: type `NonNull<u8>` cannot be dereferenced
    |
    |
720 |         unsafe { *self.ptr == 0 }

error[E0308]: mismatched types
   --> library/core/src/ffi/c_str.rs:729:33
    |
    |
729 |         unsafe { CStr::from_ptr(self.ptr.as_ptr()) }
    |                  -------------- ^^^^^^^^^^^^^^^^^ expected `i8`, found `u8`
    |                  |
    |                  arguments to this function are incorrect
    |
    = note: expected raw pointer `*const i8`
               found raw pointer `*mut u8`
   --> library/core/src/ffi/c_str.rs:259:25
    |
    |
259 |     pub const unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a CStr {


error[E0614]: type `NonNull<u8>` cannot be dereferenced
    |
756 |             let ret = *self.ptr;
    |                       ^^^^^^^^^

