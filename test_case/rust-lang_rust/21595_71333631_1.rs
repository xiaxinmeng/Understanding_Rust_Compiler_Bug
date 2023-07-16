
$ rustc main.rs && ./main 
main.rs:2:12: 2:17 warning: negation of unsigned int literal may be unintentional, #[warn(unsigned_negation)] on by default
main.rs:2   for i in -1u32..3 {
                     ^~~~~

