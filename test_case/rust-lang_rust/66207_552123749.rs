plain
2019-11-09T18:07:09.7533354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T18:07:09.7733886Z ##[command]git config gc.auto 0
2019-11-09T18:07:09.7830743Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T18:07:09.7886333Z ##[command]git config --get-all http.proxy
2019-11-09T18:07:10.7931775Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66207/merge:refs/remotes/pull/66207/merge
---
2019-11-09T18:10:44.5203995Z ######################################################################## 100.0%
2019-11-09T18:10:44.9800804Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-09T18:10:45.0713340Z     Updating crates.io index
2019-11-09T18:10:51.1512750Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:10:56.9757573Z     Updating git repository `***-clippy`
2019-11-09T18:11:01.9055099Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-09T18:11:01.9089034Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-09T18:11:01.9162397Z Makefile:67: recipe for target 'prepare' failed
2019-11-09T18:11:01.9162660Z make: *** [prepare] Error 1
2019-11-09T18:11:02.9182414Z Command failed. Attempt 2/5:
2019-11-09T18:11:03.0634744Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:03.0634744Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:03.4534653Z     Updating git repository `***-clippy`
2019-11-09T18:11:04.3509084Z     Updating crates.io index
2019-11-09T18:11:04.6572772Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-09T18:11:04.6593219Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-09T18:11:04.6643431Z Makefile:67: recipe for target 'prepare' failed
2019-11-09T18:11:04.6643528Z make: *** [prepare] Error 1
2019-11-09T18:11:06.6661615Z Command failed. Attempt 3/5:
2019-11-09T18:11:06.8177624Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:06.8177624Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:07.0988773Z     Updating git repository `***-clippy`
2019-11-09T18:11:07.3496948Z     Updating crates.io index
2019-11-09T18:11:07.6311953Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-09T18:11:07.6332287Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-09T18:11:07.6385252Z Makefile:67: recipe for target 'prepare' failed
2019-11-09T18:11:07.6385331Z make: *** [prepare] Error 1
2019-11-09T18:11:10.6407144Z Command failed. Attempt 4/5:
2019-11-09T18:11:10.7954773Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:10.7954773Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:11.0677286Z     Updating git repository `***-clippy`
2019-11-09T18:11:11.3170651Z     Updating crates.io index
2019-11-09T18:11:11.6012613Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-09T18:11:11.6034537Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-09T18:11:11.6086978Z Makefile:67: recipe for target 'prepare' failed
2019-11-09T18:11:11.6087140Z make: *** [prepare] Error 1
2019-11-09T18:11:15.6119743Z Command failed. Attempt 5/5:
2019-11-09T18:11:15.7589555Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:15.7589555Z     Updating git repository `https://github.com/rust-lang/cargo`
2019-11-09T18:11:16.0323820Z     Updating git repository `***-clippy`
2019-11-09T18:11:16.2807941Z     Updating crates.io index
2019-11-09T18:11:16.5577093Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-09T18:11:16.5603013Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-09T18:11:16.5658437Z make: *** [prepare] Error 1
2019-11-09T18:11:16.5659163Z Makefile:67: recipe for target 'prepare' failed
2019-11-09T18:11:16.5659613Z The command has failed after 5 attempts.
2019-11-09T18:11:16.5659786Z == clock drift check ==
2019-11-09T18:11:16.5659786Z == clock drift check ==
2019-11-09T18:11:16.5667550Z   local time: Sat Nov  9 18:11:16 UTC 2019
2019-11-09T18:11:16.7150652Z   network time: Sat, 09 Nov 2019 18:11:16 GMT
2019-11-09T18:11:16.7155454Z == end clock drift check ==
2019-11-09T18:11:21.3459440Z 
2019-11-09T18:11:21.3566893Z ##[error]Bash exited with code '1'.
2019-11-09T18:11:21.3627496Z ##[section]Starting: Checkout
2019-11-09T18:11:21.3629589Z ==============================================================================
2019-11-09T18:11:21.3629848Z Task         : Get sources
2019-11-09T18:11:21.3629906Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
