
$ ./opts_present_test -h --tandy
    help: opt_present: false, opts_present:  true
       h: opt_present:  true, opts_present:  true
   tandy: opt_present:  true, opts_present:  true
       t: opt_present: false, opts_present:  true

whatever: opts_present: false
       w: opts_present: false
rust: ~"No option \'whatever\' defined"
rust: task failed at 'explicit failure', /Users/illo/libexec/rust-copy/src/libstd/getopts.rs:351
