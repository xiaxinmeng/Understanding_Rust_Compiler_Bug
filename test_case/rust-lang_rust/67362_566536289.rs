plain
2019-12-17T13:13:00.6481023Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-17T13:13:00.7565383Z error[E0432]: unresolved import `std::sync::atomic::AtomicU64`
2019-12-17T13:13:00.7565750Z    --> src/librustc_data_structures/sync.rs:320:73
2019-12-17T13:13:00.7565973Z     |
2019-12-17T13:13:00.7566268Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32, AtomicU64};
2019-12-17T13:13:00.7566931Z     |                                                                         |
2019-12-17T13:13:00.7567508Z     |                                                                         no `AtomicU64` in `sync::atomic`
2019-12-17T13:13:00.7567886Z     |                                                                         help: a similar name exists in the module: `AtomicU8`
2019-12-17T13:13:00.7567960Z 
---
2019-12-17T13:13:01.5592689Z   local time: Tue Dec 17 13:13:01 UTC 2019
2019-12-17T13:13:01.8222671Z   network time: Tue, 17 Dec 2019 13:13:01 GMT
2019-12-17T13:13:01.8225060Z == end clock drift check ==
2019-12-17T13:13:06.3127249Z 
2019-12-17T13:13:06.3211962Z ##[error]Bash exited with code '1'.
2019-12-17T13:13:06.3244092Z ##[section]Starting: Checkout
2019-12-17T13:13:06.3245726Z ==============================================================================
2019-12-17T13:13:06.3245812Z Task         : Get sources
2019-12-17T13:13:06.3245875Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
