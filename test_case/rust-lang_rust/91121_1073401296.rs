plain
    Checking addr2line v0.16.0
error[E0252]: the name `stderr` is defined multiple times
   --> library/std/src/io/mod.rs:285:13
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                       ------ previous import of the value `stderr` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |             |
    |             |
    |             `stderr` reimported here
    |             help: remove unnecessary import
    |
    = note: `stderr` must be defined only once in the value namespace of this module
error[E0252]: the name `stdin` is defined multiple times
   --> library/std/src/io/mod.rs:285:21
    |
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                               ----- previous import of the value `stdin` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                     |
    |                     |
    |                     `stdin` reimported here
    |                     help: remove unnecessary import
    |
    = note: `stdin` must be defined only once in the value namespace of this module
error[E0252]: the name `stdout` is defined multiple times
   --> library/std/src/io/mod.rs:285:28
    |
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                                      ------ previous import of the value `stdout` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                            |
    |                            |
    |                            `stdout` reimported here
    |                            help: remove unnecessary import
    |
    = note: `stdout` must be defined only once in the value namespace of this module
error[E0252]: the name `Stderr` is defined multiple times
   --> library/std/src/io/mod.rs:285:36
    |
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                                              ------ previous import of the type `Stderr` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                    |
    |                                    |
    |                                    `Stderr` reimported here
    |                                    help: remove unnecessary import
    |
    = note: `Stderr` must be defined only once in the type namespace of this module
error[E0252]: the name `StderrLock` is defined multiple times
   --> library/std/src/io/mod.rs:285:44
    |
    |
276 | pub use self::stdio::{StderrLock, StdinLock, StdoutLock};
    |                       ---------- previous import of the type `StderrLock` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                            |
    |                                            |
    |                                            `StderrLock` reimported here
    |                                            help: remove unnecessary import
    |
    = note: `StderrLock` must be defined only once in the type namespace of this module
error[E0252]: the name `Stdin` is defined multiple times
   --> library/std/src/io/mod.rs:285:56
    |
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                                                      ----- previous import of the type `Stdin` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                                        |
    |                                                        |
    |                                                        `Stdin` reimported here
    |                                                        help: remove unnecessary import
    |
    = note: `Stdin` must be defined only once in the type namespace of this module
error[E0252]: the name `StdinLock` is defined multiple times
   --> library/std/src/io/mod.rs:285:63
    |
    |
276 | pub use self::stdio::{StderrLock, StdinLock, StdoutLock};
    |                                   --------- previous import of the type `StdinLock` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                                               |
    |                                                               |
    |                                                               `StdinLock` reimported here
    |                                                               help: remove unnecessary import
    |
    = note: `StdinLock` must be defined only once in the type namespace of this module
error[E0252]: the name `Stdout` is defined multiple times
   --> library/std/src/io/mod.rs:285:74
    |
    |
272 | pub use self::stdio::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
    |                                                             ------ previous import of the type `Stdout` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                                                          |
    |                                                                          |
    |                                                                          `Stdout` reimported here
    |                                                                          help: remove unnecessary import
    |
    = note: `Stdout` must be defined only once in the type namespace of this module
error[E0252]: the name `StdoutLock` is defined multiple times
   --> library/std/src/io/mod.rs:285:82
    |
    |
276 | pub use self::stdio::{StderrLock, StdinLock, StdoutLock};
    |                                              ---------- previous import of the type `StdoutLock` here
...
285 |     stdio::{stderr, stdin, stdout, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    |                                                                                  ^^^^^^^^^^ `StdoutLock` reimported here
    |
    = note: `StdoutLock` must be defined only once in the type namespace of this module

error[E0432]: unresolved imports `self::stdio::stderr_locked`, `self::stdio::stdin_locked`, `self::stdio::stdout_locked`
    |
    |
274 | pub use self::stdio::{stderr_locked, stdin_locked, stdout_locked};
    |                       ^^^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^ no `stdout_locked` in `io::stdio`
    |                       |              |
    |                       |              no `stdin_locked` in `io::stdio`
    |                       no `stderr_locked` in `io::stdio`
Some errors have detailed explanations: E0252, E0432.
For more information about an error, try `rustc --explain E0252`.
error: could not compile `std` due to 10 previous errors
Build completed unsuccessfully in 0:01:14
