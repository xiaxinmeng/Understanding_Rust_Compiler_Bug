
2020-07-28T13:54:52.5177406Z failures:
2020-07-28T13:54:52.5177550Z 
2020-07-28T13:54:52.5182260Z ---- lto::dev_profile stdout ----
2020-07-28T13:54:52.5182838Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test -v`
2020-07-28T13:54:52.5183333Z thread 'lto::dev_profile' panicked at '
2020-07-28T13:54:52.5183512Z Expected: execs
2020-07-28T13:54:52.5183700Z     but: differences:
2020-07-28T13:54:52.5184158Z   6 - |[RUNNING] `rustc --crate-name foo [..]--crate-type lib --emit=dep-info,metadata,link -Clinker-plugin-lto [..]|
2020-07-28T13:54:52.5185363Z     + |     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -Cembed-bitcode=no -C debuginfo=2 --test -C metadata=110e52c96c1b8618 -C extra-filename=-110e52c96c1b8618 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps/libbar-d2876efba7515ca8.rlib`|
2020-07-28T13:54:52.5194205Z 
2020-07-28T13:54:52.5195505Z   7 - |[RUNNING] `rustc --crate-name foo [..]--emit=dep-info,link -Cembed-bitcode=no [..]--test[..]|
2020-07-28T13:54:52.5197737Z     + |     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -Clinker-plugin-lto -C debuginfo=2 -C metadata=aae1986730f9b7d9 -C extra-filename=-aae1986730f9b7d9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps/libbar-d2876efba7515ca8.rmeta`|
2020-07-28T13:54:52.5198297Z 
2020-07-28T13:54:52.5198427Z 
2020-07-28T13:54:52.5198681Z other output:
2020-07-28T13:54:52.5198971Z `
2020-07-28T13:54:52.5199230Z running 1 test
2020-07-28T13:54:52.5199752Z test t1 ... ok
2020-07-28T13:54:52.5200043Z 
2020-07-28T13:54:52.5200582Z test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-07-28T13:54:52.5200749Z 
2020-07-28T13:54:52.5200827Z 
2020-07-28T13:54:52.5201014Z running 0 tests
2020-07-28T13:54:52.5201113Z 
2020-07-28T13:54:52.5201289Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-07-28T13:54:52.5201422Z 
2020-07-28T13:54:52.5201971Z `', src/tools/cargo/crates/cargo-test-support/src/lib.rs:832:13
2020-07-28T13:54:52.5202177Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-28T13:54:52.5202307Z 
2020-07-28T13:54:52.5202397Z 
2020-07-28T13:54:52.5202544Z failures:
2020-07-28T13:54:52.5202715Z     lto::dev_profile
