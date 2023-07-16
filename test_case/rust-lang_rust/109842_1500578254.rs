plain
   Compiling tracing v0.1.35
   Compiling tracing-subscriber v0.3.3
   Compiling rustfix v0.6.1
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error: call to `.clone()` on a reference in this situation does nothing
     |
     |
1114 |                 collisions.push((path, ancestor.clone()));
     |                                                ^^^^^^^^ unnecessary method call
     |
     = note: the type `&Path` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
     = note: `-D noop-method-call` implied by `-D warnings`
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:05:05
