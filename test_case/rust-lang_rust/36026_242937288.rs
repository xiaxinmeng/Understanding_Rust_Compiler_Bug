
error: temporary value does not live long enough
   --> src/main.rs:109:30
    |
109 |             let mut stderr = io::stderr().lock();
    |                              ^^^^^^^^^^^^       - temporary goes out of scope at end of statement
    |                              |
    |                              temporary created here
...
114 |         }
    |         - reference to temporary goes out of scope here
    |
    | note: you can use a new variable to extend the lifetime of a temporary
