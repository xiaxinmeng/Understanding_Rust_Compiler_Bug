plain
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
error: for loop over an `Option`. This is more readably written as an `if let` statement
    |
    |
287 |         walk_list!(visitor, visit_block, els);
    |
    |
    = note: `-D for-loop-over-fallibles` implied by `-D warnings`
help: to check pattern in a loop use `while let`
    |
259 |         while let Some(elem) = els);
    |         ~~~~~~~~~~~~~~~    ~~~
help: consider using `if let` to clear intent
    |
259 |         if let Some(elem) = els);


error: for loop over an `Option`. This is more readably written as an `if let` statement
    |
    |
695 |             walk_list!(visitor, visit_block, body);
    |
    |
help: to check pattern in a loop use `while let`
    |
259 |         while let Some(elem) = body);
    |         ~~~~~~~~~~~~~~~    ~~~
help: consider using `if let` to clear intent
    |
259 |         if let Some(elem) = body);

error: could not compile `rustc_ast` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:06:38
