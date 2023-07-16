plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
error[E0282]: type annotations needed for `&T`
   --> compiler/rustc_metadata/src/creader.rs:136:62
    |
136 |         MappedReadGuard::map(tcx.untracked().cstore.read(), |cstore| {
    |                                                              ^^^^^^
137 |             cstore.as_any().downcast_ref::<CStore>().expect("`tcx.cstore` is not a `CStore`")
    |                    ------ type must be known at this point
    |
help: consider giving this closure parameter an explicit type, where the type for type parameter `T` is specified
    |
136 |         MappedReadGuard::map(tcx.untracked().cstore.read(), |cstore: &T| {

error[E0282]: type annotations needed for `&mut T`
   --> compiler/rustc_metadata/src/creader.rs:142:64
    |
    |
142 |         MappedWriteGuard::map(tcx.untracked().cstore.write(), |cstore| {
    |                                                                ^^^^^^
143 |             cstore.untracked_as_any().downcast_mut().expect("`tcx.cstore` is not a `CStore`")
    |                    ---------------- type must be known at this point
    |
help: consider giving this closure parameter an explicit type, where the type for type parameter `T` is specified
    |
142 |         MappedWriteGuard::map(tcx.untracked().cstore.write(), |cstore: &mut T| {

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_metadata` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
