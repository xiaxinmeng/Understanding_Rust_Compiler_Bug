
<anon>:4:12: 4:15 warning: private type in exported type signature, #[warn(visible_private_types)] on by default
<anon>:4     pub x: Bar
                    ^~~
<anon>:11:28: 11:31 warning: private type in exported type signature, #[warn(visible_private_types)] on by default
<anon>:11     pub fn get_x(&self) -> Bar {
                                     ^~~
<anon>:19:5: 21:6 warning: code is never used: `bar`, #[warn(dead_code)] on by default
<anon>:19     pub fn bar(&self) {
<anon>:20         println!("onoez");
<anon>:21     }
