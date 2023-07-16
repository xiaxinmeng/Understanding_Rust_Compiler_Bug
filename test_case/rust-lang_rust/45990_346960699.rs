
error[E0433]: failed to resolve. Use of undeclared type or module `Arc`
   --> src\libstd\sys\windows\os_str.rs:162:18
    |
162 |         unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Slice) }
    |                  ^^^ Use of undeclared type or module `Arc`
error[E0433]: failed to resolve. Use of undeclared type or module `Arc`
   --> src\libstd\sys\windows\os_str.rs:162:32
    |
162 |         unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Slice) }
    |                                ^^^ Use of undeclared type or module `Arc`
error[E0412]: cannot find type `Arc` in this scope
   --> src\libstd\sys\windows\os_str.rs:121:31
    |
121 |     pub fn into_arc(&self) -> Arc<Slice> {
    |                               ^^^ not found in this scope
    |
help: possible candidate is found in another module, you can import it into scope
    |
14  | use alloc::arc::Arc;
    |
error[E0412]: cannot find type `Arc` in this scope
   --> src\libstd\sys\windows\os_str.rs:160:31
    |
160 |     pub fn into_arc(&self) -> Arc<Slice> {
    |                               ^^^ not found in this scope
    |
help: possible candidate is found in another module, you can import it into scope
    |
14  | use alloc::arc::Arc;
    |
error: aborting due to 4 previous errors
error: Could not compile `std`.
