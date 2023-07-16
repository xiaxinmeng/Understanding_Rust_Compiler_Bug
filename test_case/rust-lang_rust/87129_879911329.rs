plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: useless assignment of variable of type `StrCursor` to itself
   --> compiler/rustc_builtin_macros/src/format_foreign.rs:347:17
345 | /         macro_rules! move_to {
345 | /         macro_rules! move_to {
346 | |             ($cur:expr) => {{
347 | |                 at = $cur;
    | |                 ^^
348 | |                 let (c_, next_) = at.next_cp()?;
351 | |             }};
352 | |         }
352 | |         }
    | |_________- in this expansion of `move_to!`
...
411 |                       move_to!(at);
    |
    |
    = note: `-D dead-code` implied by `-D warnings`

error: useless assignment of variable of type `StrCursor` to itself
   --> compiler/rustc_builtin_macros/src/format_foreign.rs:347:17
345 | /         macro_rules! move_to {
345 | /         macro_rules! move_to {
346 | |             ($cur:expr) => {{
347 | |                 at = $cur;
    | |                 ^^
348 | |                 let (c_, next_) = at.next_cp()?;
351 | |             }};
352 | |         }
352 | |         }
    | |_________- in this expansion of `move_to!`
...
438 |                       move_to!(at);


error: useless assignment of variable of type `StrCursor` to itself
   --> compiler/rustc_builtin_macros/src/format_foreign.rs:347:17
345 | /         macro_rules! move_to {
345 | /         macro_rules! move_to {
346 | |             ($cur:expr) => {{
347 | |                 at = $cur;
    | |                 ^^
348 | |                 let (c_, next_) = at.next_cp()?;
351 | |             }};
352 | |         }
352 | |         }
    | |_________- in this expansion of `move_to!`
...
468 |                       move_to!(at);


error: useless assignment of variable of type `StrCursor` to itself
   --> compiler/rustc_builtin_macros/src/format_foreign.rs:347:17
345 | /         macro_rules! move_to {
345 | /         macro_rules! move_to {
346 | |             ($cur:expr) => {{
347 | |                 at = $cur;
    | |                 ^^
348 | |                 let (c_, next_) = at.next_cp()?;
351 | |             }};
352 | |         }
352 | |         }
    | |_________- in this expansion of `move_to!`
...
532 |                       move_to!(at);

error: aborting due to 4 previous errors

error: could not compile `rustc_builtin_macros`
