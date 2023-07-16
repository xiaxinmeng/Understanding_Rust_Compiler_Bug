plain
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implicit reborrow results in a read-only pointer
    |
781 |         let a = ptr::read(x);
    |                           ^
    |
    |
    = note: cast of `&mut` reference to `*const` pointer causes an implicit reborrow, which converts the reference to `&`, stripping write provenance
    = note: it is UB to write through the resulting pointer, even after casting it to `*mut`
    = note: `-D hidden-reborrow-in-ptr-casts` implied by `-D warnings`
help: to save write provenance, cast to `*mut` pointer first
781 |         let a = ptr::read(x as *mut _);
    |                             +++++++++
    |                             +++++++++
help: to make reborrow explicit, add cast to a shared reference
    |
781 |         let a = ptr::read(x as &_);


error: implicit reborrow results in a read-only pointer
    |
782 |         let b = ptr::read(y);
    |                           ^
    |
    |
    = note: cast of `&mut` reference to `*const` pointer causes an implicit reborrow, which converts the reference to `&`, stripping write provenance
    = note: it is UB to write through the resulting pointer, even after casting it to `*mut`
help: to save write provenance, cast to `*mut` pointer first
    |
782 |         let b = ptr::read(y as *mut _);
    |                             +++++++++
help: to make reborrow explicit, add cast to a shared reference
    |
782 |         let b = ptr::read(y as &_);


error: implicit reborrow results in a read-only pointer
    |
917 |         let result = ptr::read(dest);
    |                                ^^^^
    |
    |
    = note: cast of `&mut` reference to `*const` pointer causes an implicit reborrow, which converts the reference to `&`, stripping write provenance
    = note: it is UB to write through the resulting pointer, even after casting it to `*mut`
help: to save write provenance, cast to `*mut` pointer first
917 |         let result = ptr::read(dest as *mut _);
    |                                     +++++++++
    |                                     +++++++++
help: to make reborrow explicit, add cast to a shared reference
917 |         let result = ptr::read(dest as &_);
    |                                     +++++


error: implicit reborrow results in a read-only pointer
    |
    |
506 |         let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
    |
    |
    = note: cast of `&mut` reference to `*const` pointer causes an implicit reborrow, which converts the reference to `&`, stripping write provenance
    = note: it is UB to write through the resulting pointer, even after casting it to `*mut`
help: to save write provenance, cast to `*mut` pointer first
    |
506 |         let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot as *mut _) });
    |                                                                   +++++++++
help: to make reborrow explicit, add cast to a shared reference
    |
506 |         let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot as &_) });


error: implicit reborrow results in a read-only pointer
    |
    |
559 |     let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
    |
    |
    = note: cast of `&mut` reference to `*const` pointer causes an implicit reborrow, which converts the reference to `&`, stripping write provenance
    = note: it is UB to write through the resulting pointer, even after casting it to `*mut`
help: to save write provenance, cast to `*mut` pointer first
    |
559 |     let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot as *mut _) });
    |                                                               +++++++++
help: to make reborrow explicit, add cast to a shared reference
    |
559 |     let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot as &_) });

error: could not compile `core` due to 5 previous errors
Build completed unsuccessfully in 0:03:26
