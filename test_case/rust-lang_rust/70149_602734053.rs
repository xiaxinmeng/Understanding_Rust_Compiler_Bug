plain
2020-03-23T15:17:26.8262507Z ========================== Starting Command Output ===========================
2020-03-23T15:17:26.8264750Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/033644a5-f2cd-4fd1-89b4-42e659606802.sh
2020-03-23T15:17:26.8265005Z 
2020-03-23T15:17:26.8268449Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T15:17:26.8284350Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70149/merge to s
2020-03-23T15:17:26.8287000Z Task         : Get sources
2020-03-23T15:17:26.8287264Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T15:17:26.8287505Z Version      : 1.0.0
2020-03-23T15:17:26.8287665Z Author       : Microsoft
---
2020-03-23T15:17:27.8328058Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T15:17:27.8336005Z ##[command]git config gc.auto 0
2020-03-23T15:17:27.8340199Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T15:17:27.8345015Z ##[command]git config --get-all http.proxy
2020-03-23T15:17:27.8351323Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70149/merge:refs/remotes/pull/70149/merge
---
2020-03-23T16:50:24.7981006Z     Finished release [optimized] target(s) in 8m 35s
2020-03-23T16:50:24.8210025Z Testing rustbook src/doc/book
2020-03-23T16:50:39.3635849Z Error: Rustdoc returned an error: 
2020-03-23T16:50:39.3638703Z running 4 tests
2020-03-23T16:50:39.3640225Z test /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 15) ... ignored
2020-03-23T16:50:39.3641445Z test /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 35) ... ignored
2020-03-23T16:50:39.3642580Z test /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) ... FAILED
2020-03-23T16:50:39.3643679Z test /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 62) ... ok
2020-03-23T16:50:39.3644550Z failures:
2020-03-23T16:50:39.3645614Z 
2020-03-23T16:50:39.3645614Z 
2020-03-23T16:50:39.3647207Z ---- /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) stdout ----
2020-03-23T16:50:39.3648189Z error: Cannot declare a non-inline module inside a block unless it has a path attribute
2020-03-23T16:50:39.3649047Z  --> /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md:53:1
2020-03-23T16:50:39.3650060Z 3 | pub mod hosting;
2020-03-23T16:50:39.3718606Z   | ^^^^^^^^^^^^^^^^
2020-03-23T16:50:39.3719307Z 
2020-03-23T16:50:39.3720219Z error: aborting due to previous error
2020-03-23T16:50:39.3720219Z error: aborting due to previous error
2020-03-23T16:50:39.3720703Z 
2020-03-23T16:50:39.3721766Z Couldn't compile the test.
2020-03-23T16:50:39.3721912Z 
2020-03-23T16:50:39.3722044Z failures:
2020-03-23T16:50:39.3722709Z     /tmp/mdbook-G7cuXL/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52)
2020-03-23T16:50:39.3723302Z test result: FAILED. 1 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
2020-03-23T16:50:39.3723702Z 
2020-03-23T16:50:39.3723816Z 
2020-03-23T16:50:39.3723935Z 
---
2020-03-23T16:51:48.8463522Z  finished in 1.180
2020-03-23T16:51:48.8470195Z Testing rustbook src/doc/edition-guide
2020-03-23T16:51:56.8949229Z  finished in 8.047
2020-03-23T16:53:30.4442101Z Timeout for link `http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz`
2020-03-23T16:53:30.4491324Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-23T16:53:30.4492165Z     ┌── walkthrough.md:57:3 ───
2020-03-23T16:53:30.4492572Z     │
2020-03-23T16:53:30.4492572Z     │
2020-03-23T16:53:30.4493061Z  57 │   [propose to stabilize it][merge]. If there is consensus, this is done.
2020-03-23T16:53:30.4494272Z     │
2020-03-23T16:53:30.4494459Z 
2020-03-23T16:53:30.4495006Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-23T16:53:30.4495340Z 
2020-03-23T16:53:30.4495340Z 
2020-03-23T16:53:30.4495954Z     ┌── walkthrough.md:74:64 ───
2020-03-23T16:53:30.4496396Z     │
2020-03-23T16:53:30.4496973Z  74 │ > You can find the official guidelines for when to open an RFC [here][rfcwhen].
2020-03-23T16:53:30.4498295Z     │
2020-03-23T16:53:30.4498510Z 
2020-03-23T16:53:30.4499111Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-23T16:53:30.4499504Z 
2020-03-23T16:53:30.4499504Z 
2020-03-23T16:53:30.4499947Z     ┌── walkthrough.md:83:1 ───
2020-03-23T16:53:30.4500406Z     │
2020-03-23T16:53:30.4500987Z  83 │ [rust-lang/rfcs](https://github.com/rust-lang/rfcs) repo on GitHub. You can
2020-03-23T16:53:30.4502243Z     │
2020-03-23T16:53:30.4502470Z 
2020-03-23T16:53:30.4503069Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-23T16:53:30.4503447Z 
2020-03-23T16:53:30.4503447Z 
2020-03-23T16:53:30.4503902Z     ┌── walkthrough.md:85:1 ───
2020-03-23T16:53:30.4504328Z     │
2020-03-23T16:53:30.4504872Z  85 │ [README](https://github.com/rust-lang/rfcs#what-the-process-is).
2020-03-23T16:53:30.4506175Z     │
2020-03-23T16:53:30.4506384Z 
2020-03-23T16:53:30.4507023Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"
2020-03-23T16:53:30.4507614Z 
2020-03-23T16:53:30.4507614Z 
2020-03-23T16:53:30.4508079Z      ┌── walkthrough.md:107:51 ───
2020-03-23T16:53:30.4508533Z      │
2020-03-23T16:53:30.4509266Z  107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
2020-03-23T16:53:30.4510387Z      │
2020-03-23T16:53:30.4510574Z 
2020-03-23T16:53:30.4511135Z error: The server responded with 429 Too Many Requests for "***"
2020-03-23T16:53:30.4511478Z 
2020-03-23T16:53:30.4511478Z 
2020-03-23T16:53:30.4511884Z      ┌── walkthrough.md:146:26 ───
2020-03-23T16:53:30.4513523Z      │
2020-03-23T16:53:30.4514071Z  146 │ issue_ is created in the [rust-lang/rust] repo to track progress on the feature
2020-03-23T16:53:30.4514548Z      │                          ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4514994Z      │
2020-03-23T16:53:30.4515073Z 
2020-03-23T16:53:30.4515487Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-23T16:53:30.4515944Z      ┌── walkthrough.md:148:39 ───
2020-03-23T16:53:30.4516182Z      │
2020-03-23T16:53:30.4516182Z      │
2020-03-23T16:53:30.4516511Z  148 │ Here is the tracking issue on for our [`?` macro feature][tracking].
2020-03-23T16:53:30.4517249Z      │
2020-03-23T16:53:30.4517328Z 
2020-03-23T16:53:30.4517721Z error: The server responded with 429 Too Many Requests for "***"
2020-03-23T16:53:30.4517872Z 
2020-03-23T16:53:30.4517872Z 
2020-03-23T16:53:30.4518128Z      ┌── walkthrough.md:156:57 ───
2020-03-23T16:53:30.4518377Z      │
2020-03-23T16:53:30.4518718Z  156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
2020-03-23T16:53:30.4519191Z      │                                                         ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4519516Z      │
2020-03-23T16:53:30.4519593Z 
2020-03-23T16:53:30.4520003Z error: The server responded with 429 Too Many Requests for "***/pull/47732"
2020-03-23T16:53:30.4520549Z      ┌── walkthrough.md:167:58 ───
2020-03-23T16:53:30.4520779Z      │
2020-03-23T16:53:30.4521094Z  167 │ macro expansion in the compiler. Personally, I find that [improving the
2020-03-23T16:53:30.4521561Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4521561Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4521863Z      │
2020-03-23T16:53:30.4521938Z 
2020-03-23T16:53:30.4522320Z error: The server responded with 429 Too Many Requests for "***"
2020-03-23T16:53:30.4522468Z 
2020-03-23T16:53:30.4522717Z      ┌── walkthrough.md:181:27 ───
2020-03-23T16:53:30.4522958Z      │
2020-03-23T16:53:30.4523281Z  181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
2020-03-23T16:53:30.4523966Z      │
2020-03-23T16:53:30.4524050Z 
2020-03-23T16:53:30.4524050Z 
2020-03-23T16:53:30.4524443Z error: The server responded with 429 Too Many Requests for "***/issues/51934"
2020-03-23T16:53:30.4524874Z      ┌── walkthrough.md:237:32 ───
2020-03-23T16:53:30.4525102Z      │
2020-03-23T16:53:30.4525367Z  237 │   from the original RFC required [another
2020-03-23T16:53:30.4525703Z      │ ╭────────────────────────────────^
2020-03-23T16:53:30.4525703Z      │ ╭────────────────────────────────^
2020-03-23T16:53:30.4526053Z  238 │ │ FCP](***/issues/51934).
2020-03-23T16:53:30.4526792Z      │
2020-03-23T16:53:30.4526867Z 
2020-03-23T16:53:30.4526867Z 
2020-03-23T16:53:30.4527257Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-23T16:53:30.4527679Z      ┌── walkthrough.md:243:1 ───
2020-03-23T16:53:30.4527906Z      │
2020-03-23T16:53:30.4527906Z      │
2020-03-23T16:53:30.4528177Z  243 │ [moved to stabilize it][stabilizefcp].
2020-03-23T16:53:30.4528847Z      │
2020-03-23T16:53:30.4528924Z 
2020-03-23T16:53:30.4528924Z 
2020-03-23T16:53:30.4529392Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-23T16:53:30.4529813Z      ┌── walkthrough.md:253:45 ───
2020-03-23T16:53:30.4530057Z      │
2020-03-23T16:53:30.4530057Z      │
2020-03-23T16:53:30.4530356Z  253 │ The stabilization report for our feature is [here][stabrep].
2020-03-23T16:53:30.4531079Z      │
2020-03-23T16:53:30.4531154Z 
2020-03-23T16:53:30.4531154Z 
2020-03-23T16:53:30.4531541Z error: The server responded with 429 Too Many Requests for "***/pull/56245"
2020-03-23T16:53:30.4531966Z      ┌── walkthrough.md:257:13 ───
2020-03-23T16:53:30.4532195Z      │
2020-03-23T16:53:30.4532195Z      │
2020-03-23T16:53:30.4532532Z  257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
2020-03-23T16:53:30.4533198Z      │
2020-03-23T16:53:30.4533274Z 
2020-03-23T16:53:30.4533274Z 
2020-03-23T16:53:30.4533709Z error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
2020-03-23T16:53:30.4534134Z      ┌── walkthrough.md:258:55 ───
2020-03-23T16:53:30.4534379Z      │
2020-03-23T16:53:30.4534379Z      │
2020-03-23T16:53:30.4534709Z  258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
2020-03-23T16:53:30.4535482Z      │
2020-03-23T16:53:30.4535558Z 
2020-03-23T16:53:30.4536367Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"
2020-03-23T16:53:30.4536627Z 
---
2020-03-23T16:53:30.4545183Z  81 │ example of how such an issue should look can be [found
2020-03-23T16:53:30.4545894Z     │                                                 ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4546282Z     │
2020-03-23T16:53:30.4546382Z 
2020-03-23T16:53:30.4547191Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-23T16:53:30.4547987Z      ┌── bug-fix-procedure.md:235:65 ───
2020-03-23T16:53:30.4548325Z      │
2020-03-23T16:53:30.4548739Z  235 │ The first reference you will likely find is the lint definition [in
2020-03-23T16:53:30.4549359Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4549359Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4549793Z      │
2020-03-23T16:53:30.4549894Z 
2020-03-23T16:53:30.4550578Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-23T16:53:30.4551262Z      ┌── bug-fix-procedure.md:250:13 ───
2020-03-23T16:53:30.4551580Z      │
2020-03-23T16:53:30.4551580Z      │
2020-03-23T16:53:30.4552015Z  250 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
2020-03-23T16:53:30.4552868Z      │
2020-03-23T16:53:30.4552968Z 
2020-03-23T16:53:30.4552968Z 
2020-03-23T16:53:30.4577687Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
2020-03-23T16:53:30.4578753Z      ┌── bug-fix-procedure.md:254:19 ───
2020-03-23T16:53:30.4579126Z      │
2020-03-23T16:53:30.4579576Z  254 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
2020-03-23T16:53:30.4580144Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4580144Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4580521Z      │
2020-03-23T16:53:30.4580632Z 
2020-03-23T16:53:30.4581436Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
2020-03-23T16:53:30.4582186Z      ┌── bug-fix-procedure.md:283:16 ───
2020-03-23T16:53:30.4582548Z      │
2020-03-23T16:53:30.4582548Z      │
2020-03-23T16:53:30.4583004Z  283 │ this case, the [`add_lint` call][addlintsource] looks like this:
2020-03-23T16:53:30.4583937Z      │
2020-03-23T16:53:30.4584049Z 
2020-03-23T16:53:30.4584683Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"
2020-03-23T16:53:30.4585041Z 
2020-03-23T16:53:30.4585041Z 
2020-03-23T16:53:30.4585962Z     ┌── implementing_new_features.md:56:4 ───
2020-03-23T16:53:30.4586320Z     │
2020-03-23T16:53:30.4586767Z  56 │ We [value the stability of Rust]. Code that works and runs on stable
2020-03-23T16:53:30.4587294Z     │    ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4587642Z     │
2020-03-23T16:53:30.4587751Z 
2020-03-23T16:53:30.4588852Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-23T16:53:30.4589437Z     ┌── stability.md:18:51 ───
2020-03-23T16:53:30.4589778Z     │
2020-03-23T16:53:30.4590273Z  18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
2020-03-23T16:53:30.4590928Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4590928Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4591351Z     │
2020-03-23T16:53:30.4591474Z 
2020-03-23T16:53:30.4592045Z error: The server responded with 429 Too Many Requests for "***/issues/15702"
2020-03-23T16:53:30.4592639Z     ┌── stability.md:31:30 ───
2020-03-23T16:53:30.4592961Z     │
2020-03-23T16:53:30.4592961Z     │
2020-03-23T16:53:30.4593433Z  31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
2020-03-23T16:53:30.4594735Z     │
2020-03-23T16:53:30.4594844Z 
2020-03-23T16:53:30.4594844Z 
2020-03-23T16:53:30.4595486Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
2020-03-23T16:53:30.4596323Z     ┌── stabilization_guide.md:17:38 ───
2020-03-23T16:53:30.4596667Z     │
2020-03-23T16:53:30.4596667Z     │
2020-03-23T16:53:30.4597191Z  17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
2020-03-23T16:53:30.4598196Z     │
2020-03-23T16:53:30.4598321Z 
2020-03-23T16:53:30.4598857Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"
2020-03-23T16:53:30.4599153Z 
---
2020-03-23T16:53:30.4603271Z  28 │ - [The Book]: This may or may not need updating, depends.
2020-03-23T16:53:30.4603790Z     │   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4604137Z     │
2020-03-23T16:53:30.4604243Z 
2020-03-23T16:53:30.4604822Z error: The server responded with 429 Too Many Requests for "***-by-example"
2020-03-23T16:53:30.4605431Z     ┌── stabilization_guide.md:35:3 ───
2020-03-23T16:53:30.4605771Z     │
2020-03-23T16:53:30.4606150Z  35 │ - [Rust by Example]: As needed.
2020-03-23T16:53:30.4606609Z     │   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4606609Z     │   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4606954Z     │
2020-03-23T16:53:30.4607075Z 
2020-03-23T16:53:30.4607641Z error: The server responded with 429 Too Many Requests for "***/issues/32409"
2020-03-23T16:53:30.4608260Z     ┌── stabilization_guide.md:97:1 ───
2020-03-23T16:53:30.4608613Z     │
2020-03-23T16:53:30.4608949Z  97 │ [rust-lang/rust#32409]:
2020-03-23T16:53:30.4609378Z     │ ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4609378Z     │ ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4609744Z     │
2020-03-23T16:53:30.4609850Z 
2020-03-23T16:53:30.4610428Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc"
2020-03-23T16:53:30.4610764Z 
2020-03-23T16:53:30.4611143Z      ┌── compiler-debugging.md:258:5 ───
2020-03-23T16:53:30.4611489Z      │
2020-03-23T16:53:30.4611984Z  258 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
2020-03-23T16:53:30.4612867Z      │
2020-03-23T16:53:30.4612994Z 
2020-03-23T16:53:30.4613634Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc/blob/master/TUTORIAL.md"
2020-03-23T16:53:30.4614000Z 
2020-03-23T16:53:30.4614000Z 
2020-03-23T16:53:30.4614377Z      ┌── compiler-debugging.md:262:31 ───
2020-03-23T16:53:30.4614756Z      │
2020-03-23T16:53:30.4615238Z  262 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
2020-03-23T16:53:30.4616249Z      │
2020-03-23T16:53:30.4616360Z 
2020-03-23T16:53:30.4616946Z error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"
2020-03-23T16:53:30.4617293Z 
2020-03-23T16:53:30.4617293Z 
2020-03-23T16:53:30.4617670Z      ┌── compiler-debugging.md:270:5 ───
2020-03-23T16:53:30.4618016Z      │
2020-03-23T16:53:30.4618478Z  270 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
2020-03-23T16:53:30.4619379Z      │
2020-03-23T16:53:30.4619488Z 
2020-03-23T16:53:30.4620060Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf"
2020-03-23T16:53:30.4620449Z 
2020-03-23T16:53:30.4620449Z 
2020-03-23T16:53:30.4620798Z    ┌── profiling.md:8:9 ───
2020-03-23T16:53:30.4621114Z    │
2020-03-23T16:53:30.4621883Z  8 │   - The [rustc-perf](https://github.com/rust-lang-nursery/rustc-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
2020-03-23T16:53:30.4623198Z    │
2020-03-23T16:53:30.4623306Z 
2020-03-23T16:53:30.4623841Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"
2020-03-23T16:53:30.4624138Z 
2020-03-23T16:53:30.4624138Z 
2020-03-23T16:53:30.4624499Z     ┌── profiling.md:11:35 ───
2020-03-23T16:53:30.4624822Z     │
2020-03-23T16:53:30.4625469Z  11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
2020-03-23T16:53:30.4626872Z     │
2020-03-23T16:53:30.4626989Z 
2020-03-23T16:53:30.4627609Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"
2020-03-23T16:53:30.4627974Z 
---
2020-03-23T16:53:30.4727035Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf/tree/master/collector/benchmarks"
2020-03-23T16:53:30.4727443Z 
2020-03-23T16:53:30.4727862Z     ┌── profiling/with_perf.md:93:14 ───
2020-03-23T16:53:30.4728198Z     │
2020-03-23T16:53:30.4728671Z  93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
2020-03-23T16:53:30.4729977Z     │
2020-03-23T16:53:30.4730091Z 
2020-03-23T16:53:30.4733762Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-23T16:53:30.4734028Z 
2020-03-23T16:53:30.4734028Z 
2020-03-23T16:53:30.4734450Z      ┌── profiling/with_perf.md:137:45 ───
2020-03-23T16:53:30.4734754Z      │
2020-03-23T16:53:30.4735117Z  137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
2020-03-23T16:53:30.4735968Z      │
2020-03-23T16:53:30.4736058Z 
2020-03-23T16:53:30.4736492Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-23T16:53:30.4736904Z 
2020-03-23T16:53:30.4736904Z 
2020-03-23T16:53:30.4737296Z      ┌── profiling/with_perf.md:161:38 ───
2020-03-23T16:53:30.4737581Z      │
2020-03-23T16:53:30.4737947Z  161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
2020-03-23T16:53:30.4752773Z      │
2020-03-23T16:53:30.4752886Z 
2020-03-23T16:53:30.4753449Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/fmt-rfcs"
2020-03-23T16:53:30.4754376Z 
2020-03-23T16:53:30.4754376Z 
2020-03-23T16:53:30.4754761Z     ┌── conventions.md:10:36 ───
2020-03-23T16:53:30.4755065Z     │
2020-03-23T16:53:30.4755502Z  10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
2020-03-23T16:53:30.4756445Z     │
2020-03-23T16:53:30.4756561Z 
2020-03-23T16:53:30.4756561Z 
2020-03-23T16:53:30.4757287Z error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
2020-03-23T16:53:30.4757944Z     ┌── crates-io.md:19:23 ───
2020-03-23T16:53:30.4758441Z     │
2020-03-23T16:53:30.4758441Z     │
2020-03-23T16:53:30.4758757Z  19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
2020-03-23T16:53:30.4759424Z     │
2020-03-23T16:53:30.4759499Z 
2020-03-23T16:53:30.4759857Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"
2020-03-23T16:53:30.4760073Z 
---
2020-03-23T16:53:30.4762375Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustfix"
2020-03-23T16:53:30.4762589Z 
2020-03-23T16:53:30.4762834Z     ┌── diagnostics.md:82:18 ───
2020-03-23T16:53:30.4763413Z     │
2020-03-23T16:53:30.4763700Z  82 │ Server][rls] and [`rustfix`][rustfix].
2020-03-23T16:53:30.4767828Z     │
2020-03-23T16:53:30.4768108Z 
2020-03-23T16:53:30.4768654Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"
2020-03-23T16:53:30.4768955Z 
---
2020-03-23T16:53:30.4838192Z  57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
2020-03-23T16:53:30.4838615Z     │             ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4838908Z     │
2020-03-23T16:53:30.4838996Z 
2020-03-23T16:53:30.4839541Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
2020-03-23T16:53:30.4840065Z    ┌── ice-breaker/cleanup-crew.md:3:19 ───
2020-03-23T16:53:30.4840352Z    │
2020-03-23T16:53:30.4840761Z  3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
2020-03-23T16:53:30.4841120Z    │                   ^ Server responded with 429 Too Many Requests
---
2020-03-23T16:53:30.4851123Z  80 │ To learn to use [cargo-bisect-rustc], check out [this blog
2020-03-23T16:53:30.4851608Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4851967Z     │
2020-03-23T16:53:30.4852068Z 
2020-03-23T16:53:30.4852588Z error: The server responded with 429 Too Many Requests for "***/"
2020-03-23T16:53:30.4853164Z      ┌── ice-breaker/cleanup-crew.md:102:36 ───
2020-03-23T16:53:30.4853489Z      │
2020-03-23T16:53:30.4853891Z  102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
2020-03-23T16:53:30.4854450Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4854450Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4854842Z      │
2020-03-23T16:53:30.4854945Z 
2020-03-23T16:53:30.4855523Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
2020-03-23T16:53:30.4856085Z    ┌── ice-breaker/llvm.md:3:19 ───
2020-03-23T16:53:30.4856398Z    │
2020-03-23T16:53:30.4856741Z  3 │ **Github Label:** [ICEBreaker-LLVM]
2020-03-23T16:53:30.4857205Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4857205Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4857666Z    │
2020-03-23T16:53:30.4858046Z 
2020-03-23T16:53:30.4858517Z error: The server responded with 429 Too Many Requests for "***c-guide"
2020-03-23T16:53:30.4859004Z     ┌── part-2-intro.md:10:17 ───
2020-03-23T16:53:30.4859269Z     │
2020-03-23T16:53:30.4859269Z     │
2020-03-23T16:53:30.4859719Z  10 │ an issue on the [rustc-guide repo](***c-guide)
2020-03-23T16:53:30.4860644Z     │
2020-03-23T16:53:30.4860731Z 
2020-03-23T16:53:30.4860731Z 
2020-03-23T16:53:30.4861232Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
2020-03-23T16:53:30.4861718Z    ┌── rustdoc.md:6:50 ───
2020-03-23T16:53:30.4861984Z    │
2020-03-23T16:53:30.4861984Z    │
2020-03-23T16:53:30.4862355Z  6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
2020-03-23T16:53:30.4863220Z    │
2020-03-23T16:53:30.4863307Z 
2020-03-23T16:53:30.4863307Z 
2020-03-23T16:53:30.4863799Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
2020-03-23T16:53:30.4864302Z     ┌── rustdoc.md:26:22 ───
2020-03-23T16:53:30.4864562Z     │
2020-03-23T16:53:30.4864955Z  26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
2020-03-23T16:53:30.4865416Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4865416Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4865727Z     │
2020-03-23T16:53:30.4865813Z 
2020-03-23T16:53:30.4866292Z error: The server responded with 429 Too Many Requests for "***/issues/44136"
2020-03-23T16:53:30.4866760Z      ┌── rustdoc.md:115:1 ───
2020-03-23T16:53:30.4867037Z      │
2020-03-23T16:53:30.4867037Z      │
2020-03-23T16:53:30.4867419Z  115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
2020-03-23T16:53:30.4868446Z      │
2020-03-23T16:53:30.4868536Z 
2020-03-23T16:53:30.4868536Z 
2020-03-23T16:53:30.4869041Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
2020-03-23T16:53:30.4869546Z      ┌── query.md:150:1 ───
2020-03-23T16:53:30.4869802Z      │
2020-03-23T16:53:30.4870231Z  150 │ [`rustc_metadata` crate][rustc_metadata], which loads the information from the
2020-03-23T16:53:30.4870673Z      │ ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4870673Z      │ ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4871068Z      │
2020-03-23T16:53:30.4871157Z 
2020-03-23T16:53:30.4871722Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/dep_graph"
2020-03-23T16:53:30.4872547Z     ┌── queries/incremental-compilation.md:84:1 ───
2020-03-23T16:53:30.4873058Z     │
2020-03-23T16:53:30.4873058Z     │
2020-03-23T16:53:30.4874242Z  84 │ [`src/librustc/dep_graph`][dep_graph]. Construction of the DAG is done
2020-03-23T16:53:30.4875130Z     │
2020-03-23T16:53:30.4875239Z 
2020-03-23T16:53:30.4875239Z 
2020-03-23T16:53:30.4875815Z error: The server responded with 429 Too Many Requests for "***/issues/42678"
2020-03-23T16:53:30.4876425Z    ┌── queries/profiling.md:8:9 ───
2020-03-23T16:53:30.4876749Z    │
2020-03-23T16:53:30.4876749Z    │
2020-03-23T16:53:30.4877248Z  8 │ address [issue 42678](***/issues/42678).
2020-03-23T16:53:30.4878342Z    │
2020-03-23T16:53:30.4878450Z 
2020-03-23T16:53:30.4879230Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"
2020-03-23T16:53:30.4879667Z 
2020-03-23T16:53:30.4879667Z 
2020-03-23T16:53:30.4880036Z      ┌── queries/profiling.md:335:3 ───
2020-03-23T16:53:30.4880390Z      │
2020-03-23T16:53:30.4881124Z  335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
2020-03-23T16:53:30.4882873Z      │
2020-03-23T16:53:30.4882983Z 
2020-03-23T16:53:30.4882983Z 
2020-03-23T16:53:30.4883563Z error: The server responded with 429 Too Many Requests for "***/issues/42293"
2020-03-23T16:53:30.4884181Z      ┌── queries/profiling.md:337:3 ───
2020-03-23T16:53:30.4884523Z      │
2020-03-23T16:53:30.4884523Z      │
2020-03-23T16:53:30.4885097Z  337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
2020-03-23T16:53:30.4886428Z      │
2020-03-23T16:53:30.4886531Z 
2020-03-23T16:53:30.4886531Z 
2020-03-23T16:53:30.4887159Z error: The server responded with 429 Too Many Requests for "***/issues/42633"
2020-03-23T16:53:30.4887662Z      ┌── queries/profiling.md:341:3 ───
2020-03-23T16:53:30.4887935Z      │
2020-03-23T16:53:30.4887935Z      │
2020-03-23T16:53:30.4888340Z  341 │ - [GitHub issue #42633](***/issues/42633)
2020-03-23T16:53:30.4889492Z      │
2020-03-23T16:53:30.4889607Z 
2020-03-23T16:53:30.4890089Z error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"
2020-03-23T16:53:30.4890363Z 
2020-03-23T16:53:30.4890363Z 
2020-03-23T16:53:30.4890665Z    ┌── salsa.md:5:1 ───
2020-03-23T16:53:30.4890964Z    │
2020-03-23T16:53:30.4891325Z  5 │ [Salsa](https://github.com/salsa-rs/salsa).
2020-03-23T16:53:30.4892244Z    │
2020-03-23T16:53:30.4892342Z 
2020-03-23T16:53:30.4892879Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/index.html"
2020-03-23T16:53:30.4893178Z 
---
2020-03-23T16:53:30.4897121Z  49 │ - [`StringReader`] from [libsyntax] integrates `rustc_lexer` with `rustc`
2020-03-23T16:53:30.4897566Z     │                         ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4897864Z     │
2020-03-23T16:53:30.4897952Z 
2020-03-23T16:53:30.4898453Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-23T16:53:30.4899141Z     ┌── test-implementation.md:34:1 ───
2020-03-23T16:53:30.4899475Z     │
2020-03-23T16:53:30.4899888Z  34 │ [`libsyntax` crate][libsyntax]. Essentially, it's a fancy macro, that
2020-03-23T16:53:30.4900355Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-23T16:53:30.4905238Z  69 │ not stored as a string, but rather as an opaque [Symbol][Symbol] which is
2020-03-23T16:53:30.4905739Z     │                                                 ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4906076Z     │
2020-03-23T16:53:30.4906178Z 
2020-03-23T16:53:30.4906684Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
2020-03-23T16:53:30.4907215Z     ┌── macro-expansion.md:13:1 ───
2020-03-23T16:53:30.4907483Z     │
2020-03-23T16:53:30.4907862Z  13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2020-03-23T16:53:30.4908296Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-23T16:53:30.4955143Z  135 │ [`NodeId`]. This returns a `Option<Node<'tcx>>`, where [`Node`] is an enum
2020-03-23T16:53:30.4955609Z      │ ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4955934Z      │
2020-03-23T16:53:30.4956036Z 
2020-03-23T16:53:30.4956794Z error: The server responded with 429 Too Many Requests for "***/blob/3ee936378662bd2e74be951d6a7011a95a6bd84d/src/librustc/ty/mod.rs"
2020-03-23T16:53:30.4957339Z      ┌── ty.md:199:50 ───
2020-03-23T16:53:30.4957588Z      │
2020-03-23T16:53:30.4957588Z      │
2020-03-23T16:53:30.4958047Z  199 │ comparison of types for equality: we implemented [`PartialEq for TyS`][peqimpl], so we can just
2020-03-23T16:53:30.4958962Z      │
2020-03-23T16:53:30.4959048Z 
2020-03-23T16:53:30.4959048Z 
2020-03-23T16:53:30.4959662Z error: The server responded with 429 Too Many Requests for "***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs"
2020-03-23T16:53:30.4960218Z      ┌── ty.md:468:1 ───
2020-03-23T16:53:30.4960486Z      │
2020-03-23T16:53:30.4960486Z      │
2020-03-23T16:53:30.4960912Z  468 │ [Here is an example of actually using `subst` in the compiler][substex].  The exact details are not
2020-03-23T16:53:30.4961658Z      │
2020-03-23T16:53:30.4961746Z 
2020-03-23T16:53:30.4961746Z 
2020-03-23T16:53:30.4962288Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_macros/src/type_foldable.rs"
2020-03-23T16:53:30.4962812Z      ┌── ty.md:573:1 ───
2020-03-23T16:53:30.4963064Z      │
2020-03-23T16:53:30.4963064Z      │
2020-03-23T16:53:30.4963517Z  573 │ [here](***/blob/master/src/librustc_macros/src/type_foldable.rs).
2020-03-23T16:53:30.4964521Z      │
2020-03-23T16:53:30.4964625Z 
2020-03-23T16:53:30.4964625Z 
2020-03-23T16:53:30.4965222Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-23T16:53:30.4965777Z      ┌── ty.md:575:46 ───
2020-03-23T16:53:30.4966125Z      │
2020-03-23T16:53:30.4966125Z      │
2020-03-23T16:53:30.4967831Z  575 │   **`subst`** In the case of substitutions the [actual
2020-03-23T16:53:30.4968298Z      │ ╭──────────────────────────────────────────────^
2020-03-23T16:53:30.4968819Z  576 │ │ folder](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L467-L482)
2020-03-23T16:53:30.4969886Z      │
2020-03-23T16:53:30.4969963Z 
2020-03-23T16:53:30.4969963Z 
2020-03-23T16:53:30.4970485Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-23T16:53:30.4971120Z      ┌── ty.md:579:1 ───
2020-03-23T16:53:30.4971359Z      │
2020-03-23T16:53:30.4971359Z      │
2020-03-23T16:53:30.4971808Z  579 │ [fold_ty](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L545-L573)
2020-03-23T16:53:30.4972799Z      │
2020-03-23T16:53:30.4972874Z 
2020-03-23T16:53:30.4972874Z 
2020-03-23T16:53:30.4973391Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-23T16:53:30.4973851Z      ┌── ty.md:583:1 ───
2020-03-23T16:53:30.4974064Z      │
2020-03-23T16:53:30.4974064Z      │
2020-03-23T16:53:30.4974532Z  583 │ [ty_for_param](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L589-L624)
2020-03-23T16:53:30.4975715Z      │
2020-03-23T16:53:30.4975790Z 
2020-03-23T16:53:30.4975790Z 
2020-03-23T16:53:30.4976260Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/infer/higher_ranked/README.md"
2020-03-23T16:53:30.4976731Z     ┌── traits/hrtb.md:35:62 ───
2020-03-23T16:53:30.4976952Z     │
2020-03-23T16:53:30.4977280Z  35 │ to the subtyping for higher-ranked types (which is described [here][hrsubtype]
2020-03-23T16:53:30.4977765Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4977765Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.4978068Z     │
2020-03-23T16:53:30.4978142Z 
2020-03-23T16:53:30.4978553Z error: The server responded with 429 Too Many Requests for "***/issues/22019"
2020-03-23T16:53:30.4978972Z     ┌── traits/caching.md:57:30 ───
2020-03-23T16:53:30.4979212Z     │
2020-03-23T16:53:30.4979212Z     │
2020-03-23T16:53:30.4979539Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-23T16:53:30.4980491Z     │
2020-03-23T16:53:30.4980578Z 
2020-03-23T16:53:30.4980578Z 
2020-03-23T16:53:30.4981032Z error: The server responded with 429 Too Many Requests for "***/issues/18290"
2020-03-23T16:53:30.4981521Z     ┌── traits/caching.md:57:43 ───
2020-03-23T16:53:30.4981781Z     │
2020-03-23T16:53:30.4981781Z     │
2020-03-23T16:53:30.4982153Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-23T16:53:30.4982993Z     │
2020-03-23T16:53:30.4983080Z 
2020-03-23T16:53:30.4983080Z 
2020-03-23T16:53:30.4983549Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-03-23T16:53:30.4984084Z    ┌── traits/index.md:4:3 ───
2020-03-23T16:53:30.4984352Z    │
2020-03-23T16:53:30.4984352Z    │
2020-03-23T16:53:30.4984758Z  4 │ > [process of being implemented][wg]; this chapter serves as a kind of
2020-03-23T16:53:30.4985469Z    │
2020-03-23T16:53:30.4985556Z 
2020-03-23T16:53:30.4985556Z 
2020-03-23T16:53:30.4986037Z error: The server responded with 404 Not Found for "***/tree/master/src/test/ui/chalkify"
2020-03-23T16:53:30.4986568Z     ┌── traits/lowering-module.md:25:27 ───
2020-03-23T16:53:30.4986843Z     │
2020-03-23T16:53:30.4987201Z  25 │ Unit tests are located in [`src/test/ui/chalkify`][chalkify]. A good
2020-03-23T16:53:30.4987657Z     │                           ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4987657Z     │                           ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4987954Z     │
2020-03-23T16:53:30.4988041Z 
2020-03-23T16:53:30.4988559Z error: The server responded with 404 Not Found for "***/tree/master/src/test/ui/chalkify/lower_impl.rs"
2020-03-23T16:53:30.4989102Z     ┌── traits/lowering-module.md:26:17 ───
2020-03-23T16:53:30.4989398Z     │
2020-03-23T16:53:30.4989757Z  26 │ example test is [the `lower_impl` test][lower_impl]. At the time of
2020-03-23T16:53:30.4990176Z     │                 ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4990176Z     │                 ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4990475Z     │
2020-03-23T16:53:30.4990561Z 
2020-03-23T16:53:30.4991080Z error: The server responded with 404 Not Found for "***/tree/master/src/test/ui/chalkify/lower_impl.stderr"
2020-03-23T16:53:30.4991631Z     ┌── traits/lowering-module.md:49:38 ───
2020-03-23T16:53:30.4991904Z     │
2020-03-23T16:53:30.4992250Z  49 │ `#[rustc_dump_program_clauses]`, but [the stderr file] contains
2020-03-23T16:53:30.4992721Z     │                                      ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4992721Z     │                                      ^ Server responded with 404 Not Found
2020-03-23T16:53:30.4993032Z     │
2020-03-23T16:53:30.4993118Z 
2020-03-23T16:53:30.4993714Z error: The server responded with 429 Too Many Requests for "***/pull/47828"
2020-03-23T16:53:30.4994329Z      ┌── codegen/updating-llvm.md:129:1 ───
2020-03-23T16:53:30.4994628Z      │
2020-03-23T16:53:30.4994628Z      │
2020-03-23T16:53:30.4995096Z  129 │ [#47828](***/pull/47828)
2020-03-23T16:53:30.4995806Z      │
2020-03-23T16:53:30.4995883Z 
2020-03-23T16:53:30.4995883Z 
2020-03-23T16:53:30.4996270Z error: The server responded with 429 Too Many Requests for "***/pull/62474"
2020-03-23T16:53:30.4996710Z      ┌── codegen/updating-llvm.md:130:1 ───
2020-03-23T16:53:30.4996948Z      │
2020-03-23T16:53:30.4996948Z      │
2020-03-23T16:53:30.4997265Z  130 │ [#62474](***/pull/62474)
2020-03-23T16:53:30.4997959Z      │
2020-03-23T16:53:30.4998034Z 
2020-03-23T16:53:30.4998034Z 
2020-03-23T16:53:30.4998438Z error: The server responded with 429 Too Many Requests for "***/pull/62592"
2020-03-23T16:53:30.4998871Z      ┌── codegen/updating-llvm.md:131:1 ───
2020-03-23T16:53:30.4999131Z      │
2020-03-23T16:53:30.4999131Z      │
2020-03-23T16:53:30.4999483Z  131 │ [#62592](***/pull/62592). Note that sometimes it's
2020-03-23T16:53:30.5000457Z      │
2020-03-23T16:53:30.5000544Z 
2020-03-23T16:53:30.5001181Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"
2020-03-23T16:53:30.5001477Z 
---
2020-03-23T16:53:30.5003955Z error: The server responded with 429 Too Many Requests for "https://github.com/CraneStation/cranelift"
2020-03-23T16:53:30.5004233Z 
2020-03-23T16:53:30.5004641Z    ┌── codegen/backend-agnostic.md:4:1 ───
2020-03-23T16:53:30.5004980Z    │
2020-03-23T16:53:30.5005400Z  4 │ [Cranelift][cranelift]). To this end, `librustc_codegen_ssa` provides an
2020-03-23T16:53:30.5006195Z    │
2020-03-23T16:53:30.5006294Z 
2020-03-23T16:53:30.5006294Z 
2020-03-23T16:53:30.5006888Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-23T16:53:30.5007535Z     ┌── profile-guided-optimization.md:65:33 ───
2020-03-23T16:53:30.5007865Z     │
2020-03-23T16:53:30.5007865Z     │
2020-03-23T16:53:30.5008303Z  65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
2020-03-23T16:53:30.5009253Z     │
2020-03-23T16:53:30.5009351Z 
2020-03-23T16:53:30.5009351Z 
2020-03-23T16:53:30.5010008Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
2020-03-23T16:53:30.5010670Z     ┌── profile-guided-optimization.md:77:25 ───
2020-03-23T16:53:30.5011016Z     │
2020-03-23T16:53:30.5011016Z     │
2020-03-23T16:53:30.5011469Z  77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
2020-03-23T16:53:30.5012436Z     │
2020-03-23T16:53:30.5012522Z 
2020-03-23T16:53:30.5012522Z 
2020-03-23T16:53:30.5013035Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-23T16:53:30.5013709Z     ┌── profile-guided-optimization.md:88:11 ───
2020-03-23T16:53:30.5013955Z     │
2020-03-23T16:53:30.5013955Z     │
2020-03-23T16:53:30.5014272Z  88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
2020-03-23T16:53:30.5014917Z     │
2020-03-23T16:53:30.5014998Z 
2020-03-23T16:53:30.5015443Z error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"
2020-03-23T16:53:30.5015689Z 
2020-03-23T16:53:30.5015689Z 
2020-03-23T16:53:30.5015968Z      ┌── profile-guided-optimization.md:109:1 ───
2020-03-23T16:53:30.5016220Z      │
2020-03-23T16:53:30.5016564Z  109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
2020-03-23T16:53:30.5016926Z      │ ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5017165Z      │
2020-03-23T16:53:30.5017255Z 
2020-03-23T16:53:30.5017695Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
2020-03-23T16:53:30.5018183Z      ┌── profile-guided-optimization.md:122:4 ───
2020-03-23T16:53:30.5018435Z      │
2020-03-23T16:53:30.5018435Z      │
2020-03-23T16:53:30.5018772Z  122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
2020-03-23T16:53:30.5019406Z      │
2020-03-23T16:53:30.5019481Z 
2020-03-23T16:53:30.5019481Z 
2020-03-23T16:53:30.5020168Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
2020-03-23T16:53:30.5020739Z      ┌── profile-guided-optimization.md:123:17 ───
2020-03-23T16:53:30.5021046Z      │
2020-03-23T16:53:30.5021422Z  123 │ There is also a [codegen test][codegen-test] that checks that some expected
2020-03-23T16:53:30.5021872Z      │                 ^ Server responded with 429 Too Many Requests
---
2020-03-23T16:53:30.5024048Z  24 │ 1. The sanitizer runtime libraries are part of the [compiler-rt] project, and
2020-03-23T16:53:30.5024578Z     │                                                    ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5024935Z     │
2020-03-23T16:53:30.5025022Z 
2020-03-23T16:53:30.5025622Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/native.rs"
2020-03-23T16:53:30.5026167Z     ┌── sanitizers.md:25:4 ───
2020-03-23T16:53:30.5026425Z     │
2020-03-23T16:53:30.5026425Z     │
2020-03-23T16:53:30.5026808Z  25 │    [will be built as an LLVM subproject][sanitizer-build] when enabled in
2020-03-23T16:53:30.5027501Z     │
2020-03-23T16:53:30.5027602Z 
2020-03-23T16:53:30.5027602Z 
2020-03-23T16:53:30.5028186Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/compile.rs"
2020-03-23T16:53:30.5029402Z     ┌── sanitizers.md:33:21 ───
2020-03-23T16:53:30.5029723Z     │
2020-03-23T16:53:30.5033119Z  33 │    The runtimes are [placed into target libdir][sanitizer-copy].
2020-03-23T16:53:30.5036337Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5036337Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5036733Z     │
2020-03-23T16:53:30.5036843Z 
2020-03-23T16:53:30.5037556Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_llvm/declare.rs"
2020-03-23T16:53:30.5038205Z     ┌── sanitizers.md:36:4 ───
2020-03-23T16:53:30.5038527Z     │
2020-03-23T16:53:30.5038527Z     │
2020-03-23T16:53:30.5039011Z  36 │    [marked][sanitizer-attribute] with `SanitizeAddress`, `SanitizeMemory`, or
2020-03-23T16:53:30.5039883Z     │
2020-03-23T16:53:30.5040006Z 
2020-03-23T16:53:30.5040006Z 
2020-03-23T16:53:30.5040700Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_ssa/back/write.rs"
2020-03-23T16:53:30.5041379Z     ┌── sanitizers.md:41:54 ───
2020-03-23T16:53:30.5041701Z     │
2020-03-23T16:53:30.5042146Z  41 │ 3. The LLVM IR generated by rustc is instrumented by [dedicated LLVM
2020-03-23T16:53:30.5042799Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5042799Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5043226Z     │
2020-03-23T16:53:30.5043334Z 
2020-03-23T16:53:30.5044111Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/librustc_codegen_ssa/back/link.rs"
2020-03-23T16:53:30.5044810Z     ┌── sanitizers.md:46:4 ───
2020-03-23T16:53:30.5045148Z     │
2020-03-23T16:53:30.5045730Z  46 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
2020-03-23T16:53:30.5046218Z     │    ^ Server responded with 429 Too Many Requests
---
2020-03-23T16:53:30.5051497Z  45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
2020-03-23T16:53:30.5052119Z     │                               ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5052709Z     │
2020-03-23T16:53:30.5052811Z 
2020-03-23T16:53:30.5053375Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-23T16:53:30.5053961Z     ┌── debugging-support-in-rustc.md:80:1 ───
2020-03-23T16:53:30.5054308Z     │
2020-03-23T16:53:30.5054308Z     │
2020-03-23T16:53:30.5054813Z  80 │ [This comment by Tom](***c-guide/pull/316#discussion_r284027340)
2020-03-23T16:53:30.5055966Z     │
2020-03-23T16:53:30.5056065Z 
2020-03-23T16:53:30.5056065Z 
2020-03-23T16:53:30.5056613Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-23T16:53:30.5057201Z     ┌── debugging-support-in-rustc.md:86:1 ───
2020-03-23T16:53:30.5057540Z     │
2020-03-23T16:53:30.5057540Z     │
2020-03-23T16:53:30.5058069Z  86 │ [This question by Aman](***c-guide/pull/316#discussion_r285401353)
2020-03-23T16:53:30.5059305Z     │
2020-03-23T16:53:30.5059405Z 
2020-03-23T16:53:30.5059885Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"
2020-03-23T16:53:30.5060151Z 
---
2020-03-23T16:53:30.5062809Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"
2020-03-23T16:53:30.5063109Z 
2020-03-23T16:53:30.5063484Z      ┌── debugging-support-in-rustc.md:121:43 ───
2020-03-23T16:53:30.5063821Z      │
2020-03-23T16:53:30.5064320Z  121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
2020-03-23T16:53:30.5065348Z      │
2020-03-23T16:53:30.5065451Z 
2020-03-23T16:53:30.5065451Z 
2020-03-23T16:53:30.5065910Z error: The server responded with 429 Too Many Requests for "***/issues/34457"
2020-03-23T16:53:30.5066440Z      ┌── debugging-support-in-rustc.md:174:17 ───
2020-03-23T16:53:30.5066733Z      │
2020-03-23T16:53:30.5066733Z      │
2020-03-23T16:53:30.5067112Z  174 │ Tracking issue: [***/issues/34457]
2020-03-23T16:53:30.5067836Z      │
2020-03-23T16:53:30.5067924Z 
2020-03-23T16:53:30.5067924Z 
2020-03-23T16:53:30.5068393Z error: The server responded with 429 Too Many Requests for "***/issues/33014"
2020-03-23T16:53:30.5068922Z      ┌── debugging-support-in-rustc.md:229:18 ───
2020-03-23T16:53:30.5069220Z      │
2020-03-23T16:53:30.5069220Z      │
2020-03-23T16:53:30.5069619Z  229 │ Issue on Github: [***/issues/33014]
2020-03-23T16:53:30.5070347Z      │
2020-03-23T16:53:30.5070436Z 
2020-03-23T16:53:30.5070874Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"
2020-03-23T16:53:30.5071121Z 
2020-03-23T16:53:30.5071121Z 
2020-03-23T16:53:30.5071458Z      ┌── debugging-support-in-rustc.md:265:6 ───
2020-03-23T16:53:30.5071752Z      │
2020-03-23T16:53:30.5072202Z  265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
2020-03-23T16:53:30.5072551Z      │      ^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5072812Z      │
2020-03-23T16:53:30.5072888Z 
2020-03-23T16:53:30.5073286Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-23T16:53:30.5074294Z      ┌── debugging-support-in-rustc.md:279:1 ───
2020-03-23T16:53:30.5095857Z      │
2020-03-23T16:53:30.5095857Z      │
2020-03-23T16:53:30.5096399Z  279 │ [Question on Github](***c-guide/pull/316#discussion_r283062536).
2020-03-23T16:53:30.5097272Z      │
2020-03-23T16:53:30.5097367Z 
2020-03-23T16:53:30.5097738Z error: The server responded with 429 Too Many Requests for "https://github.com/nrc/stupid-stats"
2020-03-23T16:53:30.5097942Z 
2020-03-23T16:53:30.5097942Z 
2020-03-23T16:53:30.5098221Z    ┌── appendix/stupid-stats.md:3:48 ───
2020-03-23T16:53:30.5098453Z    │
2020-03-23T16:53:30.5099835Z  3 │ > **Note:** This is a copy of `@nrc`'s amazing [stupid-stats]. You should find
2020-03-23T16:53:30.5100910Z    │
2020-03-23T16:53:30.5100996Z 
2020-03-23T16:53:30.5100996Z 
2020-03-23T16:53:30.5101795Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-23T16:53:30.5102422Z     ┌── appendix/stupid-stats.md:64:43 ───
2020-03-23T16:53:30.5102738Z     │
2020-03-23T16:53:30.5102738Z     │
2020-03-23T16:53:30.5103511Z  64 │ The code for these first two phases is in [libsyntax](***/tree/master/src/libsyntax).
2020-03-23T16:53:30.5104882Z     │
2020-03-23T16:53:30.5104992Z 
2020-03-23T16:53:30.5104992Z 
2020-03-23T16:53:30.5105591Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc"
2020-03-23T16:53:30.5106243Z     ┌── appendix/stupid-stats.md:76:25 ───
2020-03-23T16:53:30.5106581Z     │
2020-03-23T16:53:30.5106581Z     │
2020-03-23T16:53:30.5107146Z  76 │ The analysis code is in [librustc](***/tree/master/src/librustc)
2020-03-23T16:53:30.5108397Z     │
2020-03-23T16:53:30.5108503Z 
2020-03-23T16:53:30.5108503Z 
2020-03-23T16:53:30.5109133Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_llvm"
2020-03-23T16:53:30.5109768Z     ┌── appendix/stupid-stats.md:87:40 ───
2020-03-23T16:53:30.5110127Z     │
2020-03-23T16:53:30.5110127Z     │
2020-03-23T16:53:30.5110751Z  87 │ interface between LLVM and rustc is in [librustc_llvm](***/tree/master/src/librustc_llvm).
2020-03-23T16:53:30.5112238Z     │
2020-03-23T16:53:30.5112341Z 
2020-03-23T16:53:30.5112922Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_utils/index.html"
2020-03-23T16:53:30.5113249Z 
---
2020-03-23T16:53:30.5120943Z  205 │ [multirust](https://github.com/brson/multirust) to get around all the PATH stuff
2020-03-23T16:53:30.5121471Z      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-03-23T16:53:30.5121810Z      │
2020-03-23T16:53:30.5121917Z 
2020-03-23T16:53:30.5122368Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-23T16:53:30.5122879Z      ┌── appendix/stupid-stats.md:402:33 ───
2020-03-23T16:53:30.5123157Z      │
2020-03-23T16:53:30.5123157Z      │
2020-03-23T16:53:30.5123602Z  402 │ do, let me know in a comment or [GitHub issue](***/issues).
2020-03-23T16:53:30.5124592Z      │
2020-03-23T16:53:30.5124680Z 
2020-03-23T16:53:30.5125185Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html"
2020-03-23T16:53:30.5125588Z 
2020-03-23T16:53:30.5125588Z 
2020-03-23T16:53:30.5125844Z     ┌── appendix/code-index.md:11:90 ───
2020-03-23T16:53:30.5126091Z     │
2020-03-23T16:53:30.5126627Z  11 │ `ast::Crate` | struct | A syntax-level representation of a parsed crate | [The parser] | [src/libsyntax/ast.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html)
2020-03-23T16:53:30.5127447Z     │                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 404 Not Found
2020-03-23T16:53:30.5127908Z     │
2020-03-23T16:53:30.5127984Z 
2020-03-23T16:53:30.5128431Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustdoc/core.rs"
2020-03-23T16:53:30.5128879Z     ┌── appendix/code-index.md:15:131 ───
2020-03-23T16:53:30.5129126Z     │
2020-03-23T16:53:30.5129126Z     │
2020-03-23T16:53:30.5129727Z  15 │ `DocContext` | struct | A state container used by rustdoc when crawling through a crate to gather its documentation | [Rustdoc] | [src/librustdoc/core.rs](***/blob/master/src/librustdoc/core.rs)
2020-03-23T16:53:30.5131192Z     │
2020-03-23T16:53:30.5131268Z 
2020-03-23T16:53:30.5131748Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/node_id/struct.NodeId.html"
2020-03-23T16:53:30.5131996Z 
---
2020-03-23T16:56:51.9864354Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2020-03-23T16:56:56.9434982Z error[E0061]: this function takes 0 arguments but 2 arguments were supplied
2020-03-23T16:56:56.9437071Z    --> src/tools/clippy/clippy_lints/src/doc.rs:238:55
2020-03-23T16:56:56.9437863Z     |
2020-03-23T16:56:56.9438760Z 238 |                 if match_type(cx, subs.as_generator().return_ty(def_id, cx.tcx), &paths::RESULT);
2020-03-23T16:56:56.9441127Z     |                                                       |
2020-03-23T16:56:56.9442039Z     |                                                       expected 0 arguments
2020-03-23T16:56:56.9446546Z 
2020-03-23T16:56:58.9737007Z error: aborting due to previous error
---
2020-03-23T17:06:57.9196734Z    Compiling measureme v0.7.1
2020-03-23T17:07:05.3660011Z    Compiling rustc-rayon v0.3.0
2020-03-23T17:07:06.6564630Z    Compiling rayon v1.2.0
2020-03-23T17:07:10.6776565Z    Compiling rustc-ap-rustc_data_structures v642.0.0
2020-03-23T17:07:11.7745905Z error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
2020-03-23T17:07:11.7747100Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:312:29
2020-03-23T17:07:11.7747857Z     |
2020-03-23T17:07:11.7748603Z 312 |             let thread_id = std::thread::current().id().as_u64() as u32;
2020-03-23T17:07:11.7750566Z     |
2020-03-23T17:07:11.7751321Z     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-23T17:07:11.7751723Z 
2020-03-23T17:07:11.7751723Z 
2020-03-23T17:07:11.7938636Z error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
2020-03-23T17:07:11.7939516Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:471:25
2020-03-23T17:07:11.7940151Z     |
2020-03-23T17:07:11.7940814Z 471 |         let thread_id = std::thread::current().id().as_u64() as u32;
2020-03-23T17:07:11.7942147Z     |
2020-03-23T17:07:11.7942845Z     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-23T17:07:11.7943218Z 
2020-03-23T17:07:11.8427288Z error: aborting due to 2 previous errors
---
2020-03-23T17:07:13.4619209Z failed to test rls: could not build
2020-03-23T17:07:13.4806899Z Building stage2 tool rustfmt (x86_64-unknown-linux-gnu)
2020-03-23T17:07:13.6887116Z    Compiling cc v1.0.50
2020-03-23T17:07:13.6919030Z    Compiling rustc-ap-rustc_data_structures v642.0.0
2020-03-23T17:07:14.7704454Z error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
2020-03-23T17:07:14.7705780Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:312:29
2020-03-23T17:07:14.7706640Z     |
2020-03-23T17:07:14.7708053Z 312 |             let thread_id = std::thread::current().id().as_u64() as u32;
2020-03-23T17:07:14.7709522Z     |
2020-03-23T17:07:14.7710523Z     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-23T17:07:14.7711006Z 
2020-03-23T17:07:14.7711006Z 
2020-03-23T17:07:14.7885445Z error[E0605]: non-primitive cast: `std::num::NonZeroU64` as `u32`
2020-03-23T17:07:14.7886412Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-642.0.0/profiling.rs:471:25
2020-03-23T17:07:14.7887063Z     |
2020-03-23T17:07:14.7887778Z 471 |         let thread_id = std::thread::current().id().as_u64() as u32;
2020-03-23T17:07:14.7889215Z     |
2020-03-23T17:07:14.7889966Z     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
2020-03-23T17:07:14.7890393Z 
2020-03-23T17:07:14.8385683Z error: aborting due to 2 previous errors
---
2020-03-23T17:07:56.4923004Z    Compiling rustc_version v0.2.3
2020-03-23T17:08:05.3283730Z    Compiling miri v0.1.0 (/checkout/src/tools/miri)
2020-03-23T17:08:34.9171430Z    Compiling openssl v0.10.25
2020-03-23T17:08:42.7595864Z    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
2020-03-23T17:08:44.7456206Z error[E0599]: no variant or associated item named `UbExperimental` found for enum `rustc_mir::interpret::UndefinedBehaviorInfo` in the current scope
2020-03-23T17:08:44.7457742Z     |
2020-03-23T17:08:44.7457742Z     |
2020-03-23T17:08:44.7458380Z 275 |                     throw_ub!(UbExperimental(format!(
2020-03-23T17:08:44.7459501Z     |                               ^^^^^^^^^^^^^^ variant or associated item not found in `rustc_mir::interpret::UndefinedBehaviorInfo`
2020-03-23T17:08:44.7460064Z 
2020-03-23T17:08:44.7482877Z error[E0599]: no variant or associated item named `UbExperimental` found for enum `rustc_mir::interpret::UndefinedBehaviorInfo` in the current scope
2020-03-23T17:08:44.7484078Z     |
2020-03-23T17:08:44.7484078Z     |
2020-03-23T17:08:44.7484576Z 280 |                     throw_ub!(UbExperimental(format!(
2020-03-23T17:08:44.7485477Z     |                               ^^^^^^^^^^^^^^ variant or associated item not found in `rustc_mir::interpret::UndefinedBehaviorInfo`
2020-03-23T17:08:44.7485946Z 
2020-03-23T17:08:44.7519877Z error[E0599]: no variant or associated item named `UbExperimental` found for enum `rustc_mir::interpret::UndefinedBehaviorInfo` in the current scope
2020-03-23T17:08:44.7521364Z     |
2020-03-23T17:08:44.7521364Z     |
2020-03-23T17:08:44.7521972Z 297 |             err_ub!(UbExperimental(format!(
2020-03-23T17:08:44.7523044Z     |                     ^^^^^^^^^^^^^^ variant or associated item not found in `rustc_mir::interpret::UndefinedBehaviorInfo`
2020-03-23T17:08:44.7523602Z 
2020-03-23T17:08:44.7712639Z error[E0599]: no variant or associated item named `UbExperimental` found for enum `rustc_mir::interpret::UndefinedBehaviorInfo` in the current scope
2020-03-23T17:08:44.7714219Z     |
2020-03-23T17:08:44.7714219Z     |
2020-03-23T17:08:44.7714836Z 341 |             err_ub!(UbExperimental(format!(
2020-03-23T17:08:44.7715934Z     |                     ^^^^^^^^^^^^^^ variant or associated item not found in `rustc_mir::interpret::UndefinedBehaviorInfo`
2020-03-23T17:08:44.7716476Z 
2020-03-23T17:08:44.7755100Z error[E0599]: no variant or associated item named `UbExperimental` found for enum `rustc_mir::interpret::UndefinedBehaviorInfo` in the current scope
2020-03-23T17:08:44.7756559Z     |
2020-03-23T17:08:44.7756559Z     |
2020-03-23T17:08:44.7757198Z 366 |             .ok_or_else(|| err_ub!(UbExperimental(format!(
2020-03-23T17:08:44.7758340Z     |                                    ^^^^^^^^^^^^^^ variant or associated item not found in `rustc_mir::interpret::UndefinedBehaviorInfo`
2020-03-23T17:08:44.8275078Z error: aborting due to 5 previous errors
2020-03-23T17:08:44.8275362Z 
2020-03-23T17:08:44.8275853Z For more information about this error, try `rustc --explain E0599`.
2020-03-23T17:08:44.8342753Z error: could not compile `miri`.
---
2020-03-23T17:08:47.3423714Z Verifying status of edition-guide...
2020-03-23T17:08:47.3423941Z Verifying status of rls...
2020-03-23T17:08:47.3424386Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2020-03-23T17:08:47.3424589Z 
2020-03-23T17:08:47.3424966Z We detected that this PR updated 'rls', but its tests failed.
2020-03-23T17:08:47.3425168Z 
2020-03-23T17:08:47.3427049Z If you do intend to update 'rls', please check the error messages above and
2020-03-23T17:08:47.3427584Z commit another update.
2020-03-23T17:08:47.3427745Z 
2020-03-23T17:08:47.3428290Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2020-03-23T17:08:47.3429946Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2020-03-23T17:08:47.3430219Z proper steps.
2020-03-23T17:08:47.3431133Z Build completed unsuccessfully in 0:00:01
2020-03-23T17:08:47.3479015Z == clock drift check ==
2020-03-23T17:08:47.3492651Z   local time: Mon Mar 23 17:08:47 UTC 2020
2020-03-23T17:08:47.6462756Z   network time: Mon, 23 Mar 2020 17:08:47 GMT
2020-03-23T17:08:47.6462756Z   network time: Mon, 23 Mar 2020 17:08:47 GMT
2020-03-23T17:08:47.6466267Z == end clock drift check ==
2020-03-23T17:08:48.5303141Z 
2020-03-23T17:08:48.5369840Z ##[error]Bash exited with code '1'.
2020-03-23T17:08:48.5383334Z ##[section]Finishing: Run build
2020-03-23T17:08:48.5432409Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70149/merge to s
2020-03-23T17:08:48.5437814Z Task         : Get sources
2020-03-23T17:08:48.5438147Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:08:48.5438436Z Version      : 1.0.0
2020-03-23T17:08:48.5438646Z Author       : Microsoft
2020-03-23T17:08:48.5438646Z Author       : Microsoft
2020-03-23T17:08:48.5438988Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T17:08:48.5439357Z ==============================================================================
2020-03-23T17:08:48.8652321Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T17:08:48.8705454Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70149/merge to s
2020-03-23T17:08:48.8792906Z Cleaning up task key
2020-03-23T17:08:48.8794353Z Start cleaning up orphan processes.
2020-03-23T17:08:48.8965700Z Terminate orphan process: pid (3455) (python)
2020-03-23T17:08:48.9405525Z ##[section]Finishing: Finalize Job
