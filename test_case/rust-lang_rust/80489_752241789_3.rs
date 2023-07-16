
>   ---- [ui] ui/unleash-the-miri-inside-of-you-here/call-non-const-from-const-fn.rs stdout ----
>   normalized stdout:
>   Yay I noticed the unleash_the_miri_inside_of_you_here attribute!!!
>   
>   
>   
>   The actual stdout differed from the expected stdout.
>   Actual stdout saved to /home/albin/rust/rustc/build/x86_64-unknown-linux-gnu/test/ui/unleash-the-miri-inside-of-you-here/call-non-const-from-const-fn/call-non-const-from-const-fn.stdout
>   normalized stderr:
>   error: any use of this value will cause an error
>     --> $DIR/call-non-const-from-const-fn.rs:12:5
>      |
>   LL |     const FOO: bool = call_non_const_fn_unleashed();
>      |     ------------------------------------------------
>   ...
>   LL |     non_const_fn()
>      |     ^^^^^^^^^^^^^^
>      |     |
>      |     calling non-const function `non_const_fn`
>      |     inside `call_non_const_fn_unleashed` at $DIR/call-non-const-from-const-fn.rs:12:5
>      |     inside `FOO` at $DIR/call-non-const-from-const-fn.rs:5:23
>   