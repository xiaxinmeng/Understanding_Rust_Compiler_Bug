plain
travis_time:end:122b525e:start=1554679127039457274,finish=1554679208375976877,duration=81336519603
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:03] 
[01:19:03] running 9 tests
[01:19:03] iiiiiiiii
[01:19:03] 
[01:19:03]  finished in 0.173
[01:19:03] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:21] 
[01:19:21] running 121 tests
[01:19:53] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:19:59] i.i......iii.i.....ii
[01:19:59] 
[01:19:59]  finished in 38.075
[01:19:59] travis_fold:end:test_debuginfo

---
[01:43:00]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2806:17
[01:43:29]      |
[01:43:29] 2806 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("b"))),
[01:43:29] 2807 | |                             ExternEntry::new_public(Some(String::from("c")))
[01:43:29] 2808 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2812:17
[01:43:29]      |
[01:43:29]      |
[01:43:29] 2812 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("e"))),
[01:43:29] 2813 | |                             ExternEntry::new_public(Some(String::from("f")))
[01:43:29] 2814 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2821:17
[01:43:29]      |
[01:43:29]      |
[01:43:29] 2821 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("e"))),
[01:43:29] 2822 | |                             ExternEntry::new_public(Some(String::from("f")))
[01:43:29] 2823 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2827:17
[01:43:29]      |
[01:43:29]      |
[01:43:29] 2827 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("b"))),
[01:43:29] 2828 | |                             ExternEntry::new_public(Some(String::from("c")))
[01:43:29] 2829 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2836:17
[01:43:29]      |
[01:43:29]      |
[01:43:29] 2836 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("b"))),
[01:43:29] 2837 | |                             ExternEntry::new_public(Some(String::from("c")))
[01:43:29] 2838 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:29] error[E0308]: mismatched types
[01:43:29]     --> src/librustc/session/config.rs:2842:17
[01:43:29]      |
[01:43:29]      |
[01:43:29] 2842 | /                 mk_set(vec![ExternEntry::new_public(Some(String::from("f"))),
[01:43:29] 2843 | |                             ExternEntry::new_public(Some(String::from("e")))
[01:43:29] 2844 | |                             ]),
[01:43:29]      | |______________________________^ expected struct `session::config::ExternEntry`, found struct `std::collections::BTreeSet`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]      = note: expected type `session::config::ExternEntry`
[01:43:29]                 found type `std::collections::BTreeSet<session::config::ExternEntry>`
[01:43:38] error: aborting due to 6 previous errors
[01:43:38] 
[01:43:38] For more information about this error, try `rustc --explain E0308`.
[01:43:39] error: Could not compile `rustc`.
[01:43:39] error: Could not compile `rustc`.
[01:43:39] 
[01:43:39] To learn more, run the command again with --verbose.
[01:43:39] 
[01:43:39] 
[01:43:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:43:39] 
[01:43:39] 
[01:43:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:39] Build completed unsuccessfully in 0:38:08
[01:43:39] Build completed unsuccessfully in 0:38:08
[01:43:39] make: *** [check] Error 1
[01:43:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32f8d6af
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 01:03:57 UTC 2019
