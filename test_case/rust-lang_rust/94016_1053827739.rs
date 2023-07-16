plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: expected one of `!`, `(`, `+`, `::`, `<`, `>`, or `as`, found `r#"assertion failed: `({left_name} {} {right_name})`
         {left_name}: `{:?}`,
        {right_name}: `{:?}`: {}"#`
    |
    |
231 |   <<<<<<< HEAD
    |               - expected one of 7 possible tokens
232 | /             r#"assertion failed: `({left_name} {} {right_name})`
233 | |   {left_name}: `{:?}`,
234 | |  {right_name}: `{:?}`: {}"#,
    | |___________________________^ unexpected token
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:10
