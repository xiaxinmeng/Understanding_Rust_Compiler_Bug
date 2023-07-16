plain
2020-03-20T10:53:26.6184877Z ========================== Starting Command Output ===========================
2020-03-20T10:53:26.6187196Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68a78713-7781-4001-885b-1a4dd447d927.sh
2020-03-20T10:53:26.6187469Z 
2020-03-20T10:53:26.6190600Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T10:53:26.6209630Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T10:53:26.6213705Z Task         : Get sources
2020-03-20T10:53:26.6214028Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T10:53:26.6214331Z Version      : 1.0.0
2020-03-20T10:53:26.6214531Z Author       : Microsoft
---
2020-03-20T10:53:29.5381805Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T10:53:29.5387627Z ##[command]git config gc.auto 0
2020-03-20T10:53:29.5390777Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T10:53:29.5394014Z ##[command]git config --get-all http.proxy
2020-03-20T10:53:29.5401329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
---
2020-03-20T12:31:43.3935765Z     Finished release [optimized] target(s) in 9m 08s
2020-03-20T12:31:43.4165667Z Testing rustbook src/doc/book
2020-03-20T12:31:57.2055589Z Error: Rustdoc returned an error: 
2020-03-20T12:31:57.2055884Z running 4 tests
2020-03-20T12:31:57.2057087Z test /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 15) ... ignored
2020-03-20T12:31:57.2058236Z test /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 35) ... ignored
2020-03-20T12:31:57.2059187Z test /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) ... FAILED
2020-03-20T12:31:57.2060234Z test /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 62) ... ok
2020-03-20T12:31:57.2060739Z failures:
2020-03-20T12:31:57.2060856Z 
2020-03-20T12:31:57.2060856Z 
2020-03-20T12:31:57.2061531Z ---- /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) stdout ----
2020-03-20T12:31:57.2062298Z error: Cannot declare a non-inline module inside a block unless it has a path attribute
2020-03-20T12:31:57.2062969Z  --> /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md:53:1
2020-03-20T12:31:57.2063423Z 3 | pub mod hosting;
2020-03-20T12:31:57.2063619Z   | ^^^^^^^^^^^^^^^^
2020-03-20T12:31:57.2063770Z 
2020-03-20T12:31:57.2063958Z error: aborting due to previous error
2020-03-20T12:31:57.2063958Z error: aborting due to previous error
2020-03-20T12:31:57.2064132Z 
2020-03-20T12:31:57.2064455Z Couldn't compile the test.
2020-03-20T12:31:57.2064617Z 
2020-03-20T12:31:57.2064755Z failures:
2020-03-20T12:31:57.2065396Z     /tmp/mdbook-TfHh5F/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52)
2020-03-20T12:31:57.2066253Z test result: FAILED. 1 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
2020-03-20T12:31:57.2066499Z 
2020-03-20T12:31:57.2066598Z 
2020-03-20T12:31:57.2072132Z 
---
2020-03-20T12:33:06.3197761Z  finished in 1.175
2020-03-20T12:33:06.3204080Z Testing rustbook src/doc/edition-guide
2020-03-20T12:33:14.4796607Z  finished in 8.159
2020-03-20T12:34:41.5501130Z Timeout for link `http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz`
2020-03-20T12:34:41.5518513Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-20T12:34:41.5519503Z     ┌── walkthrough.md:57:3 ───
2020-03-20T12:34:41.5519953Z     │
2020-03-20T12:34:41.5519953Z     │
2020-03-20T12:34:41.5520765Z  57 │   [propose to stabilize it][merge]. If there is consensus, this is done.
2020-03-20T12:34:41.5522011Z     │
2020-03-20T12:34:41.5522248Z 
2020-03-20T12:34:41.5523130Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-20T12:34:41.5523540Z 
2020-03-20T12:34:41.5523540Z 
2020-03-20T12:34:41.5524006Z     ┌── walkthrough.md:74:64 ───
2020-03-20T12:34:41.5524463Z     │
2020-03-20T12:34:41.5525049Z  74 │ > You can find the official guidelines for when to open an RFC [here][rfcwhen].
2020-03-20T12:34:41.5526412Z     │
2020-03-20T12:34:41.5526631Z 
2020-03-20T12:34:41.5527251Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-20T12:34:41.5527658Z 
2020-03-20T12:34:41.5527658Z 
2020-03-20T12:34:41.5528110Z     ┌── walkthrough.md:83:1 ───
2020-03-20T12:34:41.5528551Z     │
2020-03-20T12:34:41.5529363Z  83 │ [rust-lang/rfcs](https://github.com/rust-lang/rfcs) repo on GitHub. You can
2020-03-20T12:34:41.5530904Z     │
2020-03-20T12:34:41.5531132Z 
2020-03-20T12:34:41.5531781Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-20T12:34:41.5532174Z 
2020-03-20T12:34:41.5532174Z 
2020-03-20T12:34:41.5532660Z     ┌── walkthrough.md:85:1 ───
2020-03-20T12:34:41.5533102Z     │
2020-03-20T12:34:41.5533677Z  85 │ [README](https://github.com/rust-lang/rfcs#what-the-process-is).
2020-03-20T12:34:41.5535045Z     │
2020-03-20T12:34:41.5535282Z 
2020-03-20T12:34:41.5535925Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"
2020-03-20T12:34:41.5536333Z 
2020-03-20T12:34:41.5536333Z 
2020-03-20T12:34:41.5536815Z      ┌── walkthrough.md:107:51 ───
2020-03-20T12:34:41.5537283Z      │
2020-03-20T12:34:41.5537869Z  107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
2020-03-20T12:34:41.5539687Z      │
2020-03-20T12:34:41.5540515Z 
2020-03-20T12:34:41.5541167Z error: The server responded with 429 Too Many Requests for "***"
2020-03-20T12:34:41.5541391Z 
2020-03-20T12:34:41.5541391Z 
2020-03-20T12:34:41.5541765Z      ┌── walkthrough.md:146:26 ───
2020-03-20T12:34:41.5542122Z      │
2020-03-20T12:34:41.5542615Z  146 │ issue_ is created in the [rust-lang/rust] repo to track progress on the feature
2020-03-20T12:34:41.5543226Z      │                          ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5543639Z      │
2020-03-20T12:34:41.5543753Z 
2020-03-20T12:34:41.5544337Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-20T12:34:41.5544977Z      ┌── walkthrough.md:148:39 ───
2020-03-20T12:34:41.5545316Z      │
2020-03-20T12:34:41.5545316Z      │
2020-03-20T12:34:41.5545795Z  148 │ Here is the tracking issue on for our [`?` macro feature][tracking].
2020-03-20T12:34:41.5546848Z      │
2020-03-20T12:34:41.5546963Z 
2020-03-20T12:34:41.5547529Z error: The server responded with 429 Too Many Requests for "***"
2020-03-20T12:34:41.5547747Z 
2020-03-20T12:34:41.5547747Z 
2020-03-20T12:34:41.5548116Z      ┌── walkthrough.md:156:57 ───
2020-03-20T12:34:41.5548474Z      │
2020-03-20T12:34:41.5548963Z  156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
2020-03-20T12:34:41.5549654Z      │                                                         ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5550124Z      │
2020-03-20T12:34:41.5550239Z 
2020-03-20T12:34:41.5550828Z error: The server responded with 429 Too Many Requests for "***/pull/47732"
2020-03-20T12:34:41.5551451Z      ┌── walkthrough.md:167:58 ───
2020-03-20T12:34:41.5551790Z      │
2020-03-20T12:34:41.5552271Z  167 │ macro expansion in the compiler. Personally, I find that [improving the
2020-03-20T12:34:41.5552966Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5552966Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5553419Z      │
2020-03-20T12:34:41.5553531Z 
2020-03-20T12:34:41.5554089Z error: The server responded with 429 Too Many Requests for "***"
2020-03-20T12:34:41.5554310Z 
2020-03-20T12:34:41.5554679Z      ┌── walkthrough.md:181:27 ───
2020-03-20T12:34:41.5574529Z      │
2020-03-20T12:34:41.5575015Z  181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
2020-03-20T12:34:41.5595816Z      │
2020-03-20T12:34:41.5595936Z 
2020-03-20T12:34:41.5595936Z 
2020-03-20T12:34:41.5596712Z error: The server responded with 429 Too Many Requests for "***/issues/51934"
2020-03-20T12:34:41.5597381Z      ┌── walkthrough.md:237:32 ───
2020-03-20T12:34:41.5597813Z      │
2020-03-20T12:34:41.5598229Z  237 │   from the original RFC required [another
2020-03-20T12:34:41.5598708Z      │ ╭────────────────────────────────^
2020-03-20T12:34:41.5598708Z      │ ╭────────────────────────────────^
2020-03-20T12:34:41.5599236Z  238 │ │ FCP](***/issues/51934).
2020-03-20T12:34:41.5600389Z      │
2020-03-20T12:34:41.5600494Z 
2020-03-20T12:34:41.5600494Z 
2020-03-20T12:34:41.5601046Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-20T12:34:41.5601615Z      ┌── walkthrough.md:243:1 ───
2020-03-20T12:34:41.5601930Z      │
2020-03-20T12:34:41.5601930Z      │
2020-03-20T12:34:41.5602301Z  243 │ [moved to stabilize it][stabilizefcp].
2020-03-20T12:34:41.5603397Z      │
2020-03-20T12:34:41.5603517Z 
2020-03-20T12:34:41.5603517Z 
2020-03-20T12:34:41.5604062Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-20T12:34:41.5604659Z      ┌── walkthrough.md:253:45 ───
2020-03-20T12:34:41.5604977Z      │
2020-03-20T12:34:41.5604977Z      │
2020-03-20T12:34:41.5605389Z  253 │ The stabilization report for our feature is [here][stabrep].
2020-03-20T12:34:41.5606586Z      │
2020-03-20T12:34:41.5606699Z 
2020-03-20T12:34:41.5606699Z 
2020-03-20T12:34:41.5607288Z error: The server responded with 429 Too Many Requests for "***/pull/56245"
2020-03-20T12:34:41.5607898Z      ┌── walkthrough.md:257:13 ───
2020-03-20T12:34:41.5608238Z      │
2020-03-20T12:34:41.5608238Z      │
2020-03-20T12:34:41.5608761Z  257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
2020-03-20T12:34:41.5609731Z      │
2020-03-20T12:34:41.5609862Z 
2020-03-20T12:34:41.5609862Z 
2020-03-20T12:34:41.5610477Z error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
2020-03-20T12:34:41.5611131Z      ┌── walkthrough.md:258:55 ───
2020-03-20T12:34:41.5611475Z      │
2020-03-20T12:34:41.5611475Z      │
2020-03-20T12:34:41.5611969Z  258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
2020-03-20T12:34:41.5613118Z      │
2020-03-20T12:34:41.5613231Z 
2020-03-20T12:34:41.5613898Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"
2020-03-20T12:34:41.5614303Z 
---
2020-03-20T12:34:41.5626492Z  81 │ example of how such an issue should look can be [found
2020-03-20T12:34:41.5627114Z     │                                                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5627547Z     │
2020-03-20T12:34:41.5627672Z 
2020-03-20T12:34:41.5628459Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-20T12:34:41.5629215Z      ┌── bug-fix-procedure.md:235:65 ───
2020-03-20T12:34:41.5629580Z      │
2020-03-20T12:34:41.5630041Z  235 │ The first reference you will likely find is the lint definition [in
2020-03-20T12:34:41.5630744Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5630744Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5631213Z      │
2020-03-20T12:34:41.5631328Z 
2020-03-20T12:34:41.5632091Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-20T12:34:41.5632830Z      ┌── bug-fix-procedure.md:250:13 ───
2020-03-20T12:34:41.5633197Z      │
2020-03-20T12:34:41.5633197Z      │
2020-03-20T12:34:41.5633668Z  250 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
2020-03-20T12:34:41.5634633Z      │
2020-03-20T12:34:41.5634746Z 
2020-03-20T12:34:41.5634746Z 
2020-03-20T12:34:41.5635489Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
2020-03-20T12:34:41.5636242Z      ┌── bug-fix-procedure.md:254:19 ───
2020-03-20T12:34:41.5636597Z      │
2020-03-20T12:34:41.5637051Z  254 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
2020-03-20T12:34:41.5637631Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5637631Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5638017Z      │
2020-03-20T12:34:41.5638131Z 
2020-03-20T12:34:41.5638943Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
2020-03-20T12:34:41.5639707Z      ┌── bug-fix-procedure.md:283:16 ───
2020-03-20T12:34:41.5640073Z      │
2020-03-20T12:34:41.5640073Z      │
2020-03-20T12:34:41.5640535Z  283 │ this case, the [`add_lint` call][addlintsource] looks like this:
2020-03-20T12:34:41.5641503Z      │
2020-03-20T12:34:41.5641617Z 
2020-03-20T12:34:41.5642368Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"
2020-03-20T12:34:41.5647981Z 
2020-03-20T12:34:41.5647981Z 
2020-03-20T12:34:41.5648505Z     ┌── implementing_new_features.md:56:4 ───
2020-03-20T12:34:41.5648847Z     │
2020-03-20T12:34:41.5649277Z  56 │ We [value the stability of Rust]. Code that works and runs on stable
2020-03-20T12:34:41.5649785Z     │    ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5650121Z     │
2020-03-20T12:34:41.5650225Z 
2020-03-20T12:34:41.5650987Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-20T12:34:41.5651584Z     ┌── stability.md:18:51 ───
2020-03-20T12:34:41.5651928Z     │
2020-03-20T12:34:41.5652572Z  18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
2020-03-20T12:34:41.5653263Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5653263Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5653801Z     │
2020-03-20T12:34:41.5653915Z 
2020-03-20T12:34:41.5654506Z error: The server responded with 429 Too Many Requests for "***/issues/15702"
2020-03-20T12:34:41.5655122Z     ┌── stability.md:31:30 ───
2020-03-20T12:34:41.5655449Z     │
2020-03-20T12:34:41.5655449Z     │
2020-03-20T12:34:41.5655935Z  31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
2020-03-20T12:34:41.5656970Z     │
2020-03-20T12:34:41.5657081Z 
2020-03-20T12:34:41.5657081Z 
2020-03-20T12:34:41.5657731Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
2020-03-20T12:34:41.5658415Z     ┌── stabilization_guide.md:17:38 ───
2020-03-20T12:34:41.5658783Z     │
2020-03-20T12:34:41.5658783Z     │
2020-03-20T12:34:41.5659230Z  17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
2020-03-20T12:34:41.5660275Z     │
2020-03-20T12:34:41.5660389Z 
2020-03-20T12:34:41.5660936Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"
2020-03-20T12:34:41.5661247Z 
---
2020-03-20T12:34:41.5702192Z  28 │ - [The Book]: This may or may not need updating, depends.
2020-03-20T12:34:41.5702752Z     │   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5703110Z     │
2020-03-20T12:34:41.5703224Z 
2020-03-20T12:34:41.5703848Z error: The server responded with 429 Too Many Requests for "***-by-example"
2020-03-20T12:34:41.5704480Z     ┌── stabilization_guide.md:35:3 ───
2020-03-20T12:34:41.5704844Z     │
2020-03-20T12:34:41.5705219Z  35 │ - [Rust by Example]: As needed.
2020-03-20T12:34:41.5705700Z     │   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5705700Z     │   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5706072Z     │
2020-03-20T12:34:41.5706184Z 
2020-03-20T12:34:41.5706768Z error: The server responded with 429 Too Many Requests for "***/issues/32409"
2020-03-20T12:34:41.5707415Z     ┌── stabilization_guide.md:97:1 ───
2020-03-20T12:34:41.5707767Z     │
2020-03-20T12:34:41.5708119Z  97 │ [rust-lang/rust#32409]:
2020-03-20T12:34:41.5708583Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5708583Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5708936Z     │
2020-03-20T12:34:41.5709057Z 
2020-03-20T12:34:41.5709647Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc"
2020-03-20T12:34:41.5710001Z 
2020-03-20T12:34:41.5710386Z      ┌── compiler-debugging.md:258:5 ───
2020-03-20T12:34:41.5710742Z      │
2020-03-20T12:34:41.5711241Z  258 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
2020-03-20T12:34:41.5712159Z      │
2020-03-20T12:34:41.5712271Z 
2020-03-20T12:34:41.5712939Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc/blob/master/TUTORIAL.md"
2020-03-20T12:34:41.5713318Z 
2020-03-20T12:34:41.5713318Z 
2020-03-20T12:34:41.5713705Z      ┌── compiler-debugging.md:262:31 ───
2020-03-20T12:34:41.5714217Z      │
2020-03-20T12:34:41.5714721Z  262 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
2020-03-20T12:34:41.5715851Z      │
2020-03-20T12:34:41.5715965Z 
2020-03-20T12:34:41.5716571Z error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"
2020-03-20T12:34:41.5716916Z 
2020-03-20T12:34:41.5716916Z 
2020-03-20T12:34:41.5717316Z      ┌── compiler-debugging.md:270:5 ───
2020-03-20T12:34:41.5717673Z      │
2020-03-20T12:34:41.5718150Z  270 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
2020-03-20T12:34:41.5719075Z      │
2020-03-20T12:34:41.5719189Z 
2020-03-20T12:34:41.5719758Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf"
2020-03-20T12:34:41.5720096Z 
2020-03-20T12:34:41.5720096Z 
2020-03-20T12:34:41.5720450Z    ┌── profiling.md:8:9 ───
2020-03-20T12:34:41.5720773Z    │
2020-03-20T12:34:41.5721489Z  8 │   - The [rustc-perf](https://github.com/rust-lang-nursery/rustc-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
2020-03-20T12:34:41.5723203Z    │
2020-03-20T12:34:41.5723331Z 
2020-03-20T12:34:41.5723883Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"
2020-03-20T12:34:41.5724192Z 
2020-03-20T12:34:41.5724192Z 
2020-03-20T12:34:41.5724546Z     ┌── profiling.md:11:35 ───
2020-03-20T12:34:41.5724891Z     │
2020-03-20T12:34:41.5725552Z  11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
2020-03-20T12:34:41.5726955Z     │
2020-03-20T12:34:41.5727068Z 
2020-03-20T12:34:41.5727697Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"
2020-03-20T12:34:41.5728089Z 
---
2020-03-20T12:34:41.5737688Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf/tree/master/collector/benchmarks"
2020-03-20T12:34:41.5738083Z 
2020-03-20T12:34:41.5738546Z     ┌── profiling/with_perf.md:93:14 ───
2020-03-20T12:34:41.5738905Z     │
2020-03-20T12:34:41.5739367Z  93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
2020-03-20T12:34:41.5740396Z     │
2020-03-20T12:34:41.5740507Z 
2020-03-20T12:34:41.5741074Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-20T12:34:41.5741394Z 
2020-03-20T12:34:41.5741394Z 
2020-03-20T12:34:41.5741776Z      ┌── profiling/with_perf.md:137:45 ───
2020-03-20T12:34:41.5742124Z      │
2020-03-20T12:34:41.5742600Z  137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
2020-03-20T12:34:41.5743666Z      │
2020-03-20T12:34:41.5743793Z 
2020-03-20T12:34:41.5744361Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-20T12:34:41.5744677Z 
2020-03-20T12:34:41.5744677Z 
2020-03-20T12:34:41.5745071Z      ┌── profiling/with_perf.md:161:38 ───
2020-03-20T12:34:41.5745430Z      │
2020-03-20T12:34:41.5745896Z  161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
2020-03-20T12:34:41.5746945Z      │
2020-03-20T12:34:41.5747059Z 
2020-03-20T12:34:41.5747620Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/fmt-rfcs"
2020-03-20T12:34:41.5747956Z 
2020-03-20T12:34:41.5747956Z 
2020-03-20T12:34:41.5748316Z     ┌── conventions.md:10:36 ───
2020-03-20T12:34:41.5748650Z     │
2020-03-20T12:34:41.5749109Z  10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
2020-03-20T12:34:41.5750143Z     │
2020-03-20T12:34:41.5750253Z 
2020-03-20T12:34:41.5750253Z 
2020-03-20T12:34:41.5751152Z error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
2020-03-20T12:34:41.5751873Z     ┌── crates-io.md:19:23 ───
2020-03-20T12:34:41.5752218Z     │
2020-03-20T12:34:41.5752218Z     │
2020-03-20T12:34:41.5752684Z  19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
2020-03-20T12:34:41.5753668Z     │
2020-03-20T12:34:41.5753782Z 
2020-03-20T12:34:41.5754313Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"
2020-03-20T12:34:41.5754611Z 
---
2020-03-20T12:34:41.5757730Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustfix"
2020-03-20T12:34:41.5758115Z 
2020-03-20T12:34:41.5758602Z     ┌── diagnostics.md:82:18 ───
2020-03-20T12:34:41.5758910Z     │
2020-03-20T12:34:41.5759275Z  82 │ Server][rls] and [`rustfix`][rustfix].
2020-03-20T12:34:41.5760102Z     │
2020-03-20T12:34:41.5760219Z 
2020-03-20T12:34:41.5760765Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"
2020-03-20T12:34:41.5761119Z 
---
2020-03-20T12:34:41.5776965Z  57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
2020-03-20T12:34:41.5777522Z     │             ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5777896Z     │
2020-03-20T12:34:41.5778006Z 
2020-03-20T12:34:41.5778668Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
2020-03-20T12:34:41.5779334Z    ┌── ice-breaker/cleanup-crew.md:3:19 ───
2020-03-20T12:34:41.5779696Z    │
2020-03-20T12:34:41.5780098Z  3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
2020-03-20T12:34:41.5780630Z    │                   ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.5791281Z  80 │ To learn to use [cargo-bisect-rustc], check out [this blog
2020-03-20T12:34:41.5791827Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5792217Z     │
2020-03-20T12:34:41.5792330Z 
2020-03-20T12:34:41.5792899Z error: The server responded with 429 Too Many Requests for "***/"
2020-03-20T12:34:41.5793541Z      ┌── ice-breaker/cleanup-crew.md:102:36 ───
2020-03-20T12:34:41.5793903Z      │
2020-03-20T12:34:41.5794353Z  102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
2020-03-20T12:34:41.5794978Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5794978Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5795401Z      │
2020-03-20T12:34:41.5795526Z 
2020-03-20T12:34:41.5796153Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
2020-03-20T12:34:41.5796795Z    ┌── ice-breaker/llvm.md:3:19 ───
2020-03-20T12:34:41.5797125Z    │
2020-03-20T12:34:41.5797952Z  3 │ **Github Label:** [ICEBreaker-LLVM]
2020-03-20T12:34:41.5798476Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5798476Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5798855Z    │
2020-03-20T12:34:41.5799031Z 
2020-03-20T12:34:41.5799610Z error: The server responded with 429 Too Many Requests for "***c-guide"
2020-03-20T12:34:41.5800223Z     ┌── part-2-intro.md:10:17 ───
2020-03-20T12:34:41.5800560Z     │
2020-03-20T12:34:41.5800560Z     │
2020-03-20T12:34:41.5801073Z  10 │ an issue on the [rustc-guide repo](***c-guide)
2020-03-20T12:34:41.5802260Z     │
2020-03-20T12:34:41.5802372Z 
2020-03-20T12:34:41.5802372Z 
2020-03-20T12:34:41.5803397Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
2020-03-20T12:34:41.5804033Z    ┌── rustdoc.md:6:50 ───
2020-03-20T12:34:41.5804369Z    │
2020-03-20T12:34:41.5804369Z    │
2020-03-20T12:34:41.5804845Z  6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
2020-03-20T12:34:41.5805951Z    │
2020-03-20T12:34:41.5806062Z 
2020-03-20T12:34:41.5806062Z 
2020-03-20T12:34:41.5806690Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
2020-03-20T12:34:41.5807326Z     ┌── rustdoc.md:26:22 ───
2020-03-20T12:34:41.5807653Z     │
2020-03-20T12:34:41.5808134Z  26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
2020-03-20T12:34:41.5808739Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5808739Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5809251Z     │
2020-03-20T12:34:41.5809368Z 
2020-03-20T12:34:41.5809981Z error: The server responded with 429 Too Many Requests for "***/issues/44136"
2020-03-20T12:34:41.5810666Z      ┌── rustdoc.md:115:1 ───
2020-03-20T12:34:41.5811012Z      │
2020-03-20T12:34:41.5811012Z      │
2020-03-20T12:34:41.5811502Z  115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
2020-03-20T12:34:41.5812409Z      │
2020-03-20T12:34:41.5812537Z 
2020-03-20T12:34:41.5812537Z 
2020-03-20T12:34:41.5813176Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
2020-03-20T12:34:41.5813820Z      ┌── query.md:150:1 ───
2020-03-20T12:34:41.5814146Z      │
2020-03-20T12:34:41.5814631Z  150 │ [`rustc_metadata` crate][rustc_metadata], which loads the information from the
2020-03-20T12:34:41.5815187Z      │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5815187Z      │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5815549Z      │
2020-03-20T12:34:41.5815675Z 
2020-03-20T12:34:41.5816327Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/dep_graph"
2020-03-20T12:34:41.5817040Z     ┌── queries/incremental-compilation.md:84:1 ───
2020-03-20T12:34:41.5817407Z     │
2020-03-20T12:34:41.5817407Z     │
2020-03-20T12:34:41.5817883Z  84 │ [`src/librustc/dep_graph`][dep_graph]. Construction of the DAG is done
2020-03-20T12:34:41.5818746Z     │
2020-03-20T12:34:41.5818867Z 
2020-03-20T12:34:41.5818867Z 
2020-03-20T12:34:41.5819445Z error: The server responded with 429 Too Many Requests for "***/issues/42678"
2020-03-20T12:34:41.5820068Z    ┌── queries/profiling.md:8:9 ───
2020-03-20T12:34:41.5820401Z    │
2020-03-20T12:34:41.5820401Z    │
2020-03-20T12:34:41.5820892Z  8 │ address [issue 42678](***/issues/42678).
2020-03-20T12:34:41.5822029Z    │
2020-03-20T12:34:41.5822248Z 
2020-03-20T12:34:41.5822953Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"
2020-03-20T12:34:41.5823365Z 
2020-03-20T12:34:41.5823365Z 
2020-03-20T12:34:41.5823709Z      ┌── queries/profiling.md:335:3 ───
2020-03-20T12:34:41.5824044Z      │
2020-03-20T12:34:41.5824744Z  335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
2020-03-20T12:34:41.5826429Z      │
2020-03-20T12:34:41.5826536Z 
2020-03-20T12:34:41.5826536Z 
2020-03-20T12:34:41.5827085Z error: The server responded with 429 Too Many Requests for "***/issues/42293"
2020-03-20T12:34:41.5827675Z      ┌── queries/profiling.md:337:3 ───
2020-03-20T12:34:41.5827994Z      │
2020-03-20T12:34:41.5827994Z      │
2020-03-20T12:34:41.5828533Z  337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
2020-03-20T12:34:41.5829759Z      │
2020-03-20T12:34:41.5829865Z 
2020-03-20T12:34:41.5829865Z 
2020-03-20T12:34:41.5830402Z error: The server responded with 429 Too Many Requests for "***/issues/42633"
2020-03-20T12:34:41.5830994Z      ┌── queries/profiling.md:341:3 ───
2020-03-20T12:34:41.5831313Z      │
2020-03-20T12:34:41.5831313Z      │
2020-03-20T12:34:41.5831785Z  341 │ - [GitHub issue #42633](***/issues/42633)
2020-03-20T12:34:41.5832842Z      │
2020-03-20T12:34:41.5832963Z 
2020-03-20T12:34:41.5833514Z error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"
2020-03-20T12:34:41.5833793Z 
2020-03-20T12:34:41.5833793Z 
2020-03-20T12:34:41.5834156Z    ┌── salsa.md:5:1 ───
2020-03-20T12:34:41.5834460Z    │
2020-03-20T12:34:41.5834829Z  5 │ [Salsa](https://github.com/salsa-rs/salsa).
2020-03-20T12:34:41.5835774Z    │
2020-03-20T12:34:41.5835881Z 
2020-03-20T12:34:41.5836431Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/index.html"
2020-03-20T12:34:41.5836747Z 
---
2020-03-20T12:34:41.5841174Z  49 │ - [`StringReader`] from [libsyntax] integrates `rustc_lexer` with `rustc`
2020-03-20T12:34:41.5841705Z     │                         ^ Server responded with 404 Not Found
2020-03-20T12:34:41.5842118Z     │
2020-03-20T12:34:41.5842235Z 
2020-03-20T12:34:41.5843312Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-20T12:34:41.5843991Z     ┌── test-implementation.md:34:1 ───
2020-03-20T12:34:41.5844341Z     │
2020-03-20T12:34:41.5844805Z  34 │ [`libsyntax` crate][libsyntax]. Essentially, it's a fancy macro, that
2020-03-20T12:34:41.5845327Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.5851294Z  69 │ not stored as a string, but rather as an opaque [Symbol][Symbol] which is
2020-03-20T12:34:41.5851937Z     │                                                 ^ Server responded with 404 Not Found
2020-03-20T12:34:41.5852359Z     │
2020-03-20T12:34:41.5852496Z 
2020-03-20T12:34:41.5853137Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
2020-03-20T12:34:41.5853809Z     ┌── macro-expansion.md:13:1 ───
2020-03-20T12:34:41.5854204Z     │
2020-03-20T12:34:41.5854689Z  13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2020-03-20T12:34:41.5855240Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.5873552Z  135 │ [`NodeId`]. This returns a `Option<Node<'tcx>>`, where [`Node`] is an enum
2020-03-20T12:34:41.5874222Z      │ ^ Server responded with 404 Not Found
2020-03-20T12:34:41.5874687Z      │
2020-03-20T12:34:41.5874903Z 
2020-03-20T12:34:41.5875716Z error: The server responded with 429 Too Many Requests for "***/blob/3ee936378662bd2e74be951d6a7011a95a6bd84d/src/librustc/ty/mod.rs"
2020-03-20T12:34:41.5876557Z      ┌── ty.md:199:50 ───
2020-03-20T12:34:41.5876880Z      │
2020-03-20T12:34:41.5876880Z      │
2020-03-20T12:34:41.5877567Z  199 │ comparison of types for equality: we implemented [`PartialEq for TyS`][peqimpl], so we can just
2020-03-20T12:34:41.5878985Z      │
2020-03-20T12:34:41.5879202Z 
2020-03-20T12:34:41.5879202Z 
2020-03-20T12:34:41.5880036Z error: The server responded with 429 Too Many Requests for "***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs"
2020-03-20T12:34:41.5880894Z      ┌── ty.md:468:1 ───
2020-03-20T12:34:41.5881343Z      │
2020-03-20T12:34:41.5881343Z      │
2020-03-20T12:34:41.5882010Z  468 │ [Here is an example of actually using `subst` in the compiler][substex].  The exact details are not
2020-03-20T12:34:41.5883489Z      │
2020-03-20T12:34:41.5883707Z 
2020-03-20T12:34:41.5883707Z 
2020-03-20T12:34:41.5884484Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_macros/src/type_foldable.rs"
2020-03-20T12:34:41.5885136Z      ┌── ty.md:573:1 ───
2020-03-20T12:34:41.5885469Z      │
2020-03-20T12:34:41.5885469Z      │
2020-03-20T12:34:41.5886033Z  573 │ [here](***/blob/master/src/librustc_macros/src/type_foldable.rs).
2020-03-20T12:34:41.5887446Z      │
2020-03-20T12:34:41.5887564Z 
2020-03-20T12:34:41.5887564Z 
2020-03-20T12:34:41.5888345Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-20T12:34:41.5889112Z      ┌── ty.md:575:46 ───
2020-03-20T12:34:41.5889446Z      │
2020-03-20T12:34:41.5889446Z      │
2020-03-20T12:34:41.5889875Z  575 │   **`subst`** In the case of substitutions the [actual
2020-03-20T12:34:41.5890606Z      │ ╭──────────────────────────────────────────────^
2020-03-20T12:34:41.5891378Z  576 │ │ folder](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L467-L482)
2020-03-20T12:34:41.5893224Z      │
2020-03-20T12:34:41.5893441Z 
2020-03-20T12:34:41.5893441Z 
2020-03-20T12:34:41.5894254Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-20T12:34:41.5895076Z      ┌── ty.md:579:1 ───
2020-03-20T12:34:41.5895530Z      │
2020-03-20T12:34:41.5895530Z      │
2020-03-20T12:34:41.5896326Z  579 │ [fold_ty](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L545-L573)
2020-03-20T12:34:41.5898051Z      │
2020-03-20T12:34:41.5898267Z 
2020-03-20T12:34:41.5898267Z 
2020-03-20T12:34:41.5899075Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-20T12:34:41.5899888Z      ┌── ty.md:583:1 ───
2020-03-20T12:34:41.5900339Z      │
2020-03-20T12:34:41.5900339Z      │
2020-03-20T12:34:41.5901136Z  583 │ [ty_for_param](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L589-L624)
2020-03-20T12:34:41.5902926Z      │
2020-03-20T12:34:41.5903141Z 
2020-03-20T12:34:41.5903141Z 
2020-03-20T12:34:41.5903908Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/infer/higher_ranked/README.md"
2020-03-20T12:34:41.5904582Z     ┌── traits/hrtb.md:35:62 ───
2020-03-20T12:34:41.5904924Z     │
2020-03-20T12:34:41.5905413Z  35 │ to the subtyping for higher-ranked types (which is described [here][hrsubtype]
2020-03-20T12:34:41.5906308Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5906308Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5906863Z     │
2020-03-20T12:34:41.5907063Z 
2020-03-20T12:34:41.5907661Z error: The server responded with 429 Too Many Requests for "***/issues/22019"
2020-03-20T12:34:41.5908416Z     ┌── traits/caching.md:57:30 ───
2020-03-20T12:34:41.5908879Z     │
2020-03-20T12:34:41.5908879Z     │
2020-03-20T12:34:41.5909439Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-20T12:34:41.5910708Z     │
2020-03-20T12:34:41.5910908Z 
2020-03-20T12:34:41.5910908Z 
2020-03-20T12:34:41.5911497Z error: The server responded with 429 Too Many Requests for "***/issues/18290"
2020-03-20T12:34:41.5912200Z     ┌── traits/caching.md:57:43 ───
2020-03-20T12:34:41.5912623Z     │
2020-03-20T12:34:41.5912623Z     │
2020-03-20T12:34:41.5913192Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-20T12:34:41.5914417Z     │
2020-03-20T12:34:41.5914616Z 
2020-03-20T12:34:41.5914616Z 
2020-03-20T12:34:41.5915200Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-03-20T12:34:41.5915920Z    ┌── traits/index.md:4:3 ───
2020-03-20T12:34:41.5916218Z    │
2020-03-20T12:34:41.5916218Z    │
2020-03-20T12:34:41.5916771Z  4 │ > [process of being implemented][wg]; this chapter serves as a kind of
2020-03-20T12:34:41.5917829Z    │
2020-03-20T12:34:41.5918044Z 
2020-03-20T12:34:41.5918044Z 
2020-03-20T12:34:41.5918625Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-03-20T12:34:41.5919318Z     ┌── traits/index.md:11:3 ───
2020-03-20T12:34:41.5919733Z     │
2020-03-20T12:34:41.5919733Z     │
2020-03-20T12:34:41.5920213Z  11 │ > [Traits Working Group tracking issue][wg]!
2020-03-20T12:34:41.5921238Z     │
2020-03-20T12:34:41.5921435Z 
2020-03-20T12:34:41.5922051Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/tree/master/chalk-engine"
2020-03-20T12:34:41.5922373Z 
2020-03-20T12:34:41.5922373Z 
2020-03-20T12:34:41.5922928Z     ┌── traits/index.md:54:7 ───
2020-03-20T12:34:41.5923415Z     │
2020-03-20T12:34:41.5924183Z  54 │ * the [`chalk_engine`][chalk_engine] crate, which
2020-03-20T12:34:41.5924805Z     │       ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5925259Z     │
2020-03-20T12:34:41.5925832Z 
2020-03-20T12:34:41.5926742Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-03-20T12:34:41.5927630Z     ┌── traits/index.md:60:1 ───
2020-03-20T12:34:41.5928104Z     │
2020-03-20T12:34:41.5928610Z  60 │ [`librustc_traits`][librustc_traits].
2020-03-20T12:34:41.5929201Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5929201Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5929701Z     │
2020-03-20T12:34:41.5929916Z 
2020-03-20T12:34:41.5930980Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc/traits/mod.rs"
2020-03-20T12:34:41.5931697Z     ┌── traits/goals-and-clauses.md:41:1 ───
2020-03-20T12:34:41.5932076Z     │
2020-03-20T12:34:41.5932076Z     │
2020-03-20T12:34:41.5932691Z  41 │ [`librustc/traits/mod.rs`][traits_mod] in rustc, and in
2020-03-20T12:34:41.5933828Z     │
2020-03-20T12:34:41.5934044Z 
2020-03-20T12:34:41.5934736Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs"
2020-03-20T12:34:41.5935213Z 
2020-03-20T12:34:41.5935213Z 
2020-03-20T12:34:41.5935669Z     ┌── traits/goals-and-clauses.md:42:1 ───
2020-03-20T12:34:41.5936142Z     │
2020-03-20T12:34:41.5936661Z  42 │ [`chalk-ir/src/lib.rs`][chalk_ir] in chalk.
2020-03-20T12:34:41.5937279Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5937759Z     │
2020-03-20T12:34:41.5937973Z 
2020-03-20T12:34:41.5938678Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc/ty/sty.rs"
2020-03-20T12:34:41.5939526Z     ┌── traits/associated-types.md:97:22 ───
2020-03-20T12:34:41.5940009Z     │
2020-03-20T12:34:41.5940009Z     │
2020-03-20T12:34:41.5940496Z  97 │ variant, declared in [`librustc/ty/sty.rs`][sty]. In chalk, we use an
2020-03-20T12:34:41.5941571Z     │
2020-03-20T12:34:41.5941684Z 
2020-03-20T12:34:41.5942327Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs"
2020-03-20T12:34:41.5942718Z 
---
2020-03-20T12:34:41.5945992Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md"
2020-03-20T12:34:41.5946475Z 
2020-03-20T12:34:41.5946923Z     ┌── traits/implied-bounds.md:89:34 ───
2020-03-20T12:34:41.5947395Z     │
2020-03-20T12:34:41.5947991Z  89 │ types. The full RFC can be found [here][RFC]. We'll give here a brief view
2020-03-20T12:34:41.5949297Z     │
2020-03-20T12:34:41.5949512Z 
2020-03-20T12:34:41.5949512Z 
2020-03-20T12:34:41.5950160Z error: The server responded with 429 Too Many Requests for "***/pull/43786"
2020-03-20T12:34:41.5950957Z      ┌── traits/implied-bounds.md:313:8 ───
2020-03-20T12:34:41.5951430Z      │
2020-03-20T12:34:41.5951430Z      │
2020-03-20T12:34:41.5952022Z  313 │ can be [exploited][bug]. Indeed, suppose that we define the following
2020-03-20T12:34:41.5953197Z      │
2020-03-20T12:34:41.5953414Z 
2020-03-20T12:34:41.5953414Z 
2020-03-20T12:34:41.5954127Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify"
2020-03-20T12:34:41.5955351Z     ┌── traits/lowering-module.md:25:27 ───
2020-03-20T12:34:41.5955705Z     │
2020-03-20T12:34:41.5956312Z  25 │ Unit tests are located in [`src/test/ui/chalkify`][chalkify]. A good
2020-03-20T12:34:41.5957535Z     │                           ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5957535Z     │                           ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5958095Z     │
2020-03-20T12:34:41.5958311Z 
2020-03-20T12:34:41.5959085Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify/lower_impl.rs"
2020-03-20T12:34:41.5959931Z     ┌── traits/lowering-module.md:26:17 ───
2020-03-20T12:34:41.5960299Z     │
2020-03-20T12:34:41.5960937Z  26 │ example test is [the `lower_impl` test][lower_impl]. At the time of
2020-03-20T12:34:41.5961653Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5961653Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5962157Z     │
2020-03-20T12:34:41.5962373Z 
2020-03-20T12:34:41.5963404Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify/lower_impl.stderr"
2020-03-20T12:34:41.5964271Z     ┌── traits/lowering-module.md:49:38 ───
2020-03-20T12:34:41.5964774Z     │
2020-03-20T12:34:41.5965340Z  49 │ `#[rustc_dump_program_clauses]`, but [the stderr file] contains
2020-03-20T12:34:41.5966072Z     │                                      ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.5969515Z  31 │ [`chalk/chalk-solve/src/clauses.rs`][chalk_rules]. They are also ported in
2020-03-20T12:34:41.5970168Z     │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.5970779Z     │
2020-03-20T12:34:41.5971008Z 
2020-03-20T12:34:41.5971811Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-03-20T12:34:41.5972681Z     ┌── traits/lowering-rules.md:32:14 ───
2020-03-20T12:34:41.5973122Z     │
2020-03-20T12:34:41.5973649Z  32 │ rustc in the [`librustc_traits`][librustc_traits] crate.
2020-03-20T12:34:41.5974259Z     │              ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.5979691Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs"
2020-03-20T12:34:41.5980068Z 
2020-03-20T12:34:41.5980534Z     ┌── traits/wf.md:16:36 ───
2020-03-20T12:34:41.5980998Z     │
2020-03-20T12:34:41.5981638Z  16 │ an extended set of examples in the [`chalk/tests/test/wf_lowering.rs`][wf_test] submodule.
2020-03-20T12:34:41.5982963Z     │
2020-03-20T12:34:41.5983177Z 
2020-03-20T12:34:41.5983177Z 
2020-03-20T12:34:41.5983819Z error: The server responded with 429 Too Many Requests for "***/pull/47828"
2020-03-20T12:34:41.5984574Z      ┌── codegen/updating-llvm.md:129:1 ───
2020-03-20T12:34:41.5985063Z      │
2020-03-20T12:34:41.5985063Z      │
2020-03-20T12:34:41.5985647Z  129 │ [#47828](***/pull/47828)
2020-03-20T12:34:41.5986957Z      │
2020-03-20T12:34:41.5987183Z 
2020-03-20T12:34:41.5987183Z 
2020-03-20T12:34:41.5987823Z error: The server responded with 429 Too Many Requests for "***/pull/62474"
2020-03-20T12:34:41.5988598Z      ┌── codegen/updating-llvm.md:130:1 ───
2020-03-20T12:34:41.5988969Z      │
2020-03-20T12:34:41.5988969Z      │
2020-03-20T12:34:41.5989545Z  130 │ [#62474](***/pull/62474)
2020-03-20T12:34:41.5990847Z      │
2020-03-20T12:34:41.5991065Z 
2020-03-20T12:34:41.5991065Z 
2020-03-20T12:34:41.5991704Z error: The server responded with 429 Too Many Requests for "***/pull/62592"
2020-03-20T12:34:41.5992468Z      ┌── codegen/updating-llvm.md:131:1 ───
2020-03-20T12:34:41.5992957Z      │
2020-03-20T12:34:41.5992957Z      │
2020-03-20T12:34:41.5993606Z  131 │ [#62592](***/pull/62592). Note that sometimes it's
2020-03-20T12:34:41.5994964Z      │
2020-03-20T12:34:41.5995179Z 
2020-03-20T12:34:41.5995785Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"
2020-03-20T12:34:41.5996120Z 
---
2020-03-20T12:34:41.5999497Z error: The server responded with 429 Too Many Requests for "https://github.com/CraneStation/cranelift"
2020-03-20T12:34:41.5999784Z 
2020-03-20T12:34:41.6000258Z    ┌── codegen/backend-agnostic.md:4:1 ───
2020-03-20T12:34:41.6000694Z    │
2020-03-20T12:34:41.6001319Z  4 │ [Cranelift][cranelift]). To this end, `librustc_codegen_ssa` provides an
2020-03-20T12:34:41.6002478Z    │
2020-03-20T12:34:41.6002805Z 
2020-03-20T12:34:41.6002805Z 
2020-03-20T12:34:41.6003477Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-20T12:34:41.6004487Z     ┌── profile-guided-optimization.md:65:33 ───
2020-03-20T12:34:41.6004984Z     │
2020-03-20T12:34:41.6004984Z     │
2020-03-20T12:34:41.6005600Z  65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
2020-03-20T12:34:41.6006900Z     │
2020-03-20T12:34:41.6007039Z 
2020-03-20T12:34:41.6007039Z 
2020-03-20T12:34:41.6007870Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
2020-03-20T12:34:41.6008764Z     ┌── profile-guided-optimization.md:77:25 ───
2020-03-20T12:34:41.6009256Z     │
2020-03-20T12:34:41.6009256Z     │
2020-03-20T12:34:41.6009884Z  77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
2020-03-20T12:34:41.6011244Z     │
2020-03-20T12:34:41.6011457Z 
2020-03-20T12:34:41.6011457Z 
2020-03-20T12:34:41.6012179Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-20T12:34:41.6013024Z     ┌── profile-guided-optimization.md:88:11 ───
2020-03-20T12:34:41.6013519Z     │
2020-03-20T12:34:41.6013519Z     │
2020-03-20T12:34:41.6014116Z  88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
2020-03-20T12:34:41.6015299Z     │
2020-03-20T12:34:41.6015514Z 
2020-03-20T12:34:41.6016223Z error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"
2020-03-20T12:34:41.6016592Z 
2020-03-20T12:34:41.6016592Z 
2020-03-20T12:34:41.6017133Z      ┌── profile-guided-optimization.md:109:1 ───
2020-03-20T12:34:41.6017659Z      │
2020-03-20T12:34:41.6018268Z  109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
2020-03-20T12:34:41.6018933Z      │ ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6019432Z      │
2020-03-20T12:34:41.6019647Z 
2020-03-20T12:34:41.6020359Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
2020-03-20T12:34:41.6021190Z      ┌── profile-guided-optimization.md:122:4 ───
2020-03-20T12:34:41.6021705Z      │
2020-03-20T12:34:41.6021705Z      │
2020-03-20T12:34:41.6022320Z  122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
2020-03-20T12:34:41.6023499Z      │
2020-03-20T12:34:41.6023713Z 
2020-03-20T12:34:41.6023713Z 
2020-03-20T12:34:41.6024457Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
2020-03-20T12:34:41.6025340Z      ┌── profile-guided-optimization.md:123:17 ───
2020-03-20T12:34:41.6025834Z      │
2020-03-20T12:34:41.6026458Z  123 │ There is also a [codegen test][codegen-test] that checks that some expected
2020-03-20T12:34:41.6027157Z      │                 ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.6044997Z  24 │ 1. The sanitizer runtime libraries are part of the [compiler-rt] project, and
2020-03-20T12:34:41.6045947Z     │                                                    ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6046557Z     │
2020-03-20T12:34:41.6046809Z 
2020-03-20T12:34:41.6047712Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/native.rs"
2020-03-20T12:34:41.6048787Z     ┌── sanitizers.md:25:4 ───
2020-03-20T12:34:41.6049258Z     │
2020-03-20T12:34:41.6049258Z     │
2020-03-20T12:34:41.6049888Z  25 │    [will be built as an LLVM subproject][sanitizer-build] when enabled in
2020-03-20T12:34:41.6051065Z     │
2020-03-20T12:34:41.6051311Z 
2020-03-20T12:34:41.6051311Z 
2020-03-20T12:34:41.6052195Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/compile.rs"
2020-03-20T12:34:41.6053167Z     ┌── sanitizers.md:33:21 ───
2020-03-20T12:34:41.6053634Z     │
2020-03-20T12:34:41.6054231Z  33 │    The runtimes are [placed into target libdir][sanitizer-copy].
2020-03-20T12:34:41.6054953Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6054953Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6055497Z     │
2020-03-20T12:34:41.6055730Z 
2020-03-20T12:34:41.6056562Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_llvm/declare.rs"
2020-03-20T12:34:41.6057488Z     ┌── sanitizers.md:36:4 ───
2020-03-20T12:34:41.6057956Z     │
2020-03-20T12:34:41.6057956Z     │
2020-03-20T12:34:41.6058573Z  36 │    [marked][sanitizer-attribute] with `SanitizeAddress`, `SanitizeMemory`, or
2020-03-20T12:34:41.6059772Z     │
2020-03-20T12:34:41.6060005Z 
2020-03-20T12:34:41.6060005Z 
2020-03-20T12:34:41.6060839Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_ssa/back/write.rs"
2020-03-20T12:34:41.6061769Z     ┌── sanitizers.md:41:54 ───
2020-03-20T12:34:41.6062245Z     │
2020-03-20T12:34:41.6062854Z  41 │ 3. The LLVM IR generated by rustc is instrumented by [dedicated LLVM
2020-03-20T12:34:41.6063802Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6063802Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6064409Z     │
2020-03-20T12:34:41.6064642Z 
2020-03-20T12:34:41.6065578Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/librustc_codegen_ssa/back/link.rs"
2020-03-20T12:34:41.6066574Z     ┌── sanitizers.md:46:4 ───
2020-03-20T12:34:41.6067044Z     │
2020-03-20T12:34:41.6067674Z  46 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
2020-03-20T12:34:41.6069539Z     │    ^ Server responded with 429 Too Many Requests
---
2020-03-20T12:34:41.6075418Z  45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
2020-03-20T12:34:41.6076058Z     │                               ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6076462Z     │
2020-03-20T12:34:41.6076573Z 
2020-03-20T12:34:41.6077281Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-20T12:34:41.6077964Z     ┌── debugging-support-in-rustc.md:80:1 ───
2020-03-20T12:34:41.6078423Z     │
2020-03-20T12:34:41.6078423Z     │
2020-03-20T12:34:41.6078993Z  80 │ [This comment by Tom](***c-guide/pull/316#discussion_r284027340)
2020-03-20T12:34:41.6080304Z     │
2020-03-20T12:34:41.6080416Z 
2020-03-20T12:34:41.6080416Z 
2020-03-20T12:34:41.6081007Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-20T12:34:41.6081683Z     ┌── debugging-support-in-rustc.md:86:1 ───
2020-03-20T12:34:41.6082050Z     │
2020-03-20T12:34:41.6082050Z     │
2020-03-20T12:34:41.6082849Z  86 │ [This question by Aman](***c-guide/pull/316#discussion_r285401353)
2020-03-20T12:34:41.6084179Z     │
2020-03-20T12:34:41.6084292Z 
2020-03-20T12:34:41.6084828Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"
2020-03-20T12:34:41.6085138Z 
---
2020-03-20T12:34:41.6088561Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"
2020-03-20T12:34:41.6088891Z 
2020-03-20T12:34:41.6089307Z      ┌── debugging-support-in-rustc.md:121:43 ───
2020-03-20T12:34:41.6089682Z      │
2020-03-20T12:34:41.6090348Z  121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
2020-03-20T12:34:41.6091471Z      │
2020-03-20T12:34:41.6091585Z 
2020-03-20T12:34:41.6091585Z 
2020-03-20T12:34:41.6092191Z error: The server responded with 429 Too Many Requests for "***/issues/34457"
2020-03-20T12:34:41.6092855Z      ┌── debugging-support-in-rustc.md:174:17 ───
2020-03-20T12:34:41.6093247Z      │
2020-03-20T12:34:41.6093247Z      │
2020-03-20T12:34:41.6093730Z  174 │ Tracking issue: [***/issues/34457]
2020-03-20T12:34:41.6094662Z      │
2020-03-20T12:34:41.6094776Z 
2020-03-20T12:34:41.6094776Z 
2020-03-20T12:34:41.6095356Z error: The server responded with 429 Too Many Requests for "***/issues/33014"
2020-03-20T12:34:41.6096032Z      ┌── debugging-support-in-rustc.md:229:18 ───
2020-03-20T12:34:41.6096406Z      │
2020-03-20T12:34:41.6096406Z      │
2020-03-20T12:34:41.6096910Z  229 │ Issue on Github: [***/issues/33014]
2020-03-20T12:34:41.6097948Z      │
2020-03-20T12:34:41.6098146Z 
2020-03-20T12:34:41.6098722Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"
2020-03-20T12:34:41.6099042Z 
2020-03-20T12:34:41.6099042Z 
2020-03-20T12:34:41.6099454Z      ┌── debugging-support-in-rustc.md:265:6 ───
2020-03-20T12:34:41.6099844Z      │
2020-03-20T12:34:41.6100282Z  265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
2020-03-20T12:34:41.6100803Z      │      ^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6101184Z      │
2020-03-20T12:34:41.6101298Z 
2020-03-20T12:34:41.6101889Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-20T12:34:41.6102574Z      ┌── debugging-support-in-rustc.md:279:1 ───
2020-03-20T12:34:41.6102949Z      │
2020-03-20T12:34:41.6102949Z      │
2020-03-20T12:34:41.6103545Z  279 │ [Question on Github](***c-guide/pull/316#discussion_r283062536).
2020-03-20T12:34:41.6104841Z      │
2020-03-20T12:34:41.6104969Z 
2020-03-20T12:34:41.6105511Z error: The server responded with 429 Too Many Requests for "https://github.com/nrc/stupid-stats"
2020-03-20T12:34:41.6105819Z 
2020-03-20T12:34:41.6105819Z 
2020-03-20T12:34:41.6106198Z    ┌── appendix/stupid-stats.md:3:48 ───
2020-03-20T12:34:41.6106558Z    │
2020-03-20T12:34:41.6107047Z  3 │ > **Note:** This is a copy of `@nrc`'s amazing [stupid-stats]. You should find
2020-03-20T12:34:41.6108163Z    │
2020-03-20T12:34:41.6108273Z 
2020-03-20T12:34:41.6108273Z 
2020-03-20T12:34:41.6108893Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-20T12:34:41.6109572Z     ┌── appendix/stupid-stats.md:64:43 ───
2020-03-20T12:34:41.6109920Z     │
2020-03-20T12:34:41.6109920Z     │
2020-03-20T12:34:41.6110560Z  64 │ The code for these first two phases is in [libsyntax](***/tree/master/src/libsyntax).
2020-03-20T12:34:41.6111974Z     │
2020-03-20T12:34:41.6112102Z 
2020-03-20T12:34:41.6112102Z 
2020-03-20T12:34:41.6112719Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc"
2020-03-20T12:34:41.6113387Z     ┌── appendix/stupid-stats.md:76:25 ───
2020-03-20T12:34:41.6113733Z     │
2020-03-20T12:34:41.6113733Z     │
2020-03-20T12:34:41.6114298Z  76 │ The analysis code is in [librustc](***/tree/master/src/librustc)
2020-03-20T12:34:41.6115587Z     │
2020-03-20T12:34:41.6115708Z 
2020-03-20T12:34:41.6115708Z 
2020-03-20T12:34:41.6116352Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_llvm"
2020-03-20T12:34:41.6117022Z     ┌── appendix/stupid-stats.md:87:40 ───
2020-03-20T12:34:41.6117387Z     │
2020-03-20T12:34:41.6117387Z     │
2020-03-20T12:34:41.6118027Z  87 │ interface between LLVM and rustc is in [librustc_llvm](***/tree/master/src/librustc_llvm).
2020-03-20T12:34:41.6119466Z     │
2020-03-20T12:34:41.6119577Z 
2020-03-20T12:34:41.6120172Z error: The server responded with 429 Too Many Requests for "https://github.com/nick29581/stupid-stats/blob/master/src"
2020-03-20T12:34:41.6120528Z 
---
2020-03-20T12:34:41.6126154Z  205 │ [multirust](https://github.com/brson/multirust) to get around all the PATH stuff
2020-03-20T12:34:41.6126847Z      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-03-20T12:34:41.6127284Z      │
2020-03-20T12:34:41.6127400Z 
2020-03-20T12:34:41.6127997Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-20T12:34:41.6128717Z      ┌── appendix/stupid-stats.md:402:33 ───
2020-03-20T12:34:41.6129043Z      │
2020-03-20T12:34:41.6129043Z      │
2020-03-20T12:34:41.6129793Z  402 │ do, let me know in a comment or [GitHub issue](***/issues).
2020-03-20T12:34:41.6131052Z      │
2020-03-20T12:34:41.6131167Z 
2020-03-20T12:34:41.6131786Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html"
2020-03-20T12:34:41.6132136Z 
2020-03-20T12:34:41.6132136Z 
2020-03-20T12:34:41.6132526Z     ┌── appendix/code-index.md:11:90 ───
2020-03-20T12:34:41.6132867Z     │
2020-03-20T12:34:41.6133663Z  11 │ `ast::Crate` | struct | A syntax-level representation of a parsed crate | [The parser] | [src/libsyntax/ast.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html)
2020-03-20T12:34:41.6134909Z     │                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 404 Not Found
2020-03-20T12:34:41.6135666Z     │
2020-03-20T12:34:41.6135786Z 
2020-03-20T12:34:41.6136378Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustdoc/core.rs"
2020-03-20T12:34:41.6137012Z     ┌── appendix/code-index.md:15:131 ───
2020-03-20T12:34:41.6137330Z     │
2020-03-20T12:34:41.6137330Z     │
2020-03-20T12:34:41.6138173Z  15 │ `DocContext` | struct | A state container used by rustdoc when crawling through a crate to gather its documentation | [Rustdoc] | [src/librustdoc/core.rs](***/blob/master/src/librustdoc/core.rs)
2020-03-20T12:34:41.6140110Z     │
2020-03-20T12:34:41.6140215Z 
2020-03-20T12:34:41.6140813Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/node_id/struct.NodeId.html"
2020-03-20T12:34:41.6141151Z 
---
2020-03-20T12:55:16.5670956Z 
2020-03-20T12:55:16.5671403Z If you do intend to update 'miri', please check the error messages above and
2020-03-20T12:55:16.5671694Z commit another update.
2020-03-20T12:55:16.5671824Z 
2020-03-20T12:55:16.5672247Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2020-03-20T12:55:16.5672797Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2020-03-20T12:55:16.5673074Z proper steps.
2020-03-20T12:55:16.5677800Z Build completed unsuccessfully in 0:00:01
2020-03-20T12:55:16.5717572Z == clock drift check ==
2020-03-20T12:55:16.5729649Z   local time: Fri Mar 20 12:55:16 UTC 2020
2020-03-20T12:55:16.6806063Z   network time: Fri, 20 Mar 2020 12:55:16 GMT
2020-03-20T12:55:16.6806063Z   network time: Fri, 20 Mar 2020 12:55:16 GMT
2020-03-20T12:55:16.6812041Z == end clock drift check ==
2020-03-20T12:55:17.4688278Z 
2020-03-20T12:55:17.4764967Z ##[error]Bash exited with code '1'.
2020-03-20T12:55:17.4777292Z ##[section]Finishing: Run build
2020-03-20T12:55:17.4831055Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T12:55:17.4835806Z Task         : Get sources
2020-03-20T12:55:17.4836155Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T12:55:17.4836466Z Version      : 1.0.0
2020-03-20T12:55:17.4836682Z Author       : Microsoft
2020-03-20T12:55:17.4836682Z Author       : Microsoft
2020-03-20T12:55:17.4837035Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T12:55:17.4837430Z ==============================================================================
2020-03-20T12:55:17.7939328Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T12:55:17.7985312Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T12:55:17.8067386Z Cleaning up task key
2020-03-20T12:55:17.8068623Z Start cleaning up orphan processes.
2020-03-20T12:55:17.8246923Z Terminate orphan process: pid (3782) (python)
2020-03-20T12:55:17.8479388Z ##[section]Finishing: Finalize Job
