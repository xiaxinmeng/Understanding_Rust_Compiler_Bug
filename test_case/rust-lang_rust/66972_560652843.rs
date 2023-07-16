plain
2019-12-02T20:20:17.9107895Z 
2019-12-02T20:20:18.3508781Z error[E0061]: this function takes 2 parameters but 1 parameter was supplied
2019-12-02T20:20:18.3510053Z    --> src/librustc_interface/util.rs:184:22
2019-12-02T20:20:18.3510586Z     |
2019-12-02T20:20:18.3511208Z 184 |       let mut config = ThreadPoolBuilder::spawn_handler(|thread| {
2019-12-02T20:20:18.3512351Z 185 | |         let mut b = thread::Builder::new();
2019-12-02T20:20:18.3512351Z 185 | |         let mut b = thread::Builder::new();
2019-12-02T20:20:18.3512968Z 186 | |         if let Some(name) = thread.name() {
2019-12-02T20:20:18.3513585Z 187 | |             b = b.name(name.to_owned());
2019-12-02T20:20:18.3514602Z 199 | |         Ok(())
2019-12-02T20:20:18.3514602Z 199 | |         Ok(())
2019-12-02T20:20:18.3515509Z 200 | |     }).thread_name(|_| "rustc".to_string())
2019-12-02T20:20:18.3516445Z 
2019-12-02T20:20:18.3516445Z 
2019-12-02T20:20:18.4124332Z error[E0599]: no method named `build_scoped` found for type `rustc_rayon_core::ThreadPoolBuilder<rustc_rayon_core::registry::CustomSpawn<_>>` in the current scope
2019-12-02T20:20:18.4125811Z    --> src/librustc_interface/util.rs:230:24
2019-12-02T20:20:18.4126431Z     |
2019-12-02T20:20:18.4127030Z 230 |                 config.build_scoped(main_handler, with_pool).unwrap()
2019-12-02T20:20:18.4127794Z     |                        ^^^^^^^^^^^^ method not found in `rustc_rayon_core::ThreadPoolBuilder<rustc_rayon_core::registry::CustomSpawn<_>>`
2019-12-02T20:20:18.4798908Z error: aborting due to 3 previous errors
2019-12-02T20:20:18.4799816Z 
2019-12-02T20:20:18.4803408Z Some errors have detailed explanations: E0061, E0433, E0599.
2019-12-02T20:20:18.4811207Z For more information about an error, try `rustc --explain E0061`.
---
2019-12-02T20:20:30.7328940Z   local time: Mon Dec  2 20:20:30 UTC 2019
2019-12-02T20:20:31.2223644Z   network time: Mon, 02 Dec 2019 20:20:31 GMT
2019-12-02T20:20:31.2226063Z == end clock drift check ==
2019-12-02T20:20:32.2544201Z 
2019-12-02T20:20:32.2653933Z ##[error]Bash exited with code '1'.
2019-12-02T20:20:32.2706312Z ##[section]Starting: Checkout
2019-12-02T20:20:32.2708466Z ==============================================================================
2019-12-02T20:20:32.2708577Z Task         : Get sources
2019-12-02T20:20:32.2708697Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
