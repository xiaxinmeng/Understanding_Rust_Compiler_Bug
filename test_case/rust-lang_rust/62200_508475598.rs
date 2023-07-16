
% rustc +1.10.0 issue-62200.rs
issue-62200.rs:11:32: 11:52 error: internal compiler error: ../src/librustc_typeck/astconv.rs:916: anonymous bound region BrAnon(0) in binding but not trait ref
issue-62200.rs:11     F: FnOnce(<X as Ty>::V) -> Option<<X as Ty>::V>
                                                 ^~~~~~~~~~~~~~~~~~~~
