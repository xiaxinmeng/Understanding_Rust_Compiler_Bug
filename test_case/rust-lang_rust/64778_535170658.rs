plain
2019-09-25T19:10:58.0617138Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T19:10:58.0824396Z ##[command]git config gc.auto 0
2019-09-25T19:10:58.0915728Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T19:10:58.0989120Z ##[command]git config --get-all http.proxy
2019-09-25T19:10:58.1159676Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-25T19:14:43.2232667Z 
2019-09-25T19:14:43.2234006Z ######################################################################## 100.0%
2019-09-25T19:14:43.6656714Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-25T19:14:43.7472410Z     Updating crates.io index
2019-09-25T19:15:20.7537076Z error: no matching package named `index` found
2019-09-25T19:15:20.7537967Z location searched: /checkout/src/librustc_index
2019-09-25T19:15:20.7538217Z required by package `rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)`
2019-09-25T19:15:20.7560748Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T19:15:20.7602728Z make: *** [prepare] Error 1
2019-09-25T19:15:20.7609010Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T19:15:21.7634524Z Command failed. Attempt 2/5:
2019-09-25T19:15:21.7634524Z Command failed. Attempt 2/5:
2019-09-25T19:15:21.9042007Z error: no matching package named `index` found
2019-09-25T19:15:21.9042883Z location searched: /checkout/src/librustc_index
2019-09-25T19:15:21.9043136Z required by package `rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)`
2019-09-25T19:15:21.9060313Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T19:15:21.9103966Z make: *** [prepare] Error 1
2019-09-25T19:15:21.9108039Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T19:15:23.9125723Z Command failed. Attempt 3/5:
2019-09-25T19:15:23.9125723Z Command failed. Attempt 3/5:
2019-09-25T19:15:24.0558463Z error: no matching package named `index` found
2019-09-25T19:15:24.0558709Z location searched: /checkout/src/librustc_index
2019-09-25T19:15:24.0558779Z required by package `rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)`
2019-09-25T19:15:24.0573442Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T19:15:24.0614503Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T19:15:24.0620491Z make: *** [prepare] Error 1
2019-09-25T19:15:27.0631351Z Command failed. Attempt 4/5:
2019-09-25T19:15:27.0631351Z Command failed. Attempt 4/5:
2019-09-25T19:15:27.2081588Z error: no matching package named `index` found
2019-09-25T19:15:27.2082612Z location searched: /checkout/src/librustc_index
2019-09-25T19:15:27.2083056Z required by package `rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)`
2019-09-25T19:15:27.2098153Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T19:15:27.2137571Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T19:15:27.2138111Z make: *** [prepare] Error 1
2019-09-25T19:15:31.2161967Z Command failed. Attempt 5/5:
2019-09-25T19:15:31.2161967Z Command failed. Attempt 5/5:
2019-09-25T19:15:31.3552282Z error: no matching package named `index` found
2019-09-25T19:15:31.3552987Z location searched: /checkout/src/librustc_index
2019-09-25T19:15:31.3553191Z required by package `rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)`
2019-09-25T19:15:31.3562480Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-25T19:15:31.3607000Z Makefile:67: recipe for target 'prepare' failed
2019-09-25T19:15:31.3611341Z make: *** [prepare] Error 1
2019-09-25T19:15:31.3633578Z The command has failed after 5 attempts.
2019-09-25T19:15:31.3633697Z == clock drift check ==
2019-09-25T19:15:31.3633697Z == clock drift check ==
2019-09-25T19:15:31.3633801Z   local time: Wed Sep 25 19:15:31 UTC 2019
2019-09-25T19:15:31.4524700Z   network time: Wed, 25 Sep 2019 19:15:31 GMT
2019-09-25T19:15:31.4531352Z == end clock drift check ==
2019-09-25T19:15:32.3137560Z ##[error]Bash exited with code '1'.
2019-09-25T19:15:32.3174114Z ##[section]Starting: Checkout
2019-09-25T19:15:32.3176051Z ==============================================================================
2019-09-25T19:15:32.3176109Z Task         : Get sources
2019-09-25T19:15:32.3176158Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
