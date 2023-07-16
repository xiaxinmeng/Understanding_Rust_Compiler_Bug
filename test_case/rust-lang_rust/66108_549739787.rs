plain
2019-11-05T09:24:19.7721358Z     Checking crossbeam-deque v0.7.1
2019-11-05T09:24:20.6075824Z     Checking rustc-rayon v0.3.0
2019-11-05T09:24:20.9450406Z     Checking tempfile v3.1.0
2019-11-05T09:24:23.5808677Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-11-05T09:24:23.7608155Z error[E0523]: found two different crates with name `bitflags` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-11-05T09:24:23.7608965Z    |
2019-11-05T09:24:23.7608965Z    |
2019-11-05T09:24:23.7609267Z 36 | use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};
2019-11-05T09:24:23.7609777Z 
2019-11-05T09:24:23.7792254Z error: aborting due to previous error
2019-11-05T09:24:23.7792410Z 
2019-11-05T09:24:23.7893752Z error: could not compile `rustdoc`.
---
2019-11-05T09:24:23.8006521Z   local time: Tue Nov  5 09:24:23 UTC 2019
2019-11-05T09:24:23.8867684Z   network time: Tue, 05 Nov 2019 09:24:23 GMT
2019-11-05T09:24:23.8872164Z == end clock drift check ==
2019-11-05T09:24:25.5415420Z 
2019-11-05T09:24:25.5521952Z ##[error]Bash exited with code '1'.
2019-11-05T09:24:25.5563291Z ##[section]Starting: Checkout
2019-11-05T09:24:25.5565297Z ==============================================================================
2019-11-05T09:24:25.5565395Z Task         : Get sources
2019-11-05T09:24:25.5565481Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
