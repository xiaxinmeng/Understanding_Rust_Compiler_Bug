plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: unexpected closing delimiter: `}`
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1052:1
     |
1047 |             (Strip::None, _) => {}
     |                                 -- block is empty, you might have not meant to close it
1052 | }
     | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
error: mismatched closing delimiter: `)`
   --> compiler/rustc_codegen_ssa/src/back/link.rs:976:61
    |
976 |             if sess.target.is_like_msvc && linker_not_found {
...
988 |                 );
    |                 ^ mismatched closing delimiter

