 bash
steve@warmachine:~/tmp$ rustc asm-flow.rs --cfg overwrite_in_both_with_liveness && ./asm-flow
asm-flow.rs:6:10: 6:25 warning: unknown lint: `dead_assignment`, #[warn(unknown_lints)] on by default
asm-flow.rs:6 #![allow(dead_assignment)]
                       ^~~~~~~~~~~~~~~
steve@warmachine:~/tmp$ rustc asm-flow.rs --cfg augment_in_input && ./asm-flow  
asm-flow.rs:6:10: 6:25 warning: unknown lint: `dead_assignment`, #[warn(unknown_lints)] on by default
asm-flow.rs:6 #![allow(dead_assignment)]
                       ^~~~~~~~~~~~~~~
steve@warmachine:~/tmp$ rustc asm-flow.rs --cfg augment_in_output && ./asm-flow 
asm-flow.rs:6:10: 6:25 warning: unknown lint: `dead_assignment`, #[warn(unknown_lints)] on by default
asm-flow.rs:6 #![allow(dead_assignment)]
                       ^~~~~~~~~~~~~~~
