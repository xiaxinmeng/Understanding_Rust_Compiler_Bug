 rust
src/lib.rs:317:32: 317:41 error: transmute called with differently sized types:
unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) {zmq_free_fn} (0 bits) to
*mut unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) (64 bits) [E0512]                                                                                                                  
src/lib.rs:317         let free_fn = unsafe { transmute(zmq_free_fn) };
