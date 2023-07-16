plain
2020-04-07T02:14:58.9311076Z ========================== Starting Command Output ===========================
2020-04-07T02:14:58.9315904Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68d1778d-63d4-4049-9ac8-c0906ac3f7e3.sh
2020-04-07T02:14:58.9316399Z 
2020-04-07T02:14:58.9322744Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T02:14:58.9341986Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-07T02:14:58.9345994Z Task         : Get sources
2020-04-07T02:14:58.9346477Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:14:58.9347236Z Version      : 1.0.0
2020-04-07T02:14:58.9350130Z Author       : Microsoft
---
2020-04-07T02:14:59.9171932Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T02:14:59.9176900Z ##[command]git config gc.auto 0
2020-04-07T02:14:59.9180570Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T02:14:59.9183536Z ##[command]git config --get-all http.proxy
2020-04-07T02:14:59.9189244Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70873/merge:refs/remotes/pull/70873/merge
---
2020-04-07T02:16:52.8798653Z Looks like docker image is the same as before, not uploading
2020-04-07T02:17:00.8458558Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-07T02:17:00.8740456Z [CI_JOB_NAME=x86_64-gnu-tools]
2020-04-07T02:17:00.8765951Z == clock drift check ==
2020-04-07T02:17:00.8766643Z   local time: Tue Apr  7 02:17:00 UTC 2020
2020-04-07T02:17:01.1672092Z   network time: Tue, 07 Apr 2020 02:17:01 GMT
2020-04-07T02:17:01.1693550Z Starting sccache server...
2020-04-07T02:17:01.2490897Z configure: processing command line
2020-04-07T02:17:01.2491170Z configure: 
2020-04-07T02:17:01.2492185Z configure: rust.dist-src        := False
---
2020-04-07T02:30:22.9574257Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T02:30:24.8580689Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T02:30:26.0392721Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T02:30:33.1279074Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T02:30:42.6840845Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T02:30:48.6248423Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T02:30:54.7879235Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T02:31:00.8479722Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T02:31:11.3441052Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T03:02:35.2469125Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T03:02:38.5938842Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T03:02:40.3336447Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T03:02:53.6991401Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T03:03:07.6154597Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T03:03:18.6120525Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T03:03:28.8480845Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T03:03:39.0778662Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T03:03:55.2070746Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T04:01:53.7917360Z  finished in 66.096
2020-04-07T04:01:53.7925165Z Testing rustbook src/doc/nomicon
2020-04-07T04:02:03.1388642Z Error: Rustdoc returned an error: 
2020-04-07T04:02:03.1388917Z running 1 test
2020-04-07T04:02:03.1390326Z test /tmp/mdbook-udE2bI/vec-final.md - The_Final_Code (line 3) ... FAILED
2020-04-07T04:02:03.1390647Z failures:
2020-04-07T04:02:03.1390767Z 
2020-04-07T04:02:03.1390767Z 
2020-04-07T04:02:03.1391180Z ---- /tmp/mdbook-udE2bI/vec-final.md - The_Final_Code (line 3) stdout ----
2020-04-07T04:02:03.1391514Z error[E0061]: this function takes 2 arguments but 1 argument was supplied
2020-04-07T04:02:03.1391984Z   --> /tmp/mdbook-udE2bI/vec-final.md:37:34
2020-04-07T04:02:03.1392420Z 35 |                 let ptr = Global.alloc(Layout::array::<T>(1).unwrap());
2020-04-07T04:02:03.1393165Z    |                                  ^^^^^ ------------------------------ supplied 1 argument
2020-04-07T04:02:03.1393617Z    |                                  |
2020-04-07T04:02:03.1393850Z    |                                  expected 2 arguments
2020-04-07T04:02:03.1393850Z    |                                  expected 2 arguments
2020-04-07T04:02:03.1394032Z 
2020-04-07T04:02:03.1394295Z error[E0599]: no method named `realloc` found for struct `std::alloc::Global` in the current scope
2020-04-07T04:02:03.1395144Z   --> /tmp/mdbook-udE2bI/vec-final.md:42:34
2020-04-07T04:02:03.1395534Z 40 |                 let ptr = Global.realloc(c.cast(),
2020-04-07T04:02:03.1395881Z    |                                  ^^^^^^^ method not found in `std::alloc::Global`
2020-04-07T04:02:03.1396127Z 
2020-04-07T04:02:03.1396307Z error: aborting due to 2 previous errors
2020-04-07T04:02:03.1396307Z error: aborting due to 2 previous errors
2020-04-07T04:02:03.1396456Z 
2020-04-07T04:02:03.1396705Z Some errors have detailed explanations: E0061, E0599.
2020-04-07T04:02:03.1397187Z For more information about an error, try `rustc --explain E0061`.
2020-04-07T04:02:03.1397576Z Couldn't compile the test.
2020-04-07T04:02:03.1397704Z 
2020-04-07T04:02:03.1397817Z failures:
2020-04-07T04:02:03.1398202Z     /tmp/mdbook-udE2bI/vec-final.md - The_Final_Code (line 3)
2020-04-07T04:02:03.1398609Z test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T04:02:03.1399000Z 
2020-04-07T04:02:03.1399102Z 
2020-04-07T04:02:03.1410298Z 
---
2020-04-07T04:05:13.8187655Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"
2020-04-07T04:05:13.8190720Z 
2020-04-07T04:05:13.8194057Z      ┌── walkthrough.md:107:51 ───
2020-04-07T04:05:13.8196688Z      │
2020-04-07T04:05:13.8200000Z  107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
2020-04-07T04:05:13.8207833Z      │
2020-04-07T04:05:13.8211397Z 
2020-04-07T04:05:13.8214868Z error: The server responded with 429 Too Many Requests for "***"
2020-04-07T04:05:13.8216364Z 
---
2020-04-07T04:05:13.8264046Z  156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
2020-04-07T04:05:13.8277551Z      │                                                         ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8277968Z      │
2020-04-07T04:05:13.8278239Z 
2020-04-07T04:05:13.8278720Z error: The server responded with 429 Too Many Requests for "***/pull/47732"
2020-04-07T04:05:13.8279200Z      ┌── walkthrough.md:167:58 ───
2020-04-07T04:05:13.8279457Z      │
2020-04-07T04:05:13.8279848Z  167 │ macro expansion in the compiler. Personally, I find that [improving the
2020-04-07T04:05:13.8280367Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8280367Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8280713Z      │
2020-04-07T04:05:13.8280824Z 
2020-04-07T04:05:13.8281804Z error: The server responded with 429 Too Many Requests for "***"
2020-04-07T04:05:13.8281986Z 
2020-04-07T04:05:13.8282312Z      ┌── walkthrough.md:181:27 ───
2020-04-07T04:05:13.8282595Z      │
2020-04-07T04:05:13.8282990Z  181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
2020-04-07T04:05:13.8283836Z      │
2020-04-07T04:05:13.8283930Z 
2020-04-07T04:05:13.8283930Z 
2020-04-07T04:05:13.8284413Z error: The server responded with 429 Too Many Requests for "***/issues/51934"
2020-04-07T04:05:13.8285252Z      ┌── walkthrough.md:237:32 ───
2020-04-07T04:05:13.8285515Z      │
2020-04-07T04:05:13.8285845Z  237 │   from the original RFC required [another
2020-04-07T04:05:13.8286218Z      │ ╭────────────────────────────────^
2020-04-07T04:05:13.8286218Z      │ ╭────────────────────────────────^
2020-04-07T04:05:13.8286624Z  238 │ │ FCP](***/issues/51934).
2020-04-07T04:05:13.8288021Z      │
2020-04-07T04:05:13.8288287Z 
2020-04-07T04:05:13.8288287Z 
2020-04-07T04:05:13.8288785Z error: The server responded with 429 Too Many Requests for "***/pull/56245"
2020-04-07T04:05:13.8289470Z      ┌── walkthrough.md:257:13 ───
2020-04-07T04:05:13.8289766Z      │
2020-04-07T04:05:13.8289766Z      │
2020-04-07T04:05:13.8290187Z  257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
2020-04-07T04:05:13.8291000Z      │
2020-04-07T04:05:13.8291094Z 
2020-04-07T04:05:13.8291094Z 
2020-04-07T04:05:13.8291606Z error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
2020-04-07T04:05:13.8292459Z      ┌── walkthrough.md:258:55 ───
2020-04-07T04:05:13.8292731Z      │
2020-04-07T04:05:13.8292731Z      │
2020-04-07T04:05:13.8293128Z  258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
2020-04-07T04:05:13.8294071Z      │
2020-04-07T04:05:13.8294161Z 
2020-04-07T04:05:13.8294713Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"
2020-04-07T04:05:13.8295028Z 
---
2020-04-07T04:05:13.8305325Z  81 │ example of how such an issue should look can be [found
2020-04-07T04:05:13.8305856Z     │                                                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8306241Z     │
2020-04-07T04:05:13.8306335Z 
2020-04-07T04:05:13.8307154Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-04-07T04:05:13.8307942Z      ┌── bug-fix-procedure.md:237:65 ───
2020-04-07T04:05:13.8308231Z      │
2020-04-07T04:05:13.8308599Z  237 │ The first reference you will likely find is the lint definition [in
2020-04-07T04:05:13.8309176Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8309176Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8309920Z      │
2020-04-07T04:05:13.8310015Z 
2020-04-07T04:05:13.8310670Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-04-07T04:05:13.8311281Z      ┌── bug-fix-procedure.md:252:13 ───
2020-04-07T04:05:13.8311591Z      │
2020-04-07T04:05:13.8311591Z      │
2020-04-07T04:05:13.8311978Z  252 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
2020-04-07T04:05:13.8312918Z      │
2020-04-07T04:05:13.8313005Z 
2020-04-07T04:05:13.8313005Z 
2020-04-07T04:05:13.8314033Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
2020-04-07T04:05:13.8314691Z      ┌── bug-fix-procedure.md:256:19 ───
2020-04-07T04:05:13.8314983Z      │
2020-04-07T04:05:13.8315389Z  256 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
2020-04-07T04:05:13.8315855Z      │                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8315855Z      │                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8316173Z      │
2020-04-07T04:05:13.8316283Z 
2020-04-07T04:05:13.8316945Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
2020-04-07T04:05:13.8318062Z      ┌── bug-fix-procedure.md:285:16 ───
2020-04-07T04:05:13.8318346Z      │
2020-04-07T04:05:13.8318346Z      │
2020-04-07T04:05:13.8318893Z  285 │ this case, the [`add_lint` call][addlintsource] looks like this:
2020-04-07T04:05:13.8319689Z      │
2020-04-07T04:05:13.8319782Z 
2020-04-07T04:05:13.8320318Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"
2020-04-07T04:05:13.8320644Z 
2020-04-07T04:05:13.8320644Z 
2020-04-07T04:05:13.8320984Z     ┌── implementing_new_features.md:56:4 ───
2020-04-07T04:05:13.8321279Z     │
2020-04-07T04:05:13.8321834Z  56 │ We [value the stability of Rust]. Code that works and runs on stable
2020-04-07T04:05:13.8322260Z     │    ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8322545Z     │
2020-04-07T04:05:13.8322634Z 
2020-04-07T04:05:13.8323109Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-04-07T04:05:13.8323580Z     ┌── stability.md:18:51 ───
2020-04-07T04:05:13.8323863Z     │
2020-04-07T04:05:13.8324254Z  18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
2020-04-07T04:05:13.8324801Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8324801Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8325173Z     │
2020-04-07T04:05:13.8325261Z 
2020-04-07T04:05:13.8325729Z error: The server responded with 429 Too Many Requests for "***/issues/15702"
2020-04-07T04:05:13.8326235Z     ┌── stability.md:31:30 ───
2020-04-07T04:05:13.8326494Z     │
2020-04-07T04:05:13.8326494Z     │
2020-04-07T04:05:13.8326897Z  31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
2020-04-07T04:05:13.8327714Z     │
2020-04-07T04:05:13.8327802Z 
2020-04-07T04:05:13.8327802Z 
2020-04-07T04:05:13.8328332Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
2020-04-07T04:05:13.8329228Z     ┌── stabilization_guide.md:17:38 ───
2020-04-07T04:05:13.8329731Z     │
2020-04-07T04:05:13.8329731Z     │
2020-04-07T04:05:13.8330123Z  17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
2020-04-07T04:05:13.8331048Z     │
2020-04-07T04:05:13.8331327Z 
2020-04-07T04:05:13.8331824Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"
2020-04-07T04:05:13.8332194Z 
---
2020-04-07T04:05:13.8337751Z  28 │ - [The Book]: This may or may not need updating, depends.
2020-04-07T04:05:13.8338315Z     │   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8338672Z     │
2020-04-07T04:05:13.8338775Z 
2020-04-07T04:05:13.8339857Z error: The server responded with 429 Too Many Requests for "***-by-example"
2020-04-07T04:05:13.8358712Z     ┌── stabilization_guide.md:35:3 ───
2020-04-07T04:05:13.8359218Z     │
2020-04-07T04:05:13.8359517Z  35 │ - [Rust by Example]: As needed.
2020-04-07T04:05:13.8360071Z     │   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8360071Z     │   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8360376Z     │
2020-04-07T04:05:13.8360467Z 
2020-04-07T04:05:13.8360964Z error: The server responded with 429 Too Many Requests for "***/issues/32409"
2020-04-07T04:05:13.8361646Z     ┌── stabilization_guide.md:97:1 ───
2020-04-07T04:05:13.8361915Z     │
2020-04-07T04:05:13.8362182Z  97 │ [rust-lang/rust#32409]:
2020-04-07T04:05:13.8362546Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8362546Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8362820Z     │
2020-04-07T04:05:13.8362922Z 
2020-04-07T04:05:13.8363390Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc"
2020-04-07T04:05:13.8363824Z 
2020-04-07T04:05:13.8364133Z      ┌── compiler-debugging.md:257:5 ───
2020-04-07T04:05:13.8364423Z      │
2020-04-07T04:05:13.8364833Z  257 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
2020-04-07T04:05:13.8365559Z      │
2020-04-07T04:05:13.8365667Z 
2020-04-07T04:05:13.8366182Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/cargo-bisect-rustc/blob/master/TUTORIAL.md"
2020-04-07T04:05:13.8366477Z 
2020-04-07T04:05:13.8366477Z 
2020-04-07T04:05:13.8366787Z      ┌── compiler-debugging.md:261:31 ───
2020-04-07T04:05:13.8367244Z      │
2020-04-07T04:05:13.8367801Z  261 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
2020-04-07T04:05:13.8368816Z      │
2020-04-07T04:05:13.8368905Z 
2020-04-07T04:05:13.8369946Z error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"
2020-04-07T04:05:13.8370267Z 
2020-04-07T04:05:13.8370267Z 
2020-04-07T04:05:13.8370598Z      ┌── compiler-debugging.md:269:5 ───
2020-04-07T04:05:13.8371052Z      │
2020-04-07T04:05:13.8371446Z  269 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
2020-04-07T04:05:13.8372871Z      │
2020-04-07T04:05:13.8373122Z 
2020-04-07T04:05:13.8373122Z 
2020-04-07T04:05:13.8373791Z error: The server responded with 429 Too Many Requests for "***c-perf"
2020-04-07T04:05:13.8374453Z    ┌── profiling.md:8:9 ───
2020-04-07T04:05:13.8377204Z    │
2020-04-07T04:05:13.8377204Z    │
2020-04-07T04:05:13.8377965Z  8 │   - The [rustc-perf](***c-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
2020-04-07T04:05:13.8379688Z    │
2020-04-07T04:05:13.8379942Z 
2020-04-07T04:05:13.8394410Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"
2020-04-07T04:05:13.8394688Z 
2020-04-07T04:05:13.8394688Z 
2020-04-07T04:05:13.8395158Z     ┌── profiling.md:11:35 ───
2020-04-07T04:05:13.8424973Z     │
2020-04-07T04:05:13.8425727Z  11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
2020-04-07T04:05:13.8426882Z     │
2020-04-07T04:05:13.8426974Z 
2020-04-07T04:05:13.8427657Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"
2020-04-07T04:05:13.8427965Z 
2020-04-07T04:05:13.8427965Z 
2020-04-07T04:05:13.8428437Z     ┌── profiling.md:12:9 ───
2020-04-07T04:05:13.8428885Z     │
2020-04-07T04:05:13.8429519Z  12 │     See [their docs](https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md) for more information.
2020-04-07T04:05:13.8430202Z     │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8430798Z     │
2020-04-07T04:05:13.8430889Z 
2020-04-07T04:05:13.8431381Z error: The server responded with 429 Too Many Requests for "***c-perf"
2020-04-07T04:05:13.8431897Z     ┌── profiling/with_perf.md:59:1 ───
2020-04-07T04:05:13.8432168Z     │
2020-04-07T04:05:13.8432481Z  59 │ [the rustc-perf repository][rustc-perf-gh]:
2020-04-07T04:05:13.8433034Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8433034Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8433309Z     │
2020-04-07T04:05:13.8433395Z 
2020-04-07T04:05:13.8433930Z error: The server responded with 429 Too Many Requests for "***c-perf/blob/master/collector/README.md"
2020-04-07T04:05:13.8434642Z     ┌── profiling/with_perf.md:71:1 ───
2020-04-07T04:05:13.8434929Z     │
2020-04-07T04:05:13.8435289Z  71 │ [instructions in the rustc-perf readme][rustc-perf-readme].
2020-04-07T04:05:13.8435693Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8435693Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8435990Z     │
2020-04-07T04:05:13.8436080Z 
2020-04-07T04:05:13.8436609Z error: The server responded with 429 Too Many Requests for "***c-perf/tree/master/collector/benchmarks"
2020-04-07T04:05:13.8437165Z     ┌── profiling/with_perf.md:93:14 ───
2020-04-07T04:05:13.8437436Z     │
2020-04-07T04:05:13.8437436Z     │
2020-04-07T04:05:13.8437961Z  93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
2020-04-07T04:05:13.8438707Z     │
2020-04-07T04:05:13.8438792Z 
2020-04-07T04:05:13.8439427Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-04-07T04:05:13.8439661Z 
2020-04-07T04:05:13.8439661Z 
2020-04-07T04:05:13.8439948Z      ┌── profiling/with_perf.md:137:45 ───
2020-04-07T04:05:13.8440203Z      │
2020-04-07T04:05:13.8440751Z  137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
2020-04-07T04:05:13.8441769Z      │
2020-04-07T04:05:13.8441876Z 
2020-04-07T04:05:13.8442326Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-04-07T04:05:13.8442681Z 
2020-04-07T04:05:13.8442681Z 
2020-04-07T04:05:13.8443010Z      ┌── profiling/with_perf.md:161:38 ───
2020-04-07T04:05:13.8443307Z      │
2020-04-07T04:05:13.8443683Z  161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
2020-04-07T04:05:13.8444550Z      │
2020-04-07T04:05:13.8444640Z 
2020-04-07T04:05:13.8445237Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-dev-tools/fmt-rfcs"
2020-04-07T04:05:13.8445483Z 
2020-04-07T04:05:13.8445483Z 
2020-04-07T04:05:13.8445909Z     ┌── conventions.md:10:36 ───
2020-04-07T04:05:13.8446169Z     │
2020-04-07T04:05:13.8446528Z  10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
2020-04-07T04:05:13.8447532Z     │
2020-04-07T04:05:13.8447622Z 
2020-04-07T04:05:13.8447622Z 
2020-04-07T04:05:13.8448298Z error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
2020-04-07T04:05:13.8448880Z     ┌── crates-io.md:19:23 ───
2020-04-07T04:05:13.8449156Z     │
2020-04-07T04:05:13.8449156Z     │
2020-04-07T04:05:13.8449532Z  19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
2020-04-07T04:05:13.8450440Z     │
2020-04-07T04:05:13.8450531Z 
2020-04-07T04:05:13.8450960Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"
2020-04-07T04:05:13.8451209Z 
2020-04-07T04:05:13.8451209Z 
2020-04-07T04:05:13.8451513Z     ┌── diagnostics.md:81:63 ───
2020-04-07T04:05:13.8451779Z     │
2020-04-07T04:05:13.8452165Z  81 │ is passed) as JSON for consumption by tools, most notably the [Rust Language
2020-04-07T04:05:13.8452741Z     │                                                               ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8453114Z     │
2020-04-07T04:05:13.8453203Z 
2020-04-07T04:05:13.8453673Z error: The server responded with 429 Too Many Requests for "***fix"
2020-04-07T04:05:13.8454146Z     ┌── diagnostics.md:82:18 ───
2020-04-07T04:05:13.8454409Z     │
2020-04-07T04:05:13.8454409Z     │
2020-04-07T04:05:13.8454728Z  82 │ Server][rls] and [`rustfix`][rustfix].
2020-04-07T04:05:13.8455454Z     │
2020-04-07T04:05:13.8455559Z 
2020-04-07T04:05:13.8456034Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"
2020-04-07T04:05:13.8456314Z 
---
2020-04-07T04:05:13.8468877Z  57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
2020-04-07T04:05:13.8469327Z     │             ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8469623Z     │
2020-04-07T04:05:13.8469712Z 
2020-04-07T04:05:13.8470306Z error: The server responded with 429 Too Many Requests for "***/blob/master/triagebot.toml"
2020-04-07T04:05:13.8470848Z     ┌── ice-breaker/about.md:66:16 ───
2020-04-07T04:05:13.8471134Z     │
2020-04-07T04:05:13.8471134Z     │
2020-04-07T04:05:13.8471468Z  66 │ defined in the [`triagebot.toml`] file. For example:
2020-04-07T04:05:13.8472216Z     │
2020-04-07T04:05:13.8472305Z 
2020-04-07T04:05:13.8472305Z 
2020-04-07T04:05:13.8472817Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
2020-04-07T04:05:13.8473365Z    ┌── ice-breaker/cleanup-crew.md:3:19 ───
2020-04-07T04:05:13.8473641Z    │
2020-04-07T04:05:13.8473959Z  3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
2020-04-07T04:05:13.8474585Z    │                   ^ Server responded with 429 Too Many Requests
---
2020-04-07T04:05:13.8482340Z  80 │ To learn to use [cargo-bisect-rustc], check out [this blog
2020-04-07T04:05:13.8482809Z     │                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8483121Z     │
2020-04-07T04:05:13.8483212Z 
2020-04-07T04:05:13.8483691Z error: The server responded with 429 Too Many Requests for "***/"
2020-04-07T04:05:13.8484369Z      ┌── ice-breaker/cleanup-crew.md:102:36 ───
2020-04-07T04:05:13.8484659Z      │
2020-04-07T04:05:13.8485222Z  102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
2020-04-07T04:05:13.8486066Z      │                                    ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8486066Z      │                                    ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8486461Z      │
2020-04-07T04:05:13.8486578Z 
2020-04-07T04:05:13.8487123Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
2020-04-07T04:05:13.8487704Z    ┌── ice-breaker/llvm.md:3:19 ───
2020-04-07T04:05:13.8487989Z    │
2020-04-07T04:05:13.8488320Z  3 │ **Github Label:** [ICEBreaker-LLVM]
2020-04-07T04:05:13.8488946Z    │                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8488946Z    │                   ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8489574Z    │
2020-04-07T04:05:13.8489662Z 
2020-04-07T04:05:13.8490180Z error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-APACHE"
2020-04-07T04:05:13.8490849Z    ┌── licenses.md:3:78 ───
2020-04-07T04:05:13.8491286Z    │
2020-04-07T04:05:13.8491286Z    │
2020-04-07T04:05:13.8492435Z  3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
2020-04-07T04:05:13.8494062Z    │
2020-04-07T04:05:13.8494163Z 
2020-04-07T04:05:13.8494163Z 
2020-04-07T04:05:13.8495030Z error: The server responded with 429 Too Many Requests for "***/blob/master/LICENSE-MIT"
2020-04-07T04:05:13.8495541Z    ┌── licenses.md:3:170 ───
2020-04-07T04:05:13.8495822Z    │
2020-04-07T04:05:13.8495822Z    │
2020-04-07T04:05:13.8496907Z  3 │ The `rustc` compiler source and standard library are dual licensed under the [Apache License v2.0](***/blob/master/LICENSE-APACHE) and the [MIT License](***/blob/master/LICENSE-MIT) unless otherwise specified.
2020-04-07T04:05:13.8498776Z    │
2020-04-07T04:05:13.8498887Z 
2020-04-07T04:05:13.8498887Z 
2020-04-07T04:05:13.8499598Z error: The server responded with 429 Too Many Requests for "***/blob/master/COPYRIGHT"
2020-04-07T04:05:13.8525573Z    ┌── licenses.md:5:52 ───
2020-04-07T04:05:13.8525865Z    │
2020-04-07T04:05:13.8525865Z    │
2020-04-07T04:05:13.8526521Z  5 │ Detailed licensing information is available in the [COPYRIGHT document](***/blob/master/COPYRIGHT) of the `rust-lang/rust` repository.
2020-04-07T04:05:13.8528128Z    │
2020-04-07T04:05:13.8528217Z 
2020-04-07T04:05:13.8528217Z 
2020-04-07T04:05:13.8528722Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/issues"
2020-04-07T04:05:13.8529236Z    ┌── part-2-intro.md:8:25 ───
2020-04-07T04:05:13.8529524Z    │
2020-04-07T04:05:13.8529835Z  8 │   to file an issue on the [rustc-dev-guide
2020-04-07T04:05:13.8530189Z    │ ╭─────────────────────────^
2020-04-07T04:05:13.8530189Z    │ ╭─────────────────────────^
2020-04-07T04:05:13.8530830Z  9 │ │ repo](***c-dev-guide/issues) or contact the compiler
2020-04-07T04:05:13.8532480Z    │
2020-04-07T04:05:13.8532570Z 
2020-04-07T04:05:13.8532570Z 
2020-04-07T04:05:13.8533040Z error: The server responded with 429 Too Many Requests for "***c-dev-guide"
2020-04-07T04:05:13.8533712Z    ┌── overview.md:3:134 ───
2020-04-07T04:05:13.8534191Z    │
2020-04-07T04:05:13.8534191Z    │
2020-04-07T04:05:13.8535984Z  3 │ Coming soon!  Work is in progress on this chapter.  See ***c-dev-guide/pull/633 for the source and the [project README](***c-dev-guide) for local build instructions.
2020-04-07T04:05:13.8538108Z    │
2020-04-07T04:05:13.8538394Z 
2020-04-07T04:05:13.8538394Z 
2020-04-07T04:05:13.8539305Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
2020-04-07T04:05:13.8539846Z      ┌── query.md:155:1 ───
2020-04-07T04:05:13.8540115Z      │
2020-04-07T04:05:13.8540835Z  155 │ [`rustc_metadata` crate][rustc_metadata], which loads the information
2020-04-07T04:05:13.8541798Z      │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8541798Z      │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8542279Z      │
2020-04-07T04:05:13.8542372Z 
2020-04-07T04:05:13.8542945Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_middle/dep_graph"
2020-04-07T04:05:13.8543735Z     ┌── queries/incremental-compilation.md:84:1 ───
2020-04-07T04:05:13.8544420Z     │
2020-04-07T04:05:13.8544420Z     │
2020-04-07T04:05:13.8544953Z  84 │ [`src/librustc_middle/dep_graph`][dep_graph]. Construction of the DAG is done
2020-04-07T04:05:13.8545690Z     │
2020-04-07T04:05:13.8545807Z 
2020-04-07T04:05:13.8545807Z 
2020-04-07T04:05:13.8546285Z error: The server responded with 429 Too Many Requests for "***/issues/42678"
2020-04-07T04:05:13.8546828Z    ┌── queries/profiling.md:8:9 ───
2020-04-07T04:05:13.8547101Z    │
2020-04-07T04:05:13.8547101Z    │
2020-04-07T04:05:13.8547817Z  8 │ address [issue 42678](***/issues/42678).
2020-04-07T04:05:13.8548732Z    │
2020-04-07T04:05:13.8548820Z 
2020-04-07T04:05:13.8549457Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"
2020-04-07T04:05:13.8549825Z 
2020-04-07T04:05:13.8549825Z 
2020-04-07T04:05:13.8550137Z      ┌── queries/profiling.md:335:3 ───
2020-04-07T04:05:13.8550572Z      │
2020-04-07T04:05:13.8551178Z  335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
2020-04-07T04:05:13.8555529Z 
2020-04-07T04:05:13.8556270Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-dev-guide"
2020-04-07T04:05:13.8556826Z expected success, got: exit code: 101
2020-04-07T04:05:13.8556985Z 
2020-04-07T04:05:13.8556985Z 
2020-04-07T04:05:13.8557062Z 
2020-04-07T04:05:13.8557426Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T04:05:13.8557969Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-07T04:05:13.8558452Z ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8558866Z      │
2020-04-07T04:05:13.8558955Z 
2020-04-07T04:05:13.8559441Z error: The server responded with 429 Too Many Requests for "***/issues/42293"
2020-04-07T04:05:13.8560260Z      ┌── queries/profiling.md:337:3 ───
2020-04-07T04:05:13.8560890Z      │
2020-04-07T04:05:13.8560890Z      │
2020-04-07T04:05:13.8561355Z  337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
2020-04-07T04:05:13.8562635Z      │
2020-04-07T04:05:13.8562726Z 
2020-04-07T04:05:13.8562726Z 
2020-04-07T04:05:13.8563202Z error: The server responded with 429 Too Many Requests for "***/issues/42633"
2020-04-07T04:05:13.8563878Z      ┌── queries/profiling.md:341:3 ───
2020-04-07T04:05:13.8564575Z      │
2020-04-07T04:05:13.8564575Z      │
2020-04-07T04:05:13.8565191Z  341 │ - [GitHub issue #42633](***/issues/42633)
2020-04-07T04:05:13.8566289Z      │
2020-04-07T04:05:13.8566397Z 
2020-04-07T04:05:13.8567228Z error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"
2020-04-07T04:05:13.8567875Z 
2020-04-07T04:05:13.8567875Z 
2020-04-07T04:05:13.8568157Z    ┌── salsa.md:5:1 ───
2020-04-07T04:05:13.8568442Z    │
2020-04-07T04:05:13.8568780Z  5 │ [Salsa](https://github.com/salsa-rs/salsa).
2020-04-07T04:05:13.8570763Z    │
2020-04-07T04:05:13.8570883Z 
2020-04-07T04:05:13.8570883Z 
2020-04-07T04:05:13.8571627Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/blob/master/examples/rustc-driver-example.rs"
2020-04-07T04:05:13.8572364Z     ┌── rustc-driver.md:17:63 ───
2020-04-07T04:05:13.8572706Z     │
2020-04-07T04:05:13.8572706Z     │
2020-04-07T04:05:13.8573410Z  17 │ You can see a minimal example of how to use `rustc_interface` [here][example].
2020-04-07T04:05:13.8575332Z     │
2020-04-07T04:05:13.8575425Z 
2020-04-07T04:05:13.8575425Z 
2020-04-07T04:05:13.8576422Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
2020-04-07T04:05:13.8576871Z    ┌── rustdoc.md:6:50 ───
2020-04-07T04:05:13.8577121Z    │
2020-04-07T04:05:13.8577121Z    │
2020-04-07T04:05:13.8577469Z  6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
2020-04-07T04:05:13.8578292Z    │
2020-04-07T04:05:13.8578373Z 
2020-04-07T04:05:13.8578373Z 
2020-04-07T04:05:13.8579253Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
2020-04-07T04:05:13.8579777Z     ┌── rustdoc.md:26:22 ───
2020-04-07T04:05:13.8580035Z     │
2020-04-07T04:05:13.8581522Z  26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
2020-04-07T04:05:13.8582410Z     │                      ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8582410Z     │                      ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8582749Z     │
2020-04-07T04:05:13.8582845Z 
2020-04-07T04:05:13.8583913Z error: The server responded with 429 Too Many Requests for "***/issues/44136"
2020-04-07T04:05:13.8584624Z      ┌── rustdoc.md:115:1 ───
2020-04-07T04:05:13.8584924Z      │
2020-04-07T04:05:13.8584924Z      │
2020-04-07T04:05:13.8585340Z  115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
2020-04-07T04:05:13.8586123Z      │
2020-04-07T04:05:13.8586220Z 
2020-04-07T04:05:13.8586220Z 
2020-04-07T04:05:13.8587278Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_ast"
2020-04-07T04:05:13.8588533Z     ┌── test-implementation.md:35:1 ───
2020-04-07T04:05:13.8588836Z     │
2020-04-07T04:05:13.8589256Z  35 │ [`librustc_ast` crate][librustc_ast]. Essentially, it's a fancy macro, that
2020-04-07T04:05:13.8590205Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8590205Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8590882Z     │
2020-04-07T04:05:13.8591138Z 
2020-04-07T04:05:13.8591684Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
2020-04-07T04:05:13.8592213Z     ┌── macro-expansion.md:13:1 ───
2020-04-07T04:05:13.8592504Z     │
2020-04-07T04:05:13.8593075Z  13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2020-04-07T04:05:13.8593517Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8593517Z     │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8593991Z     │
2020-04-07T04:05:13.8594081Z 
2020-04-07T04:05:13.8594800Z error: The server responded with 429 Too Many Requests for "***/issues/41710"
2020-04-07T04:05:13.8595514Z     ┌── mir/passes.md:96:17 ───
2020-04-07T04:05:13.8595932Z     │
2020-04-07T04:05:13.8596237Z  96 │ alternatives in [rust-lang/rust#41710].
2020-04-07T04:05:13.8596677Z     │                 ^ Server responded with 429 Too Many Requests
---
2020-04-07T04:05:13.8602379Z  127 │ [`ProgramClause`]s generated by the rules module.
2020-04-07T04:05:13.8602933Z      │ ^ Server responded with 404 Not Found
2020-04-07T04:05:13.8603251Z      │
2020-04-07T04:05:13.8603347Z 
2020-04-07T04:05:13.8603850Z error: The server responded with 429 Too Many Requests for "***/pull/59089"
2020-04-07T04:05:13.8604403Z     ┌── backend/updating-llvm.md:63:1 ───
2020-04-07T04:05:13.8604694Z     │
2020-04-07T04:05:13.8604694Z     │
2020-04-07T04:05:13.8605397Z  63 │ [#59089](***/pull/59089)
2020-04-07T04:05:13.8606730Z     │
2020-04-07T04:05:13.8606990Z 
2020-04-07T04:05:13.8606990Z 
2020-04-07T04:05:13.8607666Z error: The server responded with 429 Too Many Requests for "***/pull/55835"
2020-04-07T04:05:13.8608214Z      ┌── backend/updating-llvm.md:128:1 ───
2020-04-07T04:05:13.8608535Z      │
2020-04-07T04:05:13.8608535Z      │
2020-04-07T04:05:13.8609062Z  128 │ [#55835](***/pull/55835)
2020-04-07T04:05:13.8610335Z      │
2020-04-07T04:05:13.8610426Z 
2020-04-07T04:05:13.8610426Z 
2020-04-07T04:05:13.8610906Z error: The server responded with 429 Too Many Requests for "***/pull/47828"
2020-04-07T04:05:13.8611414Z      ┌── backend/updating-llvm.md:129:1 ───
2020-04-07T04:05:13.8611692Z      │
2020-04-07T04:05:13.8611692Z      │
2020-04-07T04:05:13.8612070Z  129 │ [#47828](***/pull/47828)
2020-04-07T04:05:13.8612900Z      │
2020-04-07T04:05:13.8613007Z 
2020-04-07T04:05:13.8613007Z 
2020-04-07T04:05:13.8613469Z error: The server responded with 429 Too Many Requests for "***/pull/62474"
2020-04-07T04:05:13.8614001Z      ┌── backend/updating-llvm.md:130:1 ───
2020-04-07T04:05:13.8614285Z      │
2020-04-07T04:05:13.8614285Z      │
2020-04-07T04:05:13.8614644Z  130 │ [#62474](***/pull/62474)
2020-04-07T04:05:13.8615612Z      │
2020-04-07T04:05:13.8615702Z 
2020-04-07T04:05:13.8615702Z 
2020-04-07T04:05:13.8616186Z error: The server responded with 429 Too Many Requests for "***/pull/62592"
2020-04-07T04:05:13.8616877Z      ┌── backend/updating-llvm.md:131:1 ───
2020-04-07T04:05:13.8620630Z      │
2020-04-07T04:05:13.8620630Z      │
2020-04-07T04:05:13.8621200Z  131 │ [#62592](***/pull/62592). Note that sometimes it's
2020-04-07T04:05:13.8623230Z      │
2020-04-07T04:05:13.8623322Z 
2020-04-07T04:05:13.8623774Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"
2020-04-07T04:05:13.8624157Z 
---
2020-04-07T04:05:13.8629446Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"
2020-04-07T04:05:13.8629746Z 
2020-04-07T04:05:13.8630086Z      ┌── codegen/implicit-caller-location.md:246:1 ───
2020-04-07T04:05:13.8630592Z      │
2020-04-07T04:05:13.8631041Z  246 │ [non-viable alternatives] in the approved RFC for details). It took two more years until RFC 2091
2020-04-07T04:05:13.8632011Z      │
2020-04-07T04:05:13.8632107Z 
2020-04-07T04:05:13.8632658Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md"
2020-04-07T04:05:13.8632976Z 
2020-04-07T04:05:13.8632976Z 
2020-04-07T04:05:13.8633355Z      ┌── codegen/implicit-caller-location.md:247:27 ───
2020-04-07T04:05:13.8633675Z      │
2020-04-07T04:05:13.8634147Z  247 │ was approved, much of its [rationale] for this feature's design having been discovered through the
2020-04-07T04:05:13.8635061Z      │
2020-04-07T04:05:13.8635156Z 
2020-04-07T04:05:13.8635156Z 
2020-04-07T04:05:13.8635679Z error: The server responded with 429 Too Many Requests for "***/issues/47809"
2020-04-07T04:05:13.8636418Z      ┌── codegen/implicit-caller-location.md:252:59 ───
2020-04-07T04:05:13.8636896Z      │
2020-04-07T04:05:13.8636896Z      │
2020-04-07T04:05:13.8637333Z  252 │ approval of the RFC and the actual implementation work, a [revised design] was proposed and written
2020-04-07T04:05:13.8638299Z      │
2020-04-07T04:05:13.8638389Z 
2020-04-07T04:05:13.8638389Z 
2020-04-07T04:05:13.8639105Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-04-07T04:05:13.8639921Z     ┌── profile-guided-optimization.md:65:33 ───
2020-04-07T04:05:13.8640239Z     │
2020-04-07T04:05:13.8640239Z     │
2020-04-07T04:05:13.8640655Z  65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
2020-04-07T04:05:13.8641905Z     │
2020-04-07T04:05:13.8642004Z 
2020-04-07T04:05:13.8642004Z 
2020-04-07T04:05:13.8643016Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
2020-04-07T04:05:13.8643669Z     ┌── profile-guided-optimization.md:77:25 ───
2020-04-07T04:05:13.8644010Z     │
2020-04-07T04:05:13.8644010Z     │
2020-04-07T04:05:13.8644455Z  77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
2020-04-07T04:05:13.8645521Z     │
2020-04-07T04:05:13.8645776Z 
2020-04-07T04:05:13.8645776Z 
2020-04-07T04:05:13.8646353Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-04-07T04:05:13.8647174Z     ┌── profile-guided-optimization.md:88:11 ───
2020-04-07T04:05:13.8647657Z     │
2020-04-07T04:05:13.8647657Z     │
2020-04-07T04:05:13.8648065Z  88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
2020-04-07T04:05:13.8648845Z     │
2020-04-07T04:05:13.8648936Z 
2020-04-07T04:05:13.8649483Z error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"
2020-04-07T04:05:13.8649789Z 
2020-04-07T04:05:13.8649789Z 
2020-04-07T04:05:13.8650133Z      ┌── profile-guided-optimization.md:109:1 ───
2020-04-07T04:05:13.8650618Z      │
2020-04-07T04:05:13.8651162Z  109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
2020-04-07T04:05:13.8651582Z      │ ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8651873Z      │
2020-04-07T04:05:13.8651961Z 
2020-04-07T04:05:13.8652483Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
2020-04-07T04:05:13.8653057Z      ┌── profile-guided-optimization.md:122:4 ───
2020-04-07T04:05:13.8653355Z      │
2020-04-07T04:05:13.8653355Z      │
2020-04-07T04:05:13.8653734Z  122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
2020-04-07T04:05:13.8654464Z      │
2020-04-07T04:05:13.8654551Z 
2020-04-07T04:05:13.8654551Z 
2020-04-07T04:05:13.8655536Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
2020-04-07T04:05:13.8656116Z      ┌── profile-guided-optimization.md:123:17 ───
2020-04-07T04:05:13.8656424Z      │
2020-04-07T04:05:13.8656797Z  123 │ There is also a [codegen test][codegen-test] that checks that some expected
2020-04-07T04:05:13.8657437Z      │                 ^ Server responded with 429 Too Many Requests
---
2020-04-07T04:05:13.8659937Z  24 │ *  The sanitizer runtime libraries are part of the [compiler-rt] project, and
2020-04-07T04:05:13.8661852Z     │                                                    ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8662639Z     │
2020-04-07T04:05:13.8662743Z 
2020-04-07T04:05:13.8663458Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/native.rs"
2020-04-07T04:05:13.8664099Z     ┌── sanitizers.md:25:4 ───
2020-04-07T04:05:13.8664726Z     │
2020-04-07T04:05:13.8664726Z     │
2020-04-07T04:05:13.8665261Z  25 │    [will be built on supported targets][sanitizer-build] when enabled in `config.toml`:
2020-04-07T04:05:13.8669827Z     │
2020-04-07T04:05:13.8669937Z 
2020-04-07T04:05:13.8669937Z 
2020-04-07T04:05:13.8670953Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/bootstrap/compile.rs"
2020-04-07T04:05:13.8671550Z     ┌── sanitizers.md:32:21 ───
2020-04-07T04:05:13.8671813Z     │
2020-04-07T04:05:13.8672170Z  32 │    The runtimes are [placed into target libdir][sanitizer-copy].
2020-04-07T04:05:13.8672640Z     │                     ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8672640Z     │                     ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8672950Z     │
2020-04-07T04:05:13.8673038Z 
2020-04-07T04:05:13.8673688Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/attributes.rs"
2020-04-07T04:05:13.8674763Z     ┌── sanitizers.md:35:4 ───
2020-04-07T04:05:13.8675054Z     │
2020-04-07T04:05:13.8675054Z     │
2020-04-07T04:05:13.8675411Z  35 │    [marked][sanitizer-attribute] with appropriate LLVM attribute:
2020-04-07T04:05:13.8676135Z     │
2020-04-07T04:05:13.8676224Z 
2020-04-07T04:05:13.8676224Z 
2020-04-07T04:05:13.8676935Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_mir/transform/inline.rs"
2020-04-07T04:05:13.8677544Z     ┌── sanitizers.md:42:65 ───
2020-04-07T04:05:13.8677806Z     │
2020-04-07T04:05:13.8678193Z  42 │    functions it might be necessary to inhibit inlining, both at [MIR
2020-04-07T04:05:13.8678747Z     │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8678747Z     │                                                                 ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8679117Z     │
2020-04-07T04:05:13.8679205Z 
2020-04-07T04:05:13.8679857Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/blob/9330ec5a4c1df5fc1fa62f993ed6a04da68cb040/llvm/include/llvm/IR/Attributes.td"
2020-04-07T04:05:13.8680228Z 
2020-04-07T04:05:13.8680517Z     ┌── sanitizers.md:43:27 ───
2020-04-07T04:05:13.8680798Z     │
2020-04-07T04:05:13.8681135Z  43 │    level][inline-mir] and [LLVM level][inline-llvm].
2020-04-07T04:05:13.8681913Z     │
2020-04-07T04:05:13.8682002Z 
2020-04-07T04:05:13.8682002Z 
2020-04-07T04:05:13.8682632Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_llvm/back/write.rs"
2020-04-07T04:05:13.8683236Z     ┌── sanitizers.md:45:54 ───
2020-04-07T04:05:13.8683496Z     │
2020-04-07T04:05:13.8683496Z     │
2020-04-07T04:05:13.8683877Z  45 │ *  The LLVM IR generated by rustc is instrumented by [dedicated LLVM
2020-04-07T04:05:13.8684772Z     │
2020-04-07T04:05:13.8684862Z 
2020-04-07T04:05:13.8684862Z 
2020-04-07T04:05:13.8685510Z error: The server responded with 429 Too Many Requests for "***/blob/a29424a2265411dda7d7446516ac5fd7499e2b55/src/librustc_codegen_ssa/back/link.rs"
2020-04-07T04:05:13.8686278Z     ┌── sanitizers.md:50:4 ───
2020-04-07T04:05:13.8686542Z     │
2020-04-07T04:05:13.8686929Z  50 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
2020-04-07T04:05:13.8687381Z     │    ^ Server responded with 429 Too Many Requests
---
2020-04-07T04:05:13.8692306Z  45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
2020-04-07T04:05:13.8692790Z     │                               ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8693120Z     │
2020-04-07T04:05:13.8693207Z 
2020-04-07T04:05:13.8693685Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-07T04:05:13.8694223Z     ┌── debugging-support-in-rustc.md:80:1 ───
2020-04-07T04:05:13.8694505Z     │
2020-04-07T04:05:13.8694505Z     │
2020-04-07T04:05:13.8694960Z  80 │ [This comment by Tom](***c-dev-guide/pull/316#discussion_r284027340)
2020-04-07T04:05:13.8695998Z     │
2020-04-07T04:05:13.8696085Z 
2020-04-07T04:05:13.8696085Z 
2020-04-07T04:05:13.8696574Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-07T04:05:13.8697089Z     ┌── debugging-support-in-rustc.md:86:1 ───
2020-04-07T04:05:13.8697389Z     │
2020-04-07T04:05:13.8697389Z     │
2020-04-07T04:05:13.8697842Z  86 │ [This question by Aman](***c-dev-guide/pull/316#discussion_r285401353)
2020-04-07T04:05:13.8699054Z     │
2020-04-07T04:05:13.8699143Z 
2020-04-07T04:05:13.8699578Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"
2020-04-07T04:05:13.8699837Z 
---
2020-04-07T04:05:13.8702612Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"
2020-04-07T04:05:13.8702853Z 
2020-04-07T04:05:13.8703174Z      ┌── debugging-support-in-rustc.md:121:43 ───
2020-04-07T04:05:13.8703481Z      │
2020-04-07T04:05:13.8703895Z  121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
2020-04-07T04:05:13.8704768Z      │
2020-04-07T04:05:13.8704864Z 
2020-04-07T04:05:13.8704864Z 
2020-04-07T04:05:13.8705331Z error: The server responded with 429 Too Many Requests for "***/issues/34457"
2020-04-07T04:05:13.8705865Z      ┌── debugging-support-in-rustc.md:174:17 ───
2020-04-07T04:05:13.8706276Z      │
2020-04-07T04:05:13.8706276Z      │
2020-04-07T04:05:13.8706679Z  174 │ Tracking issue: [***/issues/34457]
2020-04-07T04:05:13.8707385Z      │
2020-04-07T04:05:13.8707473Z 
2020-04-07T04:05:13.8707473Z 
2020-04-07T04:05:13.8707945Z error: The server responded with 429 Too Many Requests for "***/issues/33014"
2020-04-07T04:05:13.8708458Z      ┌── debugging-support-in-rustc.md:229:18 ───
2020-04-07T04:05:13.8708764Z      │
2020-04-07T04:05:13.8708764Z      │
2020-04-07T04:05:13.8709142Z  229 │ Issue on Github: [***/issues/33014]
2020-04-07T04:05:13.8710041Z      │
2020-04-07T04:05:13.8710131Z 
2020-04-07T04:05:13.8710653Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"
2020-04-07T04:05:13.8710934Z 
2020-04-07T04:05:13.8710934Z 
2020-04-07T04:05:13.8711286Z      ┌── debugging-support-in-rustc.md:265:6 ───
2020-04-07T04:05:13.8711594Z      │
2020-04-07T04:05:13.8711935Z  265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
2020-04-07T04:05:13.8712368Z      │      ^ Server responded with 429 Too Many Requests
2020-04-07T04:05:13.8712660Z      │
2020-04-07T04:05:13.8712750Z 
2020-04-07T04:05:13.8713425Z error: The server responded with 429 Too Many Requests for "***c-dev-guide/pull/316"
2020-04-07T04:05:13.8713937Z      ┌── debugging-support-in-rustc.md:279:1 ───
2020-04-07T04:05:13.8714234Z      │
2020-04-07T04:05:13.8714234Z      │
2020-04-07T04:05:13.8714673Z  279 │ [Question on Github](***c-dev-guide/pull/316#discussion_r283062536).
2020-04-07T04:05:13.8715686Z      │
2020-04-07T04:05:13.8715770Z 
2020-04-07T04:05:13.8717453Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-04-07T04:05:14.1453909Z    Compiling proc-macro2 v0.4.30
---
2020-04-07T04:34:32.4254382Z    Compiling cargo v0.45.0 (/checkout/src/tools/cargo)
2020-04-07T04:34:52.9237093Z warning: unnecessary braces around block return value
2020-04-07T04:34:52.9238595Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-07T04:34:52.9239416Z    |
2020-04-07T04:34:52.9240222Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-07T04:34:52.9241852Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-07T04:34:52.9243391Z    = note: `#[warn(unused_braces)]` on by default
2020-04-07T04:34:52.9247168Z 
2020-04-07T04:42:01.5424278Z     Finished release [optimized] target(s) in 22m 32s
2020-04-07T04:42:01.8879507Z    Compiling slab v0.4.2
---
2020-04-07T04:42:44.2618317Z    Compiling tokio v0.1.22
2020-04-07T04:42:51.3961556Z warning: unnecessary braces around block return value
2020-04-07T04:42:51.3962233Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-07T04:42:51.3962669Z    |
2020-04-07T04:42:51.3964783Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-07T04:42:51.3966197Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-07T04:42:51.3968329Z    = note: `#[warn(unused_braces)]` on by default
2020-04-07T04:42:51.3968705Z 
2020-04-07T04:42:55.2213827Z warning: unnecessary braces around block return value
2020-04-07T04:42:55.2214551Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-07T04:42:55.2214551Z   --> src/tools/rls/rls/src/actions/progress.rs:36:55
2020-04-07T04:42:55.2215020Z    |
2020-04-07T04:42:55.2215716Z 36 |         static ref PROGRESS_ID_COUNTER: AtomicUsize = { AtomicUsize::new(0) };
2020-04-07T04:42:55.2216952Z    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
2020-04-07T04:42:55.2218184Z    = note: `#[warn(unused_braces)]` on by default
2020-04-07T04:42:55.2218459Z 
2020-04-07T04:48:06.2887615Z     Finished release [optimized] target(s) in 6m 04s
2020-04-07T04:48:06.3367745Z      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rls-9cb59d8154c102ac
---
2020-04-07T04:52:23.8871832Z test string::test::should_break_forward ... ok
2020-04-07T04:52:23.8872096Z test string::test::should_break_on_punctuation ... ok
2020-04-07T04:52:23.8872366Z test string::test::significant_whitespaces ... ok
2020-04-07T04:52:23.8872649Z test string::test::should_break_on_whitespace ... ok
2020-04-07T04:52:23.8872980Z test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
2020-04-07T04:52:23.8873365Z test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
2020-04-07T04:52:23.8873780Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
2020-04-07T04:52:23.8874194Z test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
2020-04-07T04:52:23.8914355Z test test::coverage_tests ... ok
2020-04-07T04:52:23.8914604Z test test::format_lines_errors_are_reported ... ok
2020-04-07T04:52:23.8928104Z test test::format_lines_errors_are_reported_with_tabs ... ok
2020-04-07T04:52:23.9245890Z test test::configuration_snippet::configuration_snippet_tests ... ok
---
2020-04-07T05:00:21.2952736Z test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T05:00:21.2953119Z 
2020-04-07T05:00:21.2956741Z      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-263e8d05fd14e725
2020-04-07T05:00:21.3111024Z ## Running run-pass tests in tests/run-pass against miri for target x86_64-unknown-linux-gnu
2020-04-07T05:00:21.3113437Z    Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
2020-04-07T05:00:21.3744864Z running 187 tests
2020-04-07T05:00:21.6395884Z test [ui] run-pass/args.rs ... ok
2020-04-07T05:00:22.0196172Z test [ui] run-pass/arrays.rs ... ok
2020-04-07T05:00:22.2150332Z test [ui] run-pass/associated-const.rs ... ok
---
2020-04-07T05:00:45.7192428Z 
2020-04-07T05:00:45.7192664Z test result: ok. 187 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T05:00:45.7192858Z 
2020-04-07T05:00:45.7253355Z ## Running compile-fail tests in tests/compile-fail against miri for target x86_64-unknown-linux-gnu
2020-04-07T05:00:45.7254177Z    Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
2020-04-07T05:00:45.7853427Z running 172 tests
2020-04-07T05:00:45.9194248Z test [compile-fail] compile-fail/alignment.rs ... ok
2020-04-07T05:00:45.9830157Z test [compile-fail] compile-fail/abort-terminator.rs ... ok
2020-04-07T05:00:46.0633403Z test [compile-fail] compile-fail/assume.rs ... ok
---
2020-04-07T05:00:58.5539751Z 
2020-04-07T05:00:58.5540163Z If you do intend to update 'rustc-dev-guide', please check the error messages above and
2020-04-07T05:00:58.5540669Z commit another update.
2020-04-07T05:00:58.5540811Z 
2020-04-07T05:00:58.5541276Z If you do NOT intend to update 'rustc-dev-guide', please ensure you did not accidentally
2020-04-07T05:00:58.5541976Z change the submodule at 'src/doc/rustc-dev-guide'. You may ask your reviewer for the
2020-04-07T05:00:58.5542234Z proper steps.
2020-04-07T05:00:58.5543955Z Build completed unsuccessfully in 0:00:01
2020-04-07T05:00:58.5590465Z == clock drift check ==
2020-04-07T05:00:58.5604298Z   local time: Tue Apr  7 05:00:58 UTC 2020
2020-04-07T05:00:58.5604298Z   local time: Tue Apr  7 05:00:58 UTC 2020
2020-04-07T05:00:58.8590544Z   network time: Tue, 07 Apr 2020 05:00:58 GMT
2020-04-07T05:00:59.1844396Z 
2020-04-07T05:00:59.1844396Z 
2020-04-07T05:00:59.1951644Z ##[error]Bash exited with code '1'.
2020-04-07T05:00:59.1973654Z ##[section]Finishing: Run build
2020-04-07T05:00:59.2030415Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-07T05:00:59.2035968Z Task         : Get sources
2020-04-07T05:00:59.2036385Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T05:00:59.2036875Z Version      : 1.0.0
2020-04-07T05:00:59.2037125Z Author       : Microsoft
2020-04-07T05:00:59.2037125Z Author       : Microsoft
2020-04-07T05:00:59.2037482Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T05:00:59.2037887Z ==============================================================================
2020-04-07T05:00:59.5727999Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T05:00:59.5790117Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70873/merge to s
2020-04-07T05:00:59.5883448Z Cleaning up task key
2020-04-07T05:00:59.5885067Z Start cleaning up orphan processes.
2020-04-07T05:00:59.6081709Z Terminate orphan process: pid (3472) (python)
2020-04-07T05:00:59.6310857Z ##[section]Finishing: Finalize Job
