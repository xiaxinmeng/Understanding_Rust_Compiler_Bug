plain
   Compiling tracing v0.1.35
   Compiling tracing-subscriber v0.3.3
   Compiling rustfix v0.6.1
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error: call to `.clone()` on a reference in this situation does nothing
     |
     |
1139 |                 collisions.push((path, ancestor.clone()));
     |                                                ^^^^^^^^ unnecessary method call
     |
     = note: the type `Path` does not implement `Clone`, so calling `clone` on `&Path` copies the reference, which does not do anything and can be removed
     = note: `-D noop-method-call` implied by `-D warnings`
error: could not compile `compiletest` (bin "compiletest" test) due to previous error
Build completed unsuccessfully in 0:05:49
