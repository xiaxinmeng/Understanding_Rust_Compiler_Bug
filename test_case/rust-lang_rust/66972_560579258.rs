plain
2019-12-02T19:40:20.9447474Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T19:40:20.9661889Z ##[command]git config gc.auto 0
2019-12-02T19:40:20.9770076Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T19:40:20.9842038Z ##[command]git config --get-all http.proxy
2019-12-02T19:40:20.9998541Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66972/merge:refs/remotes/pull/66972/merge
---
2019-12-02T19:48:52.5000078Z 
2019-12-02T19:48:52.9534070Z error[E0061]: this function takes 2 parameters but 1 parameter was supplied
2019-12-02T19:48:52.9534446Z    --> src/librustc_interface/util.rs:184:22
2019-12-02T19:48:52.9534963Z     |
2019-12-02T19:48:52.9535351Z 184 |       let mut config = ThreadPoolBuilder::spawn_handler(|thread| {
2019-12-02T19:48:52.9536008Z 185 | |         let mut b = thread::Builder::new();
2019-12-02T19:48:52.9536008Z 185 | |         let mut b = thread::Builder::new();
2019-12-02T19:48:52.9536353Z 186 | |         if let Some(name) = thread.name() {
2019-12-02T19:48:52.9536699Z 187 | |             b = b.name(name.to_owned());
2019-12-02T19:48:52.9537246Z 199 | |         Ok(())
2019-12-02T19:48:52.9537246Z 199 | |         Ok(())
2019-12-02T19:48:52.9537593Z 200 | |     }).thread_name(|_| "rustc".to_string())
2019-12-02T19:48:52.9537935Z 
2019-12-02T19:48:52.9537935Z 
2019-12-02T19:48:53.0113439Z error[E0599]: no method named `build_scoped` found for type `rustc_rayon_core::ThreadPoolBuilder<rustc_rayon_core::registry::CustomSpawn<_>>` in the current scope
2019-12-02T19:48:53.0114111Z    --> src/librustc_interface/util.rs:230:24
2019-12-02T19:48:53.0114380Z     |
2019-12-02T19:48:53.0114708Z 230 |                 config.build_scoped(main_handler, with_pool).unwrap()
2019-12-02T19:48:53.0115150Z     |                        ^^^^^^^^^^^^ method not found in `rustc_rayon_core::ThreadPoolBuilder<rustc_rayon_core::registry::CustomSpawn<_>>`
2019-12-02T19:48:53.0777261Z error: aborting due to 3 previous errors
2019-12-02T19:48:53.0777379Z 
2019-12-02T19:48:53.0777732Z Some errors have detailed explanations: E0061, E0433, E0599.
2019-12-02T19:48:53.0778028Z For more information about an error, try `rustc --explain E0061`.
---
2019-12-02T19:48:53.1150107Z   local time: Mon Dec  2 19:48:53 UTC 2019
2019-12-02T19:48:53.2673233Z   network time: Mon, 02 Dec 2019 19:48:53 GMT
2019-12-02T19:48:53.2677341Z == end clock drift check ==
2019-12-02T19:48:54.6133139Z 
2019-12-02T19:48:54.6234947Z ##[error]Bash exited with code '1'.
2019-12-02T19:48:54.6264574Z ##[section]Starting: Checkout
2019-12-02T19:48:54.6266332Z ==============================================================================
2019-12-02T19:48:54.6266387Z Task         : Get sources
2019-12-02T19:48:54.6266453Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
