
[00:01:39]    --> /checkout/src/bootstrap/config.rs:354:32
[00:01:39]     |
[00:01:39] 343 |             flags.host
[00:01:39]     |             ---------- value moved here
[00:01:39] ...
[00:01:39] 354 |         config.run_host_only = flags.host.is_empty() && !flags.target.is_empty();
[00:01:39]     |                                ^^^^^^^^^^ value used here after move
