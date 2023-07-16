plain
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
error: expected one of `.`, `;`, `?`, `}`, or an operator, found `#`
   --> library/test/src/formatters/json.rs:136:17
134 |                 )
134 |                 )
    |                  - expected one of `.`, `;`, `?`, `}`, or an operator
135 | 
136 |                 #[cfg(bootstrap)]

error: could not compile `test` due to previous error
Build completed unsuccessfully in 0:01:32
