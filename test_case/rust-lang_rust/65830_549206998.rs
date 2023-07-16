plain
2019-11-04T02:19:13.4048301Z [RUSTC-TIMING] rls_data test:false 2.245
2019-11-04T02:19:13.4071299Z    Compiling rand v0.7.0
2019-11-04T02:19:15.2308162Z [RUSTC-TIMING] parking_lot test:false 2.133
2019-11-04T02:19:15.2351709Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-04T02:19:15.6888992Z error: function is never used: `query`
2019-11-04T02:19:15.6889356Z   --> src/librustc_macros/src/query.rs:16:26
2019-11-04T02:19:15.6889609Z    |
2019-11-04T02:19:15.6890505Z 16 |     syn::custom_keyword!(query);
2019-11-04T02:19:15.6891430Z    |
2019-11-04T02:19:15.6891685Z    = note: `-D dead-code` implied by `-D warnings`
2019-11-04T02:19:15.6897365Z 
2019-11-04T02:19:15.6897365Z 
2019-11-04T02:19:15.6920029Z error: function is never used: `Keywords`
2019-11-04T02:19:15.6920368Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-11-04T02:19:15.6920604Z    |
2019-11-04T02:19:15.6921022Z 13 |     syn::custom_keyword!(Keywords);
2019-11-04T02:19:15.6927087Z 
2019-11-04T02:19:15.6947147Z error: function is never used: `Symbols`
2019-11-04T02:19:15.6947453Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-11-04T02:19:15.6947684Z    |
2019-11-04T02:19:15.6947684Z    |
2019-11-04T02:19:15.6947937Z 14 |     syn::custom_keyword!(Symbols);
2019-11-04T02:19:15.6990453Z 
2019-11-04T02:19:15.7037468Z error: aborting due to 3 previous errors
2019-11-04T02:19:15.7040769Z 
2019-11-04T02:19:15.7154498Z [RUSTC-TIMING] rustc_macros test:false 0.477
---
2019-11-04T02:19:16.7758521Z   local time: Mon Nov  4 02:19:16 UTC 2019
2019-11-04T02:19:17.0410112Z   network time: Mon, 04 Nov 2019 02:19:17 GMT
2019-11-04T02:19:17.0414067Z == end clock drift check ==
2019-11-04T02:19:18.2094184Z 
2019-11-04T02:19:18.2172886Z ##[error]Bash exited with code '1'.
2019-11-04T02:19:18.2227459Z ##[section]Starting: Checkout
2019-11-04T02:19:18.2229228Z ==============================================================================
2019-11-04T02:19:18.2229302Z Task         : Get sources
2019-11-04T02:19:18.2229387Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
