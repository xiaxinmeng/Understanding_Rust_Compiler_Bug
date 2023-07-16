plain
2019-10-26T23:35:28.9191158Z [RUSTC-TIMING] crossbeam_epoch test:false 1.367
2019-10-26T23:35:28.9221333Z    Compiling rand v0.7.0
2019-10-26T23:35:30.7689104Z [RUSTC-TIMING] parking_lot test:false 2.176
2019-10-26T23:35:30.7698955Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-26T23:35:31.2496151Z warning: function is never used: `query`
2019-10-26T23:35:31.2496913Z   --> src/librustc_macros/src/query.rs:16:26
2019-10-26T23:35:31.2497224Z    |
2019-10-26T23:35:31.2497983Z 16 |     syn::custom_keyword!(query);
2019-10-26T23:35:31.2498885Z    |
2019-10-26T23:35:31.2499191Z    = note: `#[warn(dead_code)]` on by default
2019-10-26T23:35:31.2499256Z 
2019-10-26T23:35:31.2499256Z 
2019-10-26T23:35:31.2516581Z warning: function is never used: `Keywords`
2019-10-26T23:35:31.2518208Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-10-26T23:35:31.2518527Z    |
2019-10-26T23:35:31.2519125Z 13 |     syn::custom_keyword!(Keywords);
2019-10-26T23:35:31.2520062Z 
2019-10-26T23:35:31.2544026Z warning: function is never used: `Symbols`
2019-10-26T23:35:31.2544382Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-10-26T23:35:31.2544633Z    |
2019-10-26T23:35:31.2544633Z    |
2019-10-26T23:35:31.2544908Z 14 |     syn::custom_keyword!(Symbols);
2019-10-26T23:35:31.2545266Z 
2019-10-26T23:35:33.1728586Z [RUSTC-TIMING] rand test:false 4.245
2019-10-26T23:35:33.1728586Z [RUSTC-TIMING] rand test:false 4.245
2019-10-26T23:35:33.6589240Z error: function is never used: `query`
2019-10-26T23:35:33.6589819Z   --> src/librustc_macros/src/query.rs:16:26
2019-10-26T23:35:33.6590187Z    |
2019-10-26T23:35:33.6590545Z 16 |     syn::custom_keyword!(query);
2019-10-26T23:35:33.6591252Z    |
2019-10-26T23:35:33.6591601Z    = note: `-D dead-code` implied by `-D warnings`
2019-10-26T23:35:33.6591681Z 
2019-10-26T23:35:33.6591681Z 
2019-10-26T23:35:33.6609334Z error: function is never used: `Keywords`
2019-10-26T23:35:33.6609802Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-10-26T23:35:33.6610138Z    |
2019-10-26T23:35:33.6610551Z 13 |     syn::custom_keyword!(Keywords);
2019-10-26T23:35:33.6611071Z 
2019-10-26T23:35:33.6628448Z error: function is never used: `Symbols`
2019-10-26T23:35:33.6628942Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-10-26T23:35:33.6629293Z    |
2019-10-26T23:35:33.6629293Z    |
2019-10-26T23:35:33.6629676Z 14 |     syn::custom_keyword!(Symbols);
2019-10-26T23:35:33.6630173Z 
2019-10-26T23:35:33.6726032Z error: aborting due to 3 previous errors
2019-10-26T23:35:33.6726172Z 
2019-10-26T23:35:33.6862861Z [RUSTC-TIMING] rustc_macros test:false 0.506
---
2019-10-26T23:35:40.5378272Z   local time: Sat Oct 26 23:35:40 UTC 2019
2019-10-26T23:35:41.0605701Z   network time: Sat, 26 Oct 2019 23:35:41 GMT
2019-10-26T23:35:41.0606700Z == end clock drift check ==
2019-10-26T23:35:42.3703076Z 
2019-10-26T23:35:42.3825789Z ##[error]Bash exited with code '1'.
2019-10-26T23:35:42.3862631Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T23:35:42.3866617Z ==============================================================================
2019-10-26T23:35:42.3866735Z Task         : Bash
2019-10-26T23:35:42.3866848Z Description  : Run a Bash script on macOS, Linux, or Windows
