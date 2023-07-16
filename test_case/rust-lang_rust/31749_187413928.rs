
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/compile-fail/borrowck\borrowck-while-cond.rs:13:11: 13:12 error: use of possibly uninitialized variable: `x` [E0381]
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/compile-fail/borrowck\borrowck-while-cond.rs:13     while x { } //~ ERROR use of possibly uninitialized variable: `x`
                                                                                                             ^
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/compile-fail/borrowck\borrowck-while-cond.rs:13:11: 13:12 help: run `rustc --explain E0381` to see a detailed explanation
error: aborting due to previous error
