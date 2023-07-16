
$ rustc 5723.rs && echo Running: && ./5723
5723.rs:7:8: 7:15 warning: unused variable: `oh_noes` [-W unused-variable (default)]
5723.rs:7     let oh_noes = ~[666];
                  ^~~~~~~
5723.rs:4:26: 4:36 warning: unnecessary allocation, the sigil can be removed [-W unnecessary-allocation (default)]
5723.rs:4         let ss: &[~str] = ~[~"sup?"];
                                    ^~~~~~~~~~
Running:
Segmentation fault
