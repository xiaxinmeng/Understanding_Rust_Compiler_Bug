plain
2020-01-15T06:38:06.9787039Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T06:38:06.9875135Z ##[command]git config gc.auto 0
2020-01-15T06:38:06.9960592Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T06:38:07.0023553Z ##[command]git config --get-all http.proxy
2020-01-15T06:38:07.0174415Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68218/merge:refs/remotes/pull/68218/merge
---
2020-01-15T06:51:29.1684688Z configure: build.locked-deps    := True
2020-01-15T06:51:29.1684741Z configure: llvm.ccache          := sccache
2020-01-15T06:51:29.1684919Z configure: build.cargo-native-static := True
2020-01-15T06:51:29.1685077Z configure: dist.missing-tools   := True
2020-01-15T06:51:29.1685289Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-15T06:51:29.1685369Z configure: writing `config.toml` in current directory
2020-01-15T06:51:29.1685404Z configure: 
2020-01-15T06:51:29.1685616Z configure: run `python /checkout/x.py --help`
2020-01-15T06:51:29.1685652Z configure: 
---
2020-01-15T06:55:34.9892085Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-15T06:55:36.1939438Z error[E0308]: mismatched types
2020-01-15T06:55:36.1941091Z    --> src/librustc_session/session.rs:616:32
2020-01-15T06:55:36.1942263Z     |
2020-01-15T06:55:36.1943262Z 616 |             session_directory: Lrc::new(session_dir),
2020-01-15T06:55:36.1943987Z     |                                ^^^^^^^^^^^^^^^^^^^^^ expected struct `std::sync::Arc`, found struct `std::rc::Rc`
2020-01-15T06:55:36.1945676Z     = note: expected struct `std::sync::Arc<std::path::PathBuf>`
2020-01-15T06:55:36.1947143Z                found struct `std::rc::Rc<std::path::PathBuf>`
2020-01-15T06:55:36.1947781Z 
2020-01-15T06:55:36.2496186Z error[E0308]: mismatched types
2020-01-15T06:55:36.2496186Z error[E0308]: mismatched types
2020-01-15T06:55:36.2497353Z    --> src/librustc_session/session.rs:632:61
2020-01-15T06:55:36.2497956Z     |
2020-01-15T06:55:36.2498577Z 632 |             IncrCompSession::Finalized { session_directory: Lrc::new(new_directory_path) };
2020-01-15T06:55:36.2499306Z     |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::sync::Arc`, found struct `std::rc::Rc`
2020-01-15T06:55:36.2500798Z     = note: expected struct `std::sync::Arc<std::path::PathBuf>`
2020-01-15T06:55:36.2501416Z                found struct `std::rc::Rc<std::path::PathBuf>`
2020-01-15T06:55:36.2501670Z 
2020-01-15T06:55:36.3149069Z error: aborting due to 2 previous errors
2020-01-15T06:55:36.3149069Z error: aborting due to 2 previous errors
2020-01-15T06:55:36.3150622Z 
2020-01-15T06:55:36.3152402Z For more information about this error, try `rustc --explain E0308`.
2020-01-15T06:55:36.3302671Z error: could not compile `rustc_session`.
2020-01-15T06:55:36.3302805Z 
2020-01-15T06:55:36.3303085Z To learn more, run the command again with --verbose.
2020-01-15T06:55:36.3346255Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-15T06:55:36.3360706Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-15T06:55:36.3360846Z Build completed unsuccessfully in 0:02:32
2020-01-15T06:55:36.3418453Z == clock drift check ==
2020-01-15T06:55:36.3435093Z   local time: Wed Jan 15 06:55:36 UTC 2020
2020-01-15T06:55:36.3435093Z   local time: Wed Jan 15 06:55:36 UTC 2020
2020-01-15T06:55:36.4417382Z   network time: Wed, 15 Jan 2020 06:55:36 GMT
2020-01-15T06:55:36.4420777Z == end clock drift check ==
2020-01-15T06:55:37.1901098Z 
2020-01-15T06:55:37.2006482Z ##[error]Bash exited with code '1'.
2020-01-15T06:55:37.2033143Z ##[section]Starting: Checkout
2020-01-15T06:55:37.2034999Z ==============================================================================
2020-01-15T06:55:37.2035048Z Task         : Get sources
2020-01-15T06:55:37.2035108Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
