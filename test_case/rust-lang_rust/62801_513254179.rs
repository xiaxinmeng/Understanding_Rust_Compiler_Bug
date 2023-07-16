plain
2019-07-19T14:05:54.7199428Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T14:05:54.7365637Z ##[command]git config gc.auto 0
2019-07-19T14:05:54.7420297Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T14:05:54.7468863Z ##[command]git config --get-all http.proxy
2019-07-19T14:05:54.7596240Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62801/merge:refs/remotes/pull/62801/merge
---
2019-07-19T14:06:28.5853267Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T14:06:28.5853363Z 
2019-07-19T14:06:28.5853599Z   git checkout -b <new-branch-name>
2019-07-19T14:06:28.5853629Z 
2019-07-19T14:06:28.5853679Z HEAD is now at e91545633 Merge 3427a14bdf19437c455e54af704e0250ea8aa79b into f9477a77c52af8d3dea361b3f4ac3e60653aa529
2019-07-19T14:06:28.5990759Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T14:06:28.5993678Z ==============================================================================
2019-07-19T14:06:28.5993740Z Task         : Bash
2019-07-19T14:06:28.5993808Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T14:36:47.8877949Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-07-19T14:36:48.5330715Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-07-19T14:36:49.1847875Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-07-19T14:36:50.0408171Z    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-19T14:36:50.1960617Z error[E0522]: definition of an unknown language item: `i128_add`
2019-07-19T14:36:50.1968023Z    |
2019-07-19T14:36:50.1973258Z 94 | |     (
2019-07-19T14:36:50.1973258Z 94 | |     (
2019-07-19T14:36:50.1975464Z    | |_______________________^ definition of unknown language item `i128_add`
2019-07-19T14:36:50.1980111Z ...
2019-07-19T14:36:50.2023937Z 291|           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2025094Z 
2019-07-19T14:36:50.2025094Z 
2019-07-19T14:36:50.2025458Z error[E0522]: definition of an unknown language item: `i128_addo`
2019-07-19T14:36:50.2026005Z    |
2019-07-19T14:36:50.2026005Z    |
2019-07-19T14:36:50.2026337Z 98 | |             $($body:tt)*
2019-07-19T14:36:50.2026688Z    | |________________________^ definition of unknown language item `i128_addo`
2019-07-19T14:36:50.2027985Z ...
2019-07-19T14:36:50.2028268Z 291|           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2028593Z 
2019-07-19T14:36:50.2028593Z 
2019-07-19T14:36:50.2028854Z error[E0522]: definition of an unknown language item: `u128_add`
2019-07-19T14:36:50.2029319Z     |
2019-07-19T14:36:50.2029598Z 104 | |         intrinsics! {
2019-07-19T14:36:50.2029598Z 104 | |         intrinsics! {
2019-07-19T14:36:50.2030029Z     | |_______________________^ definition of unknown language item `u128_add`
2019-07-19T14:36:50.2030273Z ...
2019-07-19T14:36:50.2030560Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2030898Z 
2019-07-19T14:36:50.2030898Z 
2019-07-19T14:36:50.2031136Z error[E0522]: definition of an unknown language item: `u128_addo`
2019-07-19T14:36:50.2031621Z     |
2019-07-19T14:36:50.2031875Z 108 | |             }
2019-07-19T14:36:50.2031875Z 108 | |             }
2019-07-19T14:36:50.2032177Z     | |________________________^ definition of unknown language item `u128_addo`
2019-07-19T14:36:50.2032379Z ...
2019-07-19T14:36:50.2032662Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2032995Z 
2019-07-19T14:36:50.2032995Z 
2019-07-19T14:36:50.2033884Z error[E0522]: definition of an unknown language item: `i128_sub`
2019-07-19T14:36:50.2034472Z     |
2019-07-19T14:36:50.2034472Z     |
2019-07-19T14:36:50.2034785Z 115 | |                 $($body)*
2019-07-19T14:36:50.2035128Z     | |_______________________^ definition of unknown language item `i128_sub`
2019-07-19T14:36:50.2035488Z ...
2019-07-19T14:36:50.2035814Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2036183Z 
2019-07-19T14:36:50.2036183Z 
2019-07-19T14:36:50.2036452Z error[E0522]: definition of an unknown language item: `i128_subo`
2019-07-19T14:36:50.2037678Z     |
2019-07-19T14:36:50.2037678Z     |
2019-07-19T14:36:50.2037988Z 119 | |         intrinsics!($($rest)*);
2019-07-19T14:36:50.2038327Z     | |________________________^ definition of unknown language item `i128_subo`
2019-07-19T14:36:50.2038563Z ...
2019-07-19T14:36:50.2038885Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2039252Z 
2019-07-19T14:36:50.2039252Z 
2019-07-19T14:36:50.2039519Z error[E0522]: definition of an unknown language item: `u128_sub`
2019-07-19T14:36:50.2040179Z     |
2019-07-19T14:36:50.2040481Z 125 | |         #[unadjusted_on_win64]
2019-07-19T14:36:50.2040481Z 125 | |         #[unadjusted_on_win64]
2019-07-19T14:36:50.2040832Z     | |_______________________^ definition of unknown language item `u128_sub`
2019-07-19T14:36:50.2041217Z ...
2019-07-19T14:36:50.2041850Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2042172Z 
2019-07-19T14:36:50.2042172Z 
2019-07-19T14:36:50.2042415Z error[E0522]: definition of an unknown language item: `u128_subo`
2019-07-19T14:36:50.2043259Z     |
2019-07-19T14:36:50.2043850Z 129 | |         }
2019-07-19T14:36:50.2043850Z 129 | |         }
2019-07-19T14:36:50.2044191Z     | |________________________^ definition of unknown language item `u128_subo`
2019-07-19T14:36:50.2044421Z ...
2019-07-19T14:36:50.2044740Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2045114Z 
2019-07-19T14:36:50.2045114Z 
2019-07-19T14:36:50.2045388Z error[E0522]: definition of an unknown language item: `i128_mul`
2019-07-19T14:36:50.2046064Z     |
2019-07-19T14:36:50.2046064Z     |
2019-07-19T14:36:50.2046366Z 113 | |             $(#[$($attr)*])*
2019-07-19T14:36:50.2046703Z     | |_______________________^ definition of unknown language item `i128_mul`
2019-07-19T14:36:50.2047122Z ...
2019-07-19T14:36:50.2047401Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2047736Z 
2019-07-19T14:36:50.2047736Z 
2019-07-19T14:36:50.2047974Z error[E0522]: definition of an unknown language item: `i128_mulo`
2019-07-19T14:36:50.2048473Z     |
2019-07-19T14:36:50.2048716Z 117 | |         }
2019-07-19T14:36:50.2048716Z 117 | |         }
2019-07-19T14:36:50.2049010Z     | |________________________^ definition of unknown language item `i128_mulo`
2019-07-19T14:36:50.2049212Z ...
2019-07-19T14:36:50.2049552Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2049904Z 
2019-07-19T14:36:50.2049904Z 
2019-07-19T14:36:50.2050141Z error[E0522]: definition of an unknown language item: `u128_mul`
2019-07-19T14:36:50.2050635Z     |
2019-07-19T14:36:50.2050635Z     |
2019-07-19T14:36:50.2050895Z 123 | |     // win64 for some methods.
2019-07-19T14:36:50.2051204Z     | |_______________________^ definition of unknown language item `u128_mul`
2019-07-19T14:36:50.2051388Z ...
2019-07-19T14:36:50.2051673Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2052004Z 
2019-07-19T14:36:50.2052004Z 
2019-07-19T14:36:50.2052240Z error[E0522]: definition of an unknown language item: `u128_mulo`
2019-07-19T14:36:50.2052727Z     |
2019-07-19T14:36:50.2052727Z     |
2019-07-19T14:36:50.2053044Z 127 | |         pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
2019-07-19T14:36:50.2053780Z     | |________________________^ definition of unknown language item `u128_mulo`
2019-07-19T14:36:50.2054006Z ...
2019-07-19T14:36:50.2054327Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2054829Z 
2019-07-19T14:36:50.2054829Z 
2019-07-19T14:36:50.2055100Z error[E0522]: definition of an unknown language item: `i128_div`
2019-07-19T14:36:50.2055658Z     |
2019-07-19T14:36:50.2055950Z 104 | |         intrinsics! {
2019-07-19T14:36:50.2055950Z 104 | |         intrinsics! {
2019-07-19T14:36:50.2056315Z     | |_______________________^ definition of unknown language item `i128_div`
2019-07-19T14:36:50.2056528Z ...
2019-07-19T14:36:50.2057005Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2057500Z 
2019-07-19T14:36:50.2057500Z 
2019-07-19T14:36:50.2057725Z error[E0522]: definition of an unknown language item: `i128_rem`
2019-07-19T14:36:50.2058194Z     |
2019-07-19T14:36:50.2058496Z 108 | |             }
2019-07-19T14:36:50.2058496Z 108 | |             }
2019-07-19T14:36:50.2070269Z     | |_______________________^ definition of unknown language item `i128_rem`
2019-07-19T14:36:50.2070527Z ...
2019-07-19T14:36:50.2071025Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2071528Z 
2019-07-19T14:36:50.2071528Z 
2019-07-19T14:36:50.2071763Z error[E0522]: definition of an unknown language item: `i128_shl`
2019-07-19T14:36:50.2072260Z     |
2019-07-19T14:36:50.2072508Z 109 | |         }
2019-07-19T14:36:50.2072508Z 109 | |         }
2019-07-19T14:36:50.2072981Z     | |_______________________^ definition of unknown language item `i128_shl`
2019-07-19T14:36:50.2073626Z ...
2019-07-19T14:36:50.2073959Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2074334Z 
2019-07-19T14:36:50.2074334Z 
2019-07-19T14:36:50.2074601Z error[E0522]: definition of an unknown language item: `i128_shlo`
2019-07-19T14:36:50.2075171Z     |
2019-07-19T14:36:50.2075171Z     |
2019-07-19T14:36:50.2075472Z 113 | |             $(#[$($attr)*])*
2019-07-19T14:36:50.2076064Z     | |________________________^ definition of unknown language item `i128_shlo`
2019-07-19T14:36:50.2076278Z ...
2019-07-19T14:36:50.2078194Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2078586Z 
2019-07-19T14:36:50.2078586Z 
2019-07-19T14:36:50.2078812Z error[E0522]: definition of an unknown language item: `u128_shl`
2019-07-19T14:36:50.2079465Z     |
2019-07-19T14:36:50.2079746Z 117 | |         }
2019-07-19T14:36:50.2079746Z 117 | |         }
2019-07-19T14:36:50.2080064Z     | |_______________________^ definition of unknown language item `u128_shl`
2019-07-19T14:36:50.2080239Z ...
2019-07-19T14:36:50.2080512Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2080831Z 
2019-07-19T14:36:50.2080831Z 
2019-07-19T14:36:50.2081196Z error[E0522]: definition of an unknown language item: `u128_shlo`
2019-07-19T14:36:50.2081718Z     |
2019-07-19T14:36:50.2081946Z 121 | |
2019-07-19T14:36:50.2081946Z 121 | |
2019-07-19T14:36:50.2082252Z     | |________________________^ definition of unknown language item `u128_shlo`
2019-07-19T14:36:50.2082429Z ...
2019-07-19T14:36:50.2082699Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2083030Z 
2019-07-19T14:36:50.2083030Z 
2019-07-19T14:36:50.2083258Z error[E0522]: definition of an unknown language item: `i128_shr`
2019-07-19T14:36:50.2084244Z     |
2019-07-19T14:36:50.2084244Z     |
2019-07-19T14:36:50.2084550Z 126 | |         $(#[$($attr:tt)*])*
2019-07-19T14:36:50.2084917Z     | |_______________________^ definition of unknown language item `i128_shr`
2019-07-19T14:36:50.2085125Z ...
2019-07-19T14:36:50.2085462Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2085834Z 
2019-07-19T14:36:50.2085834Z 
2019-07-19T14:36:50.2086106Z error[E0522]: definition of an unknown language item: `i128_shro`
2019-07-19T14:36:50.2086806Z     |
2019-07-19T14:36:50.2087300Z 130 | |
2019-07-19T14:36:50.2087300Z 130 | |
2019-07-19T14:36:50.2088087Z     | |________________________^ definition of unknown language item `i128_shro`
2019-07-19T14:36:50.2088444Z ...
2019-07-19T14:36:50.2088768Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2089102Z 
2019-07-19T14:36:50.2089102Z 
2019-07-19T14:36:50.2089351Z error[E0522]: definition of an unknown language item: `u128_shr`
2019-07-19T14:36:50.2089879Z     |
2019-07-19T14:36:50.2090472Z 134 | |         intrinsics! {
2019-07-19T14:36:50.2090472Z 134 | |         intrinsics! {
2019-07-19T14:36:50.2090946Z     | |_______________________^ definition of unknown language item `u128_shr`
2019-07-19T14:36:50.2091123Z ...
2019-07-19T14:36:50.2091499Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2091844Z 
2019-07-19T14:36:50.2091844Z 
2019-07-19T14:36:50.2092158Z error[E0522]: definition of an unknown language item: `u128_shro`
2019-07-19T14:36:50.2092613Z     |
2019-07-19T14:36:50.2092869Z 138 | |             }
2019-07-19T14:36:50.2092869Z 138 | |             }
2019-07-19T14:36:50.2093155Z     | |________________________^ definition of unknown language item `u128_shro`
2019-07-19T14:36:50.2093335Z ...
2019-07-19T14:36:50.2094205Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2094576Z 
2019-07-19T14:36:50.2094576Z 
2019-07-19T14:36:50.2094863Z error[E0522]: definition of an unknown language item: `u128_div`
2019-07-19T14:36:50.2095399Z     |
2019-07-19T14:36:50.2095702Z 273 | |         }
2019-07-19T14:36:50.2095702Z 273 | |         }
2019-07-19T14:36:50.2096046Z     | |_______________________^ definition of unknown language item `u128_div`
2019-07-19T14:36:50.2096255Z ...
2019-07-19T14:36:50.2096594Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2097223Z 
2019-07-19T14:36:50.2097223Z 
2019-07-19T14:36:50.2097464Z error[E0522]: definition of an unknown language item: `u128_rem`
2019-07-19T14:36:50.2097901Z     |
2019-07-19T14:36:50.2097901Z     |
2019-07-19T14:36:50.2098170Z 277 | |         fn from(i: u128) -> U64x2 {
2019-07-19T14:36:50.2098455Z     | |_______________________^ definition of unknown language item `u128_rem`
2019-07-19T14:36:50.2098624Z ...
2019-07-19T14:36:50.2099111Z 291 |           #[cfg_attr(not(any(stage0, feature = "no-lang-items")), lang = $lang)]
2019-07-19T14:36:50.2102961Z 
2019-07-19T14:36:51.0142387Z error: aborting due to 24 previous errors
2019-07-19T14:36:51.0142487Z 
2019-07-19T14:36:51.0142763Z For more information about this error, try `rustc --explain E0522`.
2019-07-19T14:36:51.0142763Z For more information about this error, try `rustc --explain E0522`.
2019-07-19T14:36:51.0216630Z error: Could not compile `compiler_builtins`.
2019-07-19T14:36:51.0217351Z warning: build failed, waiting for other jobs to finish...
2019-07-19T14:36:51.6614015Z error: build failed
2019-07-19T14:36:51.6633144Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-19T14:36:51.6633818Z expected success, got: exit code: 101
2019-07-19T14:36:51.6640551Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-19T14:36:51.6640829Z Build completed unsuccessfully in 0:23:58
2019-07-19T14:36:54.4394124Z ##[error]Bash exited with code '1'.
2019-07-19T14:36:54.4461615Z ##[section]Starting: Checkout
2019-07-19T14:36:54.4463854Z ==============================================================================
2019-07-19T14:36:54.4463921Z Task         : Get sources
2019-07-19T14:36:54.4463972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
