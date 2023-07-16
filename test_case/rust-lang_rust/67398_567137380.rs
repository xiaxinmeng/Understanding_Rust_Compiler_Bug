plain
2019-12-18T16:00:14.1503629Z    Compiling flate2 v1.0.12
2019-12-18T16:00:16.6226463Z    Compiling rls-span v0.5.1
2019-12-18T16:00:18.4101597Z    Compiling serde_json v1.0.40
2019-12-18T16:00:22.7814766Z    Compiling rand_core v0.5.1
2019-12-18T16:00:23.3881918Z    Compiling rustc_jobserver v0.0.0 (/checkout/src/librustc_jobserver)
2019-12-18T16:00:26.4136714Z    Compiling polonius-engine v0.11.0
2019-12-18T16:00:28.4901644Z    Compiling backtrace v0.3.40
2019-12-18T16:00:30.3857576Z    Compiling rls-data v0.19.0
2019-12-18T16:00:33.9364973Z    Compiling rand_chacha v0.2.1
---
2019-12-18T16:25:45.1188695Z    Compiling flate2 v1.0.12
2019-12-18T16:25:47.4262082Z    Compiling crossbeam-deque v0.7.1
2019-12-18T16:25:48.3921265Z    Compiling polonius-engine v0.11.0
2019-12-18T16:25:48.5212961Z    Compiling chalk-engine v0.9.0
2019-12-18T16:25:49.0670451Z    Compiling rustc_jobserver v0.0.0 (/checkout/src/librustc_jobserver)
2019-12-18T16:25:50.2339892Z    Compiling backtrace v0.3.40
2019-12-18T16:25:50.2644040Z    Compiling rand_chacha v0.2.1
2019-12-18T16:25:52.4909576Z    Compiling quote v1.0.2
2019-12-18T16:25:55.8298766Z    Compiling rand v0.7.0
---
2019-12-18T16:57:53.9049093Z    Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
2019-12-18T16:57:54.0351869Z     Finished release [optimized] target(s) in 3m 50s
2019-12-18T16:57:54.0680354Z thread '<unnamed>' panicked at 'assertion failed: `(left != right)`
2019-12-18T16:57:54.0681945Z   left: `0`,
2019-12-18T16:57:54.0683441Z  right: `0`: jobserver must be initialized', src/librustc_jobserver/lib.rs:41:5
2019-12-18T16:57:54.0685774Z thread '<unnamed>' panicked at 'assertion failed: `(left != right)`
2019-12-18T16:57:54.0687382Z   left: `0`,
2019-12-18T16:57:54.0687382Z   left: `0`,
2019-12-18T16:57:54.0689127Z  right: `0`: jobserver must be initialized', src/librustc_jobserver/lib.rs:41:5
2019-12-18T16:57:54.0691136Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2019-12-18T16:57:54.0691767Z   left: `0`,
2019-12-18T16:57:54.0692253Z  right: `0`: jobserver must be initialized', src/librustc_jobserver/lib.rs:41:5
2019-12-18T16:57:54.0692478Z Rayon: detected unexpected panic; aborting
2019-12-18T16:57:54.3178901Z 
2019-12-18T16:57:54.3178901Z 
2019-12-18T16:57:54.3180729Z command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/favicon.inc" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "/checkout/src/doc/index.md" "--markdown-css" "rust.css"
2019-12-18T16:57:54.3181298Z 
2019-12-18T16:57:54.3181459Z 
2019-12-18T16:57:54.3185186Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-12-18T16:57:54.3186274Z Build completed unsuccessfully in 1:35:28
2019-12-18T16:57:54.3186274Z Build completed unsuccessfully in 1:35:28
2019-12-18T16:57:54.3238209Z == clock drift check ==
2019-12-18T16:57:54.3261902Z   local time: Wed Dec 18 16:57:54 UTC 2019
2019-12-18T16:57:54.7725171Z   network time: Wed, 18 Dec 2019 16:57:54 GMT
2019-12-18T16:57:54.7726807Z == end clock drift check ==
2019-12-18T16:57:56.1420995Z 
2019-12-18T16:57:56.1521708Z ##[error]Bash exited with code '1'.
2019-12-18T16:57:56.1591109Z ##[section]Starting: Checkout
2019-12-18T16:57:56.1593116Z ==============================================================================
2019-12-18T16:57:56.1593232Z Task         : Get sources
2019-12-18T16:57:56.1593316Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
