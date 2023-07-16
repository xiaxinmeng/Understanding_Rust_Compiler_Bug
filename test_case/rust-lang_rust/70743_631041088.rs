
    Compiling rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
error[E0616]: field `alloc_map` of struct `rustc_middle::ty::GlobalCtxt` is private
   --> src/librustc_mir_build/hair/pattern/const_to_pat.rs:315:36
    |
315 |                     let data = tcx.alloc_map.lock().unwrap_memory(ptr.alloc_id);
    |                                    ^^^^^^^^^ private field

error[E0599]: no method named `unwrap_memory` found for struct `std::cell::RefMut<'_, rustc_middle::mir::interpret::AllocMap<'_>>` in the current scope
   --> src/librustc_mir_build/hair/pattern/const_to_pat.rs:315:53
    |
315 |                     let data = tcx.alloc_map.lock().unwrap_memory(ptr.alloc_id);
    |                                                     ^^^^^^^^^^^^^ method not found in `std::cell::RefMut<'_, rustc_middle::mir::interpret::AllocMap<'_>>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0616.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `rustc_mir_build`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
