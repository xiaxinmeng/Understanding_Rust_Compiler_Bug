plain
2019-09-08T08:31:56.8305981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T08:31:56.8515471Z ##[command]git config gc.auto 0
2019-09-08T08:31:56.8581538Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T08:31:56.8645308Z ##[command]git config --get-all http.proxy
2019-09-08T08:31:56.8787232Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64279/merge:refs/remotes/pull/64279/merge
---
2019-09-08T10:30:53.0877524Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-08T10:30:57.9360959Z     Finished release [optimized] target(s) in 9m 10s
2019-09-08T10:32:38.4812984Z Error: there are broken links
2019-09-08T10:32:38.4815030Z  Caused By: There was an error while fetching "https://public.etherpad-mozilla.org/p/rust-compiler-meeting", https://public.etherpad-mozilla.org/p/rust-compiler-meeting: error trying to connect: Connection refused (os error 111)
2019-09-08T10:32:38.4815779Z  Caused By: "https://cs.au.dk/~amoeller/spa/" returned 503 Service Unavailable
2019-09-08T10:32:38.4831656Z 
2019-09-08T10:32:38.4832421Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-08T10:32:38.4832681Z expected success, got: exit code: 101
2019-09-08T10:32:38.4832810Z 
---
2019-09-08T10:49:32.6480278Z    Compiling rls-analysis v0.18.0
2019-09-08T10:50:37.7037831Z    Compiling crossbeam-epoch v0.7.2
2019-09-08T10:50:43.2051519Z    Compiling tempfile v3.0.5
2019-09-08T10:50:46.5692934Z    Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
2019-09-08T10:50:46.7454627Z warning: use of deprecated item 'rustc_plugin': import this through `rustc_driver::plugin` instead to make TLS work correctly. See ***/issues/62717
2019-09-08T10:50:46.7457913Z   |
2019-09-08T10:50:46.7458767Z 7 | extern crate rustc_plugin;
2019-09-08T10:50:46.7459356Z   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-08T10:50:46.7460069Z   |
---
2019-09-08T10:52:25.6331971Z    |
2019-09-08T10:52:25.6332381Z 21 | #![feature(rustc_diagnostic_macros)]
2019-09-08T10:52:25.6332883Z    |            ^^^^^^^^^^^^^^^^^^^^^^^ feature has been removed
2019-09-08T10:52:25.6333051Z 
2019-09-08T10:52:26.2643003Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:26.2644396Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:26.2644753Z     |
2019-09-08T10:52:26.2645092Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:26.2645469Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:26.2645931Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:26.2646648Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:26.2646978Z 5   |  | }
2019-09-08T10:52:26.2647474Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:26.2647844Z ...
2019-09-08T10:52:26.2648292Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:26.2648559Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:26.2649026Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:26.2649262Z 184 | |      );
2019-09-08T10:52:26.2649659Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:26.2649997Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:26.2650591Z 187 | |      )
2019-09-08T10:52:26.2650821Z 188 | |  }
2019-09-08T10:52:26.2650821Z 188 | |  }
2019-09-08T10:52:26.2651429Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:26.2651641Z     | 
2019-09-08T10:52:26.2651921Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:26.2652115Z     |
2019-09-08T10:52:26.2652395Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:26.2652630Z 5   | |
2019-09-08T10:52:26.2652902Z 6   | |  E0178: r##"
2019-09-08T10:52:26.2653234Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:26.2654395Z 422 | |
2019-09-08T10:52:26.2654714Z 423 | |  }
2019-09-08T10:52:26.2655014Z     | |__- in this macro invocation
2019-09-08T10:52:26.2655056Z 
2019-09-08T10:52:26.2655056Z 
2019-09-08T10:52:27.2355556Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2390755Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2391643Z     |
2019-09-08T10:52:27.2392144Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2393096Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2394035Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2394641Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2395151Z 5   |  | }
2019-09-08T10:52:27.2395652Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2396052Z ...
2019-09-08T10:52:27.2396527Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2397221Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2397989Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2398454Z 184 | |      );
2019-09-08T10:52:27.2399120Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2399776Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2400786Z 187 | |      )
2019-09-08T10:52:27.2401233Z 188 | |  }
2019-09-08T10:52:27.2401233Z 188 | |  }
2019-09-08T10:52:27.2401696Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2402104Z     | 
2019-09-08T10:52:27.2402564Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2402925Z     |
2019-09-08T10:52:27.2403391Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2404253Z 5   | |
2019-09-08T10:52:27.2404807Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2405338Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2406406Z 422 | |
2019-09-08T10:52:27.2406850Z 423 | |  }
2019-09-08T10:52:27.2407329Z     | |__- in this macro invocation
2019-09-08T10:52:27.2407655Z 
2019-09-08T10:52:27.2407655Z 
2019-09-08T10:52:27.2408319Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2409015Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2412963Z     |
2019-09-08T10:52:27.2413293Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2413906Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2414445Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2414964Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2415307Z 5   |  | }
2019-09-08T10:52:27.2415662Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2415876Z ...
2019-09-08T10:52:27.2416201Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2416541Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2416884Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2417183Z 184 | |      );
2019-09-08T10:52:27.2417589Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2417930Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2418645Z 187 | |      )
2019-09-08T10:52:27.2418923Z 188 | |  }
2019-09-08T10:52:27.2418923Z 188 | |  }
2019-09-08T10:52:27.2419267Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2419493Z     | 
2019-09-08T10:52:27.2419808Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2420049Z     |
2019-09-08T10:52:27.2420351Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2420732Z 5   | |
2019-09-08T10:52:27.2421045Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2421395Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2421936Z 422 | |
2019-09-08T10:52:27.2422212Z 423 | |  }
2019-09-08T10:52:27.2422525Z     | |__- in this macro invocation
2019-09-08T10:52:27.2422575Z 
2019-09-08T10:52:27.2422575Z 
2019-09-08T10:52:27.2422845Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2423169Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2423582Z     |
2019-09-08T10:52:27.2424760Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2425175Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2425633Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2425990Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2426305Z 5   |  | }
2019-09-08T10:52:27.2426634Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2426851Z ...
2019-09-08T10:52:27.2427186Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2427504Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2428014Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2428309Z 184 | |      );
2019-09-08T10:52:27.2428616Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2428973Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2429951Z 187 | |      )
2019-09-08T10:52:27.2430238Z 188 | |  }
2019-09-08T10:52:27.2430238Z 188 | |  }
2019-09-08T10:52:27.2430695Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2430929Z     | 
2019-09-08T10:52:27.2431228Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2431446Z     |
2019-09-08T10:52:27.2431759Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2432014Z 5   | |
2019-09-08T10:52:27.2432294Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2432651Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2433148Z 422 | |
2019-09-08T10:52:27.2433436Z 423 | |  }
2019-09-08T10:52:27.2434330Z     | |__- in this macro invocation
2019-09-08T10:52:27.2434380Z 
2019-09-08T10:52:27.2434380Z 
2019-09-08T10:52:27.2434731Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2435062Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2435281Z     |
2019-09-08T10:52:27.2435626Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2435991Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2436449Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2436801Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2437090Z 5   |  | }
2019-09-08T10:52:27.2437618Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2437831Z ...
2019-09-08T10:52:27.2438130Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2438457Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2438791Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2439090Z 184 | |      );
2019-09-08T10:52:27.2439397Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2439836Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2440517Z 187 | |      )
2019-09-08T10:52:27.2440788Z 188 | |  }
2019-09-08T10:52:27.2440788Z 188 | |  }
2019-09-08T10:52:27.2441116Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2441328Z     | 
2019-09-08T10:52:27.2441646Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2441871Z     |
2019-09-08T10:52:27.2442159Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2442431Z 5   | |
2019-09-08T10:52:27.2442783Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2443152Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2444218Z 422 | |
2019-09-08T10:52:27.2444504Z 423 | |  }
2019-09-08T10:52:27.2444835Z     | |__- in this macro invocation
2019-09-08T10:52:27.2444873Z 
2019-09-08T10:52:27.2444873Z 
2019-09-08T10:52:27.2445457Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2445938Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2446171Z     |
2019-09-08T10:52:27.2446485Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2446876Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2447325Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2447690Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2447991Z 5   |  | }
2019-09-08T10:52:27.2448477Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2448878Z ...
2019-09-08T10:52:27.2449185Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2449500Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2450185Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2450466Z 184 | |      );
2019-09-08T10:52:27.2450802Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2451135Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2451825Z 187 | |      )
2019-09-08T10:52:27.2452091Z 188 | |  }
2019-09-08T10:52:27.2452091Z 188 | |  }
2019-09-08T10:52:27.2452397Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2452631Z     | 
2019-09-08T10:52:27.2453007Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2453247Z     |
2019-09-08T10:52:27.2453559Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2454234Z 5   | |
2019-09-08T10:52:27.2454572Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2454926Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2455460Z 422 | |
2019-09-08T10:52:27.2455745Z 423 | |  }
2019-09-08T10:52:27.2456039Z     | |__- in this macro invocation
2019-09-08T10:52:27.2456099Z 
2019-09-08T10:52:27.2456099Z 
2019-09-08T10:52:27.2456363Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2456689Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2456939Z     |
2019-09-08T10:52:27.2457254Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2457779Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2458217Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2458552Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2458995Z 5   |  | }
2019-09-08T10:52:27.2459314Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2459516Z ...
2019-09-08T10:52:27.2459842Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2460156Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2460484Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2460958Z 184 | |      );
2019-09-08T10:52:27.2461287Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2461649Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2462454Z 187 | |      )
2019-09-08T10:52:27.2462756Z 188 | |  }
2019-09-08T10:52:27.2462756Z 188 | |  }
2019-09-08T10:52:27.2463076Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2463479Z     | 
2019-09-08T10:52:27.2463893Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2464112Z     |
2019-09-08T10:52:27.2464412Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2464713Z 5   | |
2019-09-08T10:52:27.2465006Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2465375Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2465894Z 422 | |
2019-09-08T10:52:27.2466192Z 423 | |  }
2019-09-08T10:52:27.2466489Z     | |__- in this macro invocation
2019-09-08T10:52:27.2466529Z 
2019-09-08T10:52:27.2466529Z 
2019-09-08T10:52:27.2466798Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2467343Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2467553Z     |
2019-09-08T10:52:27.2467869Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2468365Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2468781Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2469145Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2469427Z 5   |  | }
2019-09-08T10:52:27.2560553Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2560872Z ...
2019-09-08T10:52:27.2561223Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2561713Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2562248Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2562597Z 184 | |      );
2019-09-08T10:52:27.2562935Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2563266Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2564600Z 187 | |      )
2019-09-08T10:52:27.2564882Z 188 | |  }
2019-09-08T10:52:27.2564882Z 188 | |  }
2019-09-08T10:52:27.2565232Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2565458Z     | 
2019-09-08T10:52:27.2565771Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2566010Z     |
2019-09-08T10:52:27.2566308Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2566589Z 5   | |
2019-09-08T10:52:27.2567065Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2567450Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2567977Z 422 | |
2019-09-08T10:52:27.2568245Z 423 | |  }
2019-09-08T10:52:27.2568548Z     | |__- in this macro invocation
2019-09-08T10:52:27.2568734Z 
2019-09-08T10:52:27.2568734Z 
2019-09-08T10:52:27.2569023Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2569341Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2569576Z     |
2019-09-08T10:52:27.2569886Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2570265Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2570684Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2571029Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2571336Z 5   |  | }
2019-09-08T10:52:27.2571733Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2571970Z ...
2019-09-08T10:52:27.2572291Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2572598Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2572947Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2573242Z 184 | |      );
2019-09-08T10:52:27.2573546Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2574310Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2575026Z 187 | |      )
2019-09-08T10:52:27.2575321Z 188 | |  }
2019-09-08T10:52:27.2575321Z 188 | |  }
2019-09-08T10:52:27.2575641Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2575877Z     | 
2019-09-08T10:52:27.2576190Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2576416Z     |
2019-09-08T10:52:27.2576738Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2577004Z 5   | |
2019-09-08T10:52:27.2577454Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2577965Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2578452Z 422 | |
2019-09-08T10:52:27.2578742Z 423 | |  }
2019-09-08T10:52:27.2579035Z     | |__- in this macro invocation
2019-09-08T10:52:27.2579074Z 
2019-09-08T10:52:27.2579074Z 
2019-09-08T10:52:27.2579350Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2579662Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2579871Z     |
2019-09-08T10:52:27.2580206Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2580560Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2581076Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2581441Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2581722Z 5   |  | }
2019-09-08T10:52:27.2582073Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2582430Z ...
2019-09-08T10:52:27.2582718Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2583045Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2583364Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2583820Z 184 | |      );
2019-09-08T10:52:27.2584614Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2585008Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2585719Z 187 | |      )
2019-09-08T10:52:27.2586022Z 188 | |  }
2019-09-08T10:52:27.2586022Z 188 | |  }
2019-09-08T10:52:27.2586343Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2586562Z     | 
2019-09-08T10:52:27.2586893Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2587263Z     |
2019-09-08T10:52:27.2587707Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2587971Z 5   | |
2019-09-08T10:52:27.2588253Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2588584Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2589072Z 422 | |
2019-09-08T10:52:27.2589341Z 423 | |  }
2019-09-08T10:52:27.2590061Z     | |__- in this macro invocation
2019-09-08T10:52:27.2590104Z 
2019-09-08T10:52:27.2590104Z 
2019-09-08T10:52:27.2590356Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2590793Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2591039Z     |
2019-09-08T10:52:27.2591330Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2591690Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2592105Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2592445Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2592724Z 5   |  | }
2019-09-08T10:52:27.2593032Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2593247Z ...
2019-09-08T10:52:27.2593534Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2594421Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2594799Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2595089Z 184 | |      );
2019-09-08T10:52:27.2595439Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2595785Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2596647Z 187 | |      )
2019-09-08T10:52:27.2596927Z 188 | |  }
2019-09-08T10:52:27.2596927Z 188 | |  }
2019-09-08T10:52:27.2597402Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2597638Z     | 
2019-09-08T10:52:27.2597932Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2598151Z     |
2019-09-08T10:52:27.2598432Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2598678Z 5   | |
2019-09-08T10:52:27.2598983Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2599312Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2599872Z 422 | |
2019-09-08T10:52:27.2600161Z 423 | |  }
2019-09-08T10:52:27.2600435Z     | |__- in this macro invocation
2019-09-08T10:52:27.2600491Z 
2019-09-08T10:52:27.2600491Z 
2019-09-08T10:52:27.2600736Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2601052Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2601276Z     |
2019-09-08T10:52:27.2601567Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2601915Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2602340Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2602660Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2602961Z 5   |  | }
2019-09-08T10:52:27.2603271Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2603465Z ...
2019-09-08T10:52:27.2604184Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2604517Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2604857Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2605167Z 184 | |      );
2019-09-08T10:52:27.2605651Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2606009Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2606707Z 187 | |      )
2019-09-08T10:52:27.2607007Z 188 | |  }
2019-09-08T10:52:27.2607007Z 188 | |  }
2019-09-08T10:52:27.2607478Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2607688Z     | 
2019-09-08T10:52:27.2607995Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2608191Z     |
2019-09-08T10:52:27.2608529Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2608821Z 5   | |
2019-09-08T10:52:27.2609083Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2609396Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2609881Z 422 | |
2019-09-08T10:52:27.2610130Z 423 | |  }
2019-09-08T10:52:27.2610414Z     | |__- in this macro invocation
2019-09-08T10:52:27.2610449Z 
2019-09-08T10:52:27.2610449Z 
2019-09-08T10:52:27.2610691Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2611011Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2611209Z     |
2019-09-08T10:52:27.2611488Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2611845Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2612231Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2612567Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2612830Z 5   |  | }
2019-09-08T10:52:27.2613125Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2613610Z ...
2019-09-08T10:52:27.2614298Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2614625Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2615001Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2615292Z 184 | |      );
2019-09-08T10:52:27.2615624Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2615965Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2616676Z 187 | |      )
2019-09-08T10:52:27.2617058Z 188 | |  }
2019-09-08T10:52:27.2617058Z 188 | |  }
2019-09-08T10:52:27.2617429Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2617653Z     | 
2019-09-08T10:52:27.2617964Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2618195Z     |
2019-09-08T10:52:27.2618513Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2618978Z 5   | |
2019-09-08T10:52:27.2619271Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2619620Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2620125Z 422 | |
2019-09-08T10:52:27.2620391Z 423 | |  }
2019-09-08T10:52:27.2620675Z     | |__- in this macro invocation
2019-09-08T10:52:27.2620952Z 
2019-09-08T10:52:27.2620952Z 
2019-09-08T10:52:27.2621388Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2621697Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2621925Z     |
2019-09-08T10:52:27.2622232Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2622587Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2623019Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2623693Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2624222Z 5   |  | }
2019-09-08T10:52:27.2624570Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2624783Z ...
2019-09-08T10:52:27.2625110Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2625427Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2625782Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2626090Z 184 | |      );
2019-09-08T10:52:27.2626410Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2626871Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2627932Z 187 | |      )
2019-09-08T10:52:27.2628223Z 188 | |  }
2019-09-08T10:52:27.2628223Z 188 | |  }
2019-09-08T10:52:27.2628521Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2628741Z     | 
2019-09-08T10:52:27.2629040Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2629244Z     |
2019-09-08T10:52:27.2710043Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2710634Z 5   | |
2019-09-08T10:52:27.2711093Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2711493Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2711983Z 422 | |
2019-09-08T10:52:27.2712434Z 423 | |  }
2019-09-08T10:52:27.2712711Z     | |__- in this macro invocation
2019-09-08T10:52:27.2712751Z 
2019-09-08T10:52:27.2712751Z 
2019-09-08T10:52:27.2713014Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2713318Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2714231Z     |
2019-09-08T10:52:27.2714576Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2714948Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2715394Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2715768Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2716061Z 5   |  | }
2019-09-08T10:52:27.2716420Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2716632Z ...
2019-09-08T10:52:27.2716944Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2717733Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2718098Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2718370Z 184 | |      );
2019-09-08T10:52:27.2718687Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2719016Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2719675Z 187 | |      )
2019-09-08T10:52:27.2719936Z 188 | |  }
2019-09-08T10:52:27.2719936Z 188 | |  }
2019-09-08T10:52:27.2720251Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:52:27.2720461Z     | 
2019-09-08T10:52:27.2720764Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:52:27.2720987Z     |
2019-09-08T10:52:27.2721265Z 4   | /  register_long_diagnostics! {
2019-09-08T10:52:27.2721512Z 5   | |
2019-09-08T10:52:27.2721809Z 6   | |  E0178: r##"
2019-09-08T10:52:27.2722140Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:52:27.2722628Z 422 | |
2019-09-08T10:52:27.2722995Z 423 | |  }
2019-09-08T10:52:27.2723289Z     | |__- in this macro invocation
2019-09-08T10:52:27.2723327Z 
2019-09-08T10:52:27.2723327Z 
2019-09-08T10:52:27.2723573Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:52:27.2724336Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:52:27.2724571Z     |
2019-09-08T10:52:27.2724883Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:52:27.2725267Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:52:27.2725710Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:52:27.2726055Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:52:27.2726473Z 5   |  | }
2019-09-08T10:52:27.2726839Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:52:27.2727070Z ...
2019-09-08T10:52:27.2727536Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:52:27.2727836Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:52:27.2728163Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:52:27.2728430Z 184 | |      );
2019-09-08T10:52:27.2728718Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:52:27.2729051Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
---
2019-09-08T10:53:01.1232226Z    |
2019-09-08T10:53:01.1232823Z 21 | #![feature(rustc_diagnostic_macros)]
2019-09-08T10:53:01.1233339Z    |            ^^^^^^^^^^^^^^^^^^^^^^^ feature has been removed
2019-09-08T10:53:01.1233536Z 
2019-09-08T10:53:01.7602814Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.7604348Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.7605107Z     |
2019-09-08T10:53:01.7605845Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.7606365Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.7606925Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.7607419Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.7607882Z 5   |  | }
2019-09-08T10:53:01.7608321Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.7608698Z ...
2019-09-08T10:53:01.7609123Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.7609572Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.7610032Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.7610442Z 184 | |      );
2019-09-08T10:53:01.7611218Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.7611820Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.7613813Z 187 | |      )
2019-09-08T10:53:01.7614289Z 188 | |  }
2019-09-08T10:53:01.7614289Z 188 | |  }
2019-09-08T10:53:01.7614797Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.7615195Z     | 
2019-09-08T10:53:01.7615834Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.7616330Z     |
2019-09-08T10:53:01.7616767Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.7617316Z 5   | |
2019-09-08T10:53:01.7617724Z 6   | |  E0178: r##"
2019-09-08T10:53:01.7618463Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.7619468Z 422 | |
2019-09-08T10:53:01.7619865Z 423 | |  }
2019-09-08T10:53:01.7620297Z     | |__- in this macro invocation
2019-09-08T10:53:01.7620637Z 
2019-09-08T10:53:01.7620637Z 
2019-09-08T10:53:01.7944507Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.7945247Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.7946010Z     |
2019-09-08T10:53:01.7946655Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.7947878Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.7948266Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.7948618Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.7949033Z 5   |  | }
2019-09-08T10:53:01.7949319Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.7949730Z ...
2019-09-08T10:53:01.7950005Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.7950298Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.7950588Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.7951398Z 184 | |      );
2019-09-08T10:53:01.7952309Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.7952871Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.7953585Z 187 | |      )
2019-09-08T10:53:01.7953875Z 188 | |  }
2019-09-08T10:53:01.7953875Z 188 | |  }
2019-09-08T10:53:01.7954213Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.7954439Z     | 
2019-09-08T10:53:01.7954753Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.7955130Z     |
2019-09-08T10:53:01.7955476Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.7955742Z 5   | |
2019-09-08T10:53:01.7956281Z 6   | |  E0178: r##"
2019-09-08T10:53:01.7956766Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.7957227Z 422 | |
2019-09-08T10:53:01.7957469Z 423 | |  }
2019-09-08T10:53:01.7957736Z     | |__- in this macro invocation
2019-09-08T10:53:01.7957794Z 
2019-09-08T10:53:01.7957794Z 
2019-09-08T10:53:01.8163746Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.8164137Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.8164419Z     |
2019-09-08T10:53:01.8164735Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.8165126Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.8165762Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.8166069Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.8166346Z 5   |  | }
2019-09-08T10:53:01.8166854Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.8167238Z ...
2019-09-08T10:53:01.8167521Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.8167817Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.8168146Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8168409Z 184 | |      );
2019-09-08T10:53:01.8168695Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.8169366Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8170045Z 187 | |      )
2019-09-08T10:53:01.8170322Z 188 | |  }
2019-09-08T10:53:01.8170322Z 188 | |  }
2019-09-08T10:53:01.8170591Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.8170799Z     | 
2019-09-08T10:53:01.8171074Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.8171256Z     |
2019-09-08T10:53:01.8171531Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.8172888Z 5   | |
2019-09-08T10:53:01.8173301Z 6   | |  E0178: r##"
2019-09-08T10:53:01.8173689Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.8174199Z 422 | |
2019-09-08T10:53:01.8174516Z 423 | |  }
2019-09-08T10:53:01.8174813Z     | |__- in this macro invocation
2019-09-08T10:53:01.8174854Z 
2019-09-08T10:53:01.8174854Z 
2019-09-08T10:53:01.8403625Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.8404048Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.8404311Z     |
2019-09-08T10:53:01.8404632Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.8405007Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.8406026Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.8406380Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.8406701Z 5   |  | }
2019-09-08T10:53:01.8407039Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.8407249Z ...
2019-09-08T10:53:01.8407873Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.8408336Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.8408644Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8409237Z 184 | |      );
2019-09-08T10:53:01.8409896Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.8410224Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8410783Z 187 | |      )
2019-09-08T10:53:01.8411024Z 188 | |  }
2019-09-08T10:53:01.8411024Z 188 | |  }
2019-09-08T10:53:01.8411277Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.8411459Z     | 
2019-09-08T10:53:01.8412379Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.8412647Z     |
2019-09-08T10:53:01.8412947Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.8413241Z 5   | |
2019-09-08T10:53:01.8413554Z 6   | |  E0178: r##"
2019-09-08T10:53:01.8413926Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.8414445Z 422 | |
2019-09-08T10:53:01.8414742Z 423 | |  }
2019-09-08T10:53:01.8415041Z     | |__- in this macro invocation
2019-09-08T10:53:01.8415082Z 
2019-09-08T10:53:01.8415082Z 
2019-09-08T10:53:01.8645542Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.8646294Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.8646489Z     |
2019-09-08T10:53:01.8646773Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.8647095Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.8647461Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.8647777Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.8648035Z 5   |  | }
2019-09-08T10:53:01.8648330Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.8648511Z ...
2019-09-08T10:53:01.8648857Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.8649185Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.8649475Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8649730Z 184 | |      );
2019-09-08T10:53:01.8650021Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.8650307Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8650911Z 187 | |      )
2019-09-08T10:53:01.8651147Z 188 | |  }
2019-09-08T10:53:01.8651147Z 188 | |  }
2019-09-08T10:53:01.8651437Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.8651635Z     | 
2019-09-08T10:53:01.8652576Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.8652858Z     |
2019-09-08T10:53:01.8653176Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.8653464Z 5   | |
2019-09-08T10:53:01.8653762Z 6   | |  E0178: r##"
2019-09-08T10:53:01.8654116Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.8654812Z 422 | |
2019-09-08T10:53:01.8655090Z 423 | |  }
2019-09-08T10:53:01.8655578Z     | |__- in this macro invocation
2019-09-08T10:53:01.8655617Z 
2019-09-08T10:53:01.8655617Z 
2019-09-08T10:53:01.8760657Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.8760997Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.8761182Z     |
2019-09-08T10:53:01.8761432Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.8761759Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.8762473Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.8763022Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.8763377Z 5   |  | }
2019-09-08T10:53:01.8763709Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.8763959Z ...
2019-09-08T10:53:01.8764270Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.8764590Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.8764966Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8765260Z 184 | |      );
2019-09-08T10:53:01.8765901Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.8766165Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8766724Z 187 | |      )
2019-09-08T10:53:01.8766946Z 188 | |  }
2019-09-08T10:53:01.8766946Z 188 | |  }
2019-09-08T10:53:01.8767198Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.8767390Z     | 
2019-09-08T10:53:01.8767632Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.8767819Z     |
2019-09-08T10:53:01.8768175Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.8768380Z 5   | |
2019-09-08T10:53:01.8768626Z 6   | |  E0178: r##"
2019-09-08T10:53:01.8768905Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.8769317Z 422 | |
2019-09-08T10:53:01.8769531Z 423 | |  }
2019-09-08T10:53:01.8769759Z     | |__- in this macro invocation
2019-09-08T10:53:01.8769816Z 
2019-09-08T10:53:01.8769816Z 
2019-09-08T10:53:01.8984187Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.8984597Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.8984860Z     |
2019-09-08T10:53:01.8985369Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.8985974Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.8986765Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.8987233Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.8987502Z 5   |  | }
2019-09-08T10:53:01.8987787Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.8987970Z ...
2019-09-08T10:53:01.8988254Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.8988526Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.8988847Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8989095Z 184 | |      );
2019-09-08T10:53:01.8989362Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.8989991Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.8990607Z 187 | |      )
2019-09-08T10:53:01.8991020Z 188 | |  }
2019-09-08T10:53:01.8991020Z 188 | |  }
2019-09-08T10:53:01.8991291Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.8991496Z     | 
2019-09-08T10:53:01.8991772Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.8992648Z     |
2019-09-08T10:53:01.8992983Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.8993251Z 5   | |
2019-09-08T10:53:01.8993544Z 6   | |  E0178: r##"
2019-09-08T10:53:01.8993937Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.8994447Z 422 | |
2019-09-08T10:53:01.8994896Z 423 | |  }
2019-09-08T10:53:01.8995239Z     | |__- in this macro invocation
2019-09-08T10:53:01.8995281Z 
2019-09-08T10:53:01.8995281Z 
2019-09-08T10:53:01.9323384Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.9323772Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.9324028Z     |
2019-09-08T10:53:01.9324377Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.9324751Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.9325211Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.9325855Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.9326127Z 5   |  | }
2019-09-08T10:53:01.9326659Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.9326953Z ...
2019-09-08T10:53:01.9327273Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.9327592Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.9327924Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9328312Z 184 | |      );
2019-09-08T10:53:01.9328620Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.9329348Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9330016Z 187 | |      )
2019-09-08T10:53:01.9330301Z 188 | |  }
2019-09-08T10:53:01.9330301Z 188 | |  }
2019-09-08T10:53:01.9330745Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.9330944Z     | 
2019-09-08T10:53:01.9331256Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.9331456Z     |
2019-09-08T10:53:01.9332208Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.9332984Z 5   | |
2019-09-08T10:53:01.9333678Z 6   | |  E0178: r##"
2019-09-08T10:53:01.9334085Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.9334619Z 422 | |
2019-09-08T10:53:01.9334912Z 423 | |  }
2019-09-08T10:53:01.9335233Z     | |__- in this macro invocation
2019-09-08T10:53:01.9335275Z 
2019-09-08T10:53:01.9335275Z 
2019-09-08T10:53:01.9603455Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.9603908Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.9604146Z     |
2019-09-08T10:53:01.9604481Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.9604857Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.9605304Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.9605804Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.9606064Z 5   |  | }
2019-09-08T10:53:01.9606376Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.9606562Z ...
2019-09-08T10:53:01.9606833Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.9607383Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.9607681Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9607942Z 184 | |      );
2019-09-08T10:53:01.9608240Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.9608533Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9609156Z 187 | |      )
2019-09-08T10:53:01.9609399Z 188 | |  }
2019-09-08T10:53:01.9609399Z 188 | |  }
2019-09-08T10:53:01.9609790Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.9610018Z     | 
2019-09-08T10:53:01.9610289Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.9610500Z     |
2019-09-08T10:53:01.9610762Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.9611006Z 5   | |
2019-09-08T10:53:01.9611282Z 6   | |  E0178: r##"
2019-09-08T10:53:01.9611760Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.9613331Z 422 | |
2019-09-08T10:53:01.9613671Z 423 | |  }
2019-09-08T10:53:01.9613990Z     | |__- in this macro invocation
2019-09-08T10:53:01.9614033Z 
2019-09-08T10:53:01.9614033Z 
2019-09-08T10:53:01.9924109Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:01.9924549Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:01.9924823Z     |
2019-09-08T10:53:01.9926494Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:01.9926822Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:01.9927207Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:01.9927493Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:01.9927969Z 5   |  | }
2019-09-08T10:53:01.9928284Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:01.9928492Z ...
2019-09-08T10:53:01.9928752Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:01.9929016Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:01.9929315Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9929562Z 184 | |      );
2019-09-08T10:53:01.9929840Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:01.9930121Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:01.9931365Z 187 | |      )
2019-09-08T10:53:01.9931784Z 188 | |  }
2019-09-08T10:53:01.9931784Z 188 | |  }
2019-09-08T10:53:01.9933178Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:01.9933449Z     | 
2019-09-08T10:53:01.9933761Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:01.9933976Z     |
2019-09-08T10:53:01.9934311Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:01.9934581Z 5   | |
2019-09-08T10:53:01.9934892Z 6   | |  E0178: r##"
2019-09-08T10:53:01.9935245Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:01.9935920Z 422 | |
2019-09-08T10:53:01.9936466Z 423 | |  }
2019-09-08T10:53:01.9936699Z     | |__- in this macro invocation
2019-09-08T10:53:01.9936757Z 
2019-09-08T10:53:01.9936757Z 
2019-09-08T10:53:02.0366556Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.0367148Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.0367406Z     |
2019-09-08T10:53:02.0367709Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.0368334Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.0368789Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.0369318Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.0369940Z 5   |  | }
2019-09-08T10:53:02.0370384Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.0370576Z ...
2019-09-08T10:53:02.0371024Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.0371305Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.0371912Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0372764Z 184 | |      );
2019-09-08T10:53:02.0373090Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.0373452Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0374149Z 187 | |      )
2019-09-08T10:53:02.0374457Z 188 | |  }
2019-09-08T10:53:02.0374457Z 188 | |  }
2019-09-08T10:53:02.0374781Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:02.0375021Z     | 
2019-09-08T10:53:02.0375336Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:02.0375567Z     |
2019-09-08T10:53:02.0376351Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:02.0376763Z 5   | |
2019-09-08T10:53:02.0377035Z 6   | |  E0178: r##"
2019-09-08T10:53:02.0377387Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:02.0378202Z 422 | |
2019-09-08T10:53:02.0378502Z 423 | |  }
2019-09-08T10:53:02.0378961Z     | |__- in this macro invocation
2019-09-08T10:53:02.0379003Z 
2019-09-08T10:53:02.0379003Z 
2019-09-08T10:53:02.0640791Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.0641128Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.0641326Z     |
2019-09-08T10:53:02.0641809Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.0642829Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.0643291Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.0643659Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.0643950Z 5   |  | }
2019-09-08T10:53:02.0644486Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.0644757Z ...
2019-09-08T10:53:02.0645070Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.0645416Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.0646063Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0646357Z 184 | |      );
2019-09-08T10:53:02.0646635Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.0646925Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0647523Z 187 | |      )
2019-09-08T10:53:02.0647789Z 188 | |  }
2019-09-08T10:53:02.0647789Z 188 | |  }
2019-09-08T10:53:02.0648058Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:02.0648246Z     | 
2019-09-08T10:53:02.0648531Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:02.0648721Z     |
2019-09-08T10:53:02.0648972Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:02.0649211Z 5   | |
2019-09-08T10:53:02.0649461Z 6   | |  E0178: r##"
2019-09-08T10:53:02.0649876Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:02.0650327Z 422 | |
2019-09-08T10:53:02.0650568Z 423 | |  }
2019-09-08T10:53:02.0650844Z     | |__- in this macro invocation
2019-09-08T10:53:02.0650879Z 
2019-09-08T10:53:02.0650879Z 
2019-09-08T10:53:02.0919980Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.0920295Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.0920486Z     |
2019-09-08T10:53:02.0920740Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.0921017Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.0921479Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.0922773Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.0923125Z 5   |  | }
2019-09-08T10:53:02.0923495Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.0923708Z ...
2019-09-08T10:53:02.0924015Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.0924363Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.0924713Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0925001Z 184 | |      );
2019-09-08T10:53:02.0925349Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.0925688Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.0926403Z 187 | |      )
2019-09-08T10:53:02.0926610Z 188 | |  }
2019-09-08T10:53:02.0926610Z 188 | |  }
2019-09-08T10:53:02.0926863Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:02.0927215Z     | 
2019-09-08T10:53:02.0927448Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:02.0927634Z     |
2019-09-08T10:53:02.0927859Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:02.0928064Z 5   | |
2019-09-08T10:53:02.0928305Z 6   | |  E0178: r##"
2019-09-08T10:53:02.0928566Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:02.0928967Z 422 | |
2019-09-08T10:53:02.0929185Z 423 | |  }
2019-09-08T10:53:02.0929417Z     | |__- in this macro invocation
2019-09-08T10:53:02.0929449Z 
2019-09-08T10:53:02.0929449Z 
2019-09-08T10:53:02.1160359Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.1161034Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.1161279Z     |
2019-09-08T10:53:02.1162079Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.1162824Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.1163283Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.1163659Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.1163956Z 5   |  | }
2019-09-08T10:53:02.1164284Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.1164513Z ...
2019-09-08T10:53:02.1164831Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.1165320Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.1165791Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.1166043Z 184 | |      );
2019-09-08T10:53:02.1166316Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.1166624Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.1167398Z 187 | |      )
2019-09-08T10:53:02.1167634Z 188 | |  }
2019-09-08T10:53:02.1167634Z 188 | |  }
2019-09-08T10:53:02.1167909Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:02.1168119Z     | 
2019-09-08T10:53:02.1168383Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:02.1168568Z     |
2019-09-08T10:53:02.1168841Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:02.1169074Z 5   | |
2019-09-08T10:53:02.1169320Z 6   | |  E0178: r##"
2019-09-08T10:53:02.1169640Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:02.1170192Z 422 | |
2019-09-08T10:53:02.1170427Z 423 | |  }
2019-09-08T10:53:02.1170675Z     | |__- in this macro invocation
2019-09-08T10:53:02.1170711Z 
2019-09-08T10:53:02.1170711Z 
2019-09-08T10:53:02.1443199Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.1443595Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.1443853Z     |
2019-09-08T10:53:02.1444171Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.1445190Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.1446072Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.1446541Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.1446798Z 5   |  | }
2019-09-08T10:53:02.1447069Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.1447249Z ...
2019-09-08T10:53:02.1447675Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.1447930Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.1448201Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.1448697Z 184 | |      );
2019-09-08T10:53:02.1448948Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.1449244Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.1449784Z 187 | |      )
2019-09-08T10:53:02.1450026Z 188 | |  }
2019-09-08T10:53:02.1450026Z 188 | |  }
2019-09-08T10:53:02.1450292Z     | |__- in this expansion of `register_long_diagnostics!`
2019-09-08T10:53:02.1450467Z     | 
2019-09-08T10:53:02.1450736Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/error_codes.rs:4:1
2019-09-08T10:53:02.1450909Z     |
2019-09-08T10:53:02.1451231Z 4   | /  register_long_diagnostics! {
2019-09-08T10:53:02.1451497Z 5   | |
2019-09-08T10:53:02.1451729Z 6   | |  E0178: r##"
2019-09-08T10:53:02.1452790Z 7   | |  In types, the `+` type operator has low precedence, so it is often necessary
2019-09-08T10:53:02.1453323Z 422 | |
2019-09-08T10:53:02.1453622Z 423 | |  }
2019-09-08T10:53:02.1453927Z     | |__- in this macro invocation
2019-09-08T10:53:02.1453970Z 
2019-09-08T10:53:02.1453970Z 
2019-09-08T10:53:02.1761677Z error: cannot find macro `__register_diagnostic!` in this scope
2019-09-08T10:53:02.1762606Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-581.0.0/diagnostics/macros.rs:3:37
2019-09-08T10:53:02.1762936Z     |
2019-09-08T10:53:02.1763275Z 2   |  / macro_rules! register_diagnostic {
2019-09-08T10:53:02.1763673Z 3   |  |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
2019-09-08T10:53:02.1764128Z     |  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `register_diagnostic`
2019-09-08T10:53:02.1764483Z 4   |  |     ($code:tt) => (__register_diagnostic! { $code })
2019-09-08T10:53:02.1764776Z 5   |  | }
2019-09-08T10:53:02.1765124Z     |  |_- in this expansion of `$crate::register_diagnostic!`
2019-09-08T10:53:02.1765590Z ...
2019-09-08T10:53:02.1766161Z 181 | /  macro_rules! register_long_diagnostics {
2019-09-08T10:53:02.1766426Z 182 | |      ($($code:tt: $description:tt),*) => (
2019-09-08T10:53:02.1766696Z 183 | |          $($crate::register_diagnostic! { $code, $description })*
2019-09-08T10:53:02.1766943Z 184 | |      );
2019-09-08T10:53:02.1767189Z 185 | |      ($($code:tt: $description:tt),*,) => (
2019-09-08T10:53:02.1767460Z 186 | |          $($crate::register_diagnostic! { $code, $description })*
---
2019-09-08T11:00:27.6754714Z test [compile-fail] compile-fail/slice-too-big.rs ... ok
2019-09-08T11:00:27.6833457Z 
2019-09-08T11:00:27.6845777Z error: error pattern ' tried to deallocate Stack memory but gave Machine(Rust) as the kind' not found!
2019-09-08T11:00:27.6855136Z status: exit code: 1
2019-09-08T11:00:27.6860985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stack_free.rs" "-L" "/tmp/compiletest6AitnM" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletest6AitnM/stack_free.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletest6AitnM/stack_free.stage-id.aux" "-A" "unused"
2019-09-08T11:00:27.6861351Z ------------------------------------------
2019-09-08T11:00:27.6876887Z 
2019-09-08T11:00:27.6878338Z ------------------------------------------
2019-09-08T11:00:27.6878586Z stderr:
---
2019-09-08T11:00:34.8137849Z Verifying status of edition-guide...
2019-09-08T11:00:34.8155147Z Verifying status of rls...
2019-09-08T11:00:34.8173654Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-09-08T11:00:34.8183438Z 
2019-09-08T11:00:34.8184372Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-09-08T11:00:34.8184433Z 
2019-09-08T11:00:34.8184759Z If you do intend to update 'rls', please check the error messages above and
2019-09-08T11:00:34.8184876Z commit another update.
2019-09-08T11:00:34.8184910Z 
2019-09-08T11:00:34.8185174Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-09-08T11:00:34.8185445Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-09-08T11:00:34.8185498Z proper steps.
2019-09-08T11:00:34.8208645Z   local time: Sun Sep  8 11:00:34 UTC 2019
2019-09-08T11:00:34.9810714Z   network time: Sun, 08 Sep 2019 11:00:34 GMT
2019-09-08T11:00:34.9811240Z == end clock drift check ==
2019-09-08T11:00:34.9811240Z == end clock drift check ==
2019-09-08T11:00:35.6170659Z ##[error]Bash exited with code '3'.
2019-09-08T11:00:35.6204762Z ##[section]Starting: Checkout
2019-09-08T11:00:35.6206768Z ==============================================================================
2019-09-08T11:00:35.6206832Z Task         : Get sources
2019-09-08T11:00:35.6206871Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
