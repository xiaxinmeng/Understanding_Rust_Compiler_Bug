
test.rs:1:7: 1:17 warning: unknown `deny` attribute: `wrong_span`
test.rs:1 #[deny(wrong_span)];
                 ^~~~~~~~~~
test.rs:3:4: 3:15 warning: unused import
test.rs:3 use std::bigint;
              ^~~~~~~~~~~
test.rs:4:10: 4:17 warning: unused import
test.rs:4 use std::{treemap};
                    ^~~~~~~
test.rs:6:8: 6:11 warning: unused variable: `var`
test.rs:6     let var = 1;
                  ^~~
