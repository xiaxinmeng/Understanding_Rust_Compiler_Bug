plain
    Checking unicode-width v0.1.8
error: suffixes on a string literal are invalid
   --> library/proc_macro/src/bridge/server.rs:165:35
    |
165 |         assert!(!already_running, "same-thread nesting ("reentrance") of proc macro executions is not supported");
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid suffix `reentrance`

error: expected `,`, found `") of proc macro executions is not supported"`
   --> library/proc_macro/src/bridge/server.rs:165:68
    |
165 |         assert!(!already_running, "same-thread nesting ("reentrance") of proc macro executions is not supported");
    |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `,`
    Checking getopts v0.2.21
error: could not compile `proc_macro` due to 2 previous errors
Build completed unsuccessfully in 0:01:24
