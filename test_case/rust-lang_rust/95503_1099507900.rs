plain

---- builder::tests::defaults::build_cross_compile stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Std {
         target: TargetSelection {
             triple: "A",
             file: None,
---
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "alloc",
<        ],
>        tail_args: [],
     Std {
         target: TargetSelection {
<            triple: "A",
<            file: None,
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "core",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_abort",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "proc_macro",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "test",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
             triple: "B",
         },
         compiler: Compiler {
             stage: 0,
             host: TargetSelection {
---
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "alloc",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "core",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_abort",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "proc_macro",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "test",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "unwind",
<        ],
>        tail_args: [],
 ]

', src/bootstrap/builder/tests.rs:115:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- builder::tests::defaults::build_default stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Std {
         target: TargetSelection {
             triple: "A",
             file: None,
---
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "alloc",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "core",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_abort",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "proc_macro",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "test",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 1,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "unwind",
<        ],
>        tail_args: [],
 ]

', src/bootstrap/builder/tests.rs:41:9


---- builder::tests::defaults::build_stage_0 stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Std {
         target: TargetSelection {
             triple: "A",
             file: None,
---
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "alloc",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "core",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_abort",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "proc_macro",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "test",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 0,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "unwind",
<        ],
>        tail_args: [],
 ]

', src/bootstrap/builder/tests.rs:82:9


---- builder::tests::dist::build_all stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Std {
         target: TargetSelection {
             triple: "A",
             file: None,
---
         tail_args: [],
     },
     Std {
         target: TargetSelection {
<            triple: "A",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
             triple: "B",
         },
         compiler: Compiler {
             stage: 1,
             host: TargetSelection {
---
         tail_args: [],
     },
     Std {
         target: TargetSelection {
<            triple: "B",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
             file: None,
         },
         compiler: Compiler {
             stage: 2,
             stage: 2,
             host: TargetSelection {
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "std",
<        ],
>        tail_args: [],
 ]

', src/bootstrap/builder/tests.rs:504:9


---- builder::tests::dist::build_with_empty_host stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Std {
         target: TargetSelection {
             triple: "A",
             file: None,
---
                 triple: "A",
                 file: None,
             },
         },
<        tail_args: [
<            "-p",
<            "alloc",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "core",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_abort",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "panic_unwind",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "proc_macro",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "std",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "test",
<        ],
<    },
<    Std {
<        target: TargetSelection {
<            triple: "C",
<            file: None,
<        },
<        compiler: Compiler {
<            stage: 2,
<            host: TargetSelection {
<                triple: "A",
<                file: None,
<            },
<        },
<        tail_args: [
<            "-p",
<            "unwind",
<        ],
>        tail_args: [],
 ]

', src/bootstrap/builder/tests.rs:582:9

