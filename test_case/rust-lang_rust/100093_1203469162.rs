plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unnecessary parentheses around block return value
   --> compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:620:24
    |
620 |             Some(_) => (0..0), // empty range
    |                        ^    ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
    |
620 -             Some(_) => (0..0), // empty range
620 +             Some(_) => 0..0, // empty range

   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: could not compile `rustc_mir_build` due to previous error
warning: build failed, waiting for other jobs to finish...
