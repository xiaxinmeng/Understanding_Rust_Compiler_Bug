plain
2020-03-19T19:42:16.8668817Z ========================== Starting Command Output ===========================
2020-03-19T19:42:16.8673958Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a59602d3-3896-4b89-93e8-52b3666e7fe9.sh
2020-03-19T19:42:16.8674403Z 
2020-03-19T19:42:16.8678619Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T19:42:16.8693598Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70077/merge to s
2020-03-19T19:42:16.8695952Z Task         : Get sources
2020-03-19T19:42:16.8696170Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T19:42:16.8696382Z Version      : 1.0.0
2020-03-19T19:42:16.8696526Z Author       : Microsoft
---
2020-03-19T19:42:17.8742166Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T19:42:17.8746459Z ##[command]git config gc.auto 0
2020-03-19T19:42:17.8749352Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T19:42:17.8752032Z ##[command]git config --get-all http.proxy
2020-03-19T19:42:17.8756864Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70077/merge:refs/remotes/pull/70077/merge
---
2020-03-19T21:08:18.0260243Z     Finished release [optimized] target(s) in 7m 50s
2020-03-19T21:08:18.0464999Z Testing rustbook src/doc/book
2020-03-19T21:08:30.1706240Z Error: Rustdoc returned an error: 
2020-03-19T21:08:30.1706449Z running 4 tests
2020-03-19T21:08:30.1707361Z test /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 15) ... ignored
2020-03-19T21:08:30.1708062Z test /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 35) ... ignored
2020-03-19T21:08:30.1708696Z test /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) ... FAILED
2020-03-19T21:08:30.1709314Z test /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 62) ... ok
2020-03-19T21:08:30.1709669Z failures:
2020-03-19T21:08:30.1709748Z 
2020-03-19T21:08:30.1709748Z 
2020-03-19T21:08:30.1710328Z ---- /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52) stdout ----
2020-03-19T21:08:30.1710872Z error: Cannot declare a non-inline module inside a block unless it has a path attribute
2020-03-19T21:08:30.1711319Z  --> /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md:53:1
2020-03-19T21:08:30.1711889Z 3 | pub mod hosting;
2020-03-19T21:08:30.1712037Z   | ^^^^^^^^^^^^^^^^
2020-03-19T21:08:30.1712143Z 
2020-03-19T21:08:30.1712304Z error: aborting due to previous error
2020-03-19T21:08:30.1712304Z error: aborting due to previous error
2020-03-19T21:08:30.1712437Z 
2020-03-19T21:08:30.1712698Z Couldn't compile the test.
2020-03-19T21:08:30.1712814Z 
2020-03-19T21:08:30.1712935Z failures:
2020-03-19T21:08:30.1713444Z     /tmp/mdbook-0n2Jdi/ch07-05-separating-modules-into-different-files.md - _::Separating_Modules_into_Different_Files (line 52)
2020-03-19T21:08:30.1713927Z test result: FAILED. 1 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
2020-03-19T21:08:30.1714133Z 
2020-03-19T21:08:30.1714210Z 
2020-03-19T21:08:30.1714290Z 
---
2020-03-19T21:09:30.3705547Z  finished in 1.053
2020-03-19T21:09:30.3712127Z Testing rustbook src/doc/edition-guide
2020-03-19T21:09:37.3662933Z  finished in 6.993
2020-03-19T21:11:04.6447700Z Timeout for link `http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz`
2020-03-19T21:11:04.6461556Z error: The server responded with 429 Too Many Requests for "***/pull/51587"
2020-03-19T21:11:04.6462740Z     ┌── walkthrough.md:55:46 ───
2020-03-19T21:11:04.6463362Z     │
2020-03-19T21:11:04.6463362Z     │
2020-03-19T21:11:04.6464171Z  55 │   went [through][impl2] a [number][impl3] of [iterations][impl4].
2020-03-19T21:11:04.6466763Z     │
2020-03-19T21:11:04.6466957Z 
2020-03-19T21:11:04.6466957Z 
2020-03-19T21:11:04.6468005Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-19T21:11:04.6468883Z     ┌── walkthrough.md:57:3 ───
2020-03-19T21:11:04.6469185Z     │
2020-03-19T21:11:04.6469185Z     │
2020-03-19T21:11:04.6469610Z  57 │   [propose to stabilize it][merge]. If there is consensus, this is done.
2020-03-19T21:11:04.6470447Z     │
2020-03-19T21:11:04.6470548Z 
2020-03-19T21:11:04.6471035Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-19T21:11:04.6471320Z 
2020-03-19T21:11:04.6471320Z 
2020-03-19T21:11:04.6471652Z     ┌── walkthrough.md:74:64 ───
2020-03-19T21:11:04.6471955Z     │
2020-03-19T21:11:04.6472428Z  74 │ > You can find the official guidelines for when to open an RFC [here][rfcwhen].
2020-03-19T21:11:04.6473482Z     │
2020-03-19T21:11:04.6473603Z 
2020-03-19T21:11:04.6474089Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-19T21:11:04.6474355Z 
2020-03-19T21:11:04.6474355Z 
2020-03-19T21:11:04.6474972Z     ┌── walkthrough.md:83:1 ───
2020-03-19T21:11:04.6475313Z     │
2020-03-19T21:11:04.6475758Z  83 │ [rust-lang/rfcs](https://github.com/rust-lang/rfcs) repo on GitHub. You can
2020-03-19T21:11:04.6476975Z     │
2020-03-19T21:11:04.6477074Z 
2020-03-19T21:11:04.6477562Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs"
2020-03-19T21:11:04.6477829Z 
2020-03-19T21:11:04.6477829Z 
2020-03-19T21:11:04.6478267Z     ┌── walkthrough.md:85:1 ───
2020-03-19T21:11:04.6512723Z     │
2020-03-19T21:11:04.6513438Z  85 │ [README](https://github.com/rust-lang/rfcs#what-the-process-is).
2020-03-19T21:11:04.6514634Z     │
2020-03-19T21:11:04.6515021Z 
2020-03-19T21:11:04.6515620Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2457"
2020-03-19T21:11:04.6515970Z 
2020-03-19T21:11:04.6515970Z 
2020-03-19T21:11:04.6516378Z      ┌── walkthrough.md:107:51 ───
2020-03-19T21:11:04.6516781Z      │
2020-03-19T21:11:04.6517300Z  107 │ ideas, a lot more discussion can happen (e.g. see [this RFC][nonascii] which
2020-03-19T21:11:04.6518839Z      │
2020-03-19T21:11:04.6519038Z 
2020-03-19T21:11:04.6519695Z error: The server responded with 429 Too Many Requests for "***"
2020-03-19T21:11:04.6520038Z 
2020-03-19T21:11:04.6520038Z 
2020-03-19T21:11:04.6520450Z      ┌── walkthrough.md:146:26 ───
2020-03-19T21:11:04.6520844Z      │
2020-03-19T21:11:04.6521347Z  146 │ issue_ is created in the [rust-lang/rust] repo to track progress on the feature
2020-03-19T21:11:04.6521939Z      │                          ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6522380Z      │
2020-03-19T21:11:04.6522567Z 
2020-03-19T21:11:04.6523141Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-19T21:11:04.6523848Z      ┌── walkthrough.md:148:39 ───
2020-03-19T21:11:04.6524231Z      │
2020-03-19T21:11:04.6524231Z      │
2020-03-19T21:11:04.6524736Z  148 │ Here is the tracking issue on for our [`?` macro feature][tracking].
2020-03-19T21:11:04.6526129Z      │
2020-03-19T21:11:04.6526363Z 
2020-03-19T21:11:04.6526917Z error: The server responded with 429 Too Many Requests for "***"
2020-03-19T21:11:04.6527223Z 
2020-03-19T21:11:04.6527223Z 
2020-03-19T21:11:04.6527626Z      ┌── walkthrough.md:156:57 ───
2020-03-19T21:11:04.6528005Z      │
2020-03-19T21:11:04.6528521Z  156 │ To make a change to the compiler, open a PR against the [rust-lang/rust] repo.
2020-03-19T21:11:04.6529174Z      │                                                         ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6529638Z      │
2020-03-19T21:11:04.6529848Z 
2020-03-19T21:11:04.6530413Z error: The server responded with 429 Too Many Requests for "***/pull/47732"
2020-03-19T21:11:04.6531116Z      ┌── walkthrough.md:167:58 ───
2020-03-19T21:11:04.6531501Z      │
2020-03-19T21:11:04.6531999Z  167 │ macro expansion in the compiler. Personally, I find that [improving the
2020-03-19T21:11:04.6533215Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6533215Z      │                                                          ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6533797Z      │
2020-03-19T21:11:04.6534059Z 
2020-03-19T21:11:04.6534739Z error: The server responded with 429 Too Many Requests for "***"
2020-03-19T21:11:04.6535078Z 
2020-03-19T21:11:04.6535587Z      ┌── walkthrough.md:181:27 ───
2020-03-19T21:11:04.6536061Z      │
2020-03-19T21:11:04.6536667Z  181 │ When you open a PR on the [rust-lang/rust], a bot will assign your PR to a
2020-03-19T21:11:04.6539550Z      │
2020-03-19T21:11:04.6539661Z 
2020-03-19T21:11:04.6539661Z 
2020-03-19T21:11:04.6540238Z error: The server responded with 429 Too Many Requests for "***/pull/51587"
2020-03-19T21:11:04.6540996Z      ┌── walkthrough.md:229:50 ───
2020-03-19T21:11:04.6541347Z      │
2020-03-19T21:11:04.6541347Z      │
2020-03-19T21:11:04.6541799Z  229 │ original implementation: [1][impl2], [2][impl3], [3][impl4].
2020-03-19T21:11:04.6542975Z      │
2020-03-19T21:11:04.6543085Z 
2020-03-19T21:11:04.6543085Z 
2020-03-19T21:11:04.6543674Z error: The server responded with 429 Too Many Requests for "***/issues/51934"
2020-03-19T21:11:04.6544274Z      ┌── walkthrough.md:237:32 ───
2020-03-19T21:11:04.6544607Z      │
2020-03-19T21:11:04.6545009Z  237 │   from the original RFC required [another
2020-03-19T21:11:04.6545474Z      │ ╭────────────────────────────────^
2020-03-19T21:11:04.6545474Z      │ ╭────────────────────────────────^
2020-03-19T21:11:04.6545996Z  238 │ │ FCP](***/issues/51934).
2020-03-19T21:11:04.6547093Z      │
2020-03-19T21:11:04.6547193Z 
2020-03-19T21:11:04.6547193Z 
2020-03-19T21:11:04.6547812Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-19T21:11:04.6548364Z      ┌── walkthrough.md:243:1 ───
2020-03-19T21:11:04.6548716Z      │
2020-03-19T21:11:04.6548716Z      │
2020-03-19T21:11:04.6549064Z  243 │ [moved to stabilize it][stabilizefcp].
2020-03-19T21:11:04.6549886Z      │
2020-03-19T21:11:04.6549987Z 
2020-03-19T21:11:04.6549987Z 
2020-03-19T21:11:04.6550516Z error: The server responded with 429 Too Many Requests for "***/issues/48075"
2020-03-19T21:11:04.6551124Z      ┌── walkthrough.md:253:45 ───
2020-03-19T21:11:04.6551431Z      │
2020-03-19T21:11:04.6551431Z      │
2020-03-19T21:11:04.6551831Z  253 │ The stabilization report for our feature is [here][stabrep].
2020-03-19T21:11:04.6552833Z      │
2020-03-19T21:11:04.6553039Z 
2020-03-19T21:11:04.6553039Z 
2020-03-19T21:11:04.6553584Z error: The server responded with 429 Too Many Requests for "***/pull/56245"
2020-03-19T21:11:04.6554179Z      ┌── walkthrough.md:257:13 ───
2020-03-19T21:11:04.6554490Z      │
2020-03-19T21:11:04.6554490Z      │
2020-03-19T21:11:04.6555156Z  257 │ After this, [a PR is made][stab] to remove the feature gate, enabling the feature by
2020-03-19T21:11:04.6556088Z      │
2020-03-19T21:11:04.6556187Z 
2020-03-19T21:11:04.6556187Z 
2020-03-19T21:11:04.6556751Z error: The server responded with 429 Too Many Requests for "***/blob/master/RELEASES.md"
2020-03-19T21:11:04.6557366Z      ┌── walkthrough.md:258:55 ───
2020-03-19T21:11:04.6557673Z      │
2020-03-19T21:11:04.6557673Z      │
2020-03-19T21:11:04.6558168Z  258 │ default (on the 2018 edition). A note is added to the [Release notes][relnotes]
2020-03-19T21:11:04.6559198Z      │
2020-03-19T21:11:04.6559341Z 
2020-03-19T21:11:04.6559960Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md"
2020-03-19T21:11:04.6560307Z 
---
2020-03-19T21:11:04.6667384Z  81 │ example of how such an issue should look can be [found
2020-03-19T21:11:04.6668020Z     │                                                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6668452Z     │
2020-03-19T21:11:04.6668572Z 
2020-03-19T21:11:04.6669379Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-19T21:11:04.6670115Z      ┌── bug-fix-procedure.md:235:65 ───
2020-03-19T21:11:04.6670482Z      │
2020-03-19T21:11:04.6670941Z  235 │ The first reference you will likely find is the lint definition [in
2020-03-19T21:11:04.6671620Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6671620Z      │                                                                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6672097Z      │
2020-03-19T21:11:04.6672206Z 
2020-03-19T21:11:04.6672967Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc/lint/builtin.rs"
2020-03-19T21:11:04.6673705Z      ┌── bug-fix-procedure.md:250:13 ───
2020-03-19T21:11:04.6674052Z      │
2020-03-19T21:11:04.6674052Z      │
2020-03-19T21:11:04.6674509Z  250 │ the file as [part of a `lint_array!`][lintarraysource]; remove it too,
2020-03-19T21:11:04.6675872Z      │
2020-03-19T21:11:04.6675980Z 
2020-03-19T21:11:04.6675980Z 
2020-03-19T21:11:04.6676801Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_lint/lib.rs"
2020-03-19T21:11:04.6677464Z      ┌── bug-fix-procedure.md:254:19 ───
2020-03-19T21:11:04.6677802Z      │
2020-03-19T21:11:04.6678217Z  254 │ Next, you see see [a reference to `OVERLAPPING_INHERENT_IMPLS` in
2020-03-19T21:11:04.6678728Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6678728Z      │                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6679105Z      │
2020-03-19T21:11:04.6679205Z 
2020-03-19T21:11:04.6679927Z error: The server responded with 429 Too Many Requests for "***/blob/085d71c3efe453863739c1fb68fd9bd1beff214f/src/librustc_typeck/coherence/inherent.rs"
2020-03-19T21:11:04.6680629Z      ┌── bug-fix-procedure.md:283:16 ───
2020-03-19T21:11:04.6680961Z      │
2020-03-19T21:11:04.6680961Z      │
2020-03-19T21:11:04.6681385Z  283 │ this case, the [`add_lint` call][addlintsource] looks like this:
2020-03-19T21:11:04.6682236Z      │
2020-03-19T21:11:04.6682335Z 
2020-03-19T21:11:04.6682936Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md"
2020-03-19T21:11:04.6683269Z 
2020-03-19T21:11:04.6683269Z 
2020-03-19T21:11:04.6683635Z     ┌── implementing_new_features.md:56:4 ───
2020-03-19T21:11:04.6683975Z     │
2020-03-19T21:11:04.6684393Z  56 │ We [value the stability of Rust]. Code that works and runs on stable
2020-03-19T21:11:04.6685094Z     │    ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6685444Z     │
2020-03-19T21:11:04.6685531Z 
2020-03-19T21:11:04.6685984Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-19T21:11:04.6686626Z     ┌── stability.md:18:51 ───
2020-03-19T21:11:04.6687006Z     │
2020-03-19T21:11:04.6687451Z  18 │ The `issue` field specifies the associated GitHub [issue number]. This field is
2020-03-19T21:11:04.6688074Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6688074Z     │                                                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6689008Z     │
2020-03-19T21:11:04.6689118Z 
2020-03-19T21:11:04.6690656Z error: The server responded with 429 Too Many Requests for "***/issues/15702"
2020-03-19T21:11:04.6691657Z     ┌── stability.md:31:30 ───
2020-03-19T21:11:04.6692005Z     │
2020-03-19T21:11:04.6692005Z     │
2020-03-19T21:11:04.6692505Z  31 │ Note, however, that due to a [rustc bug], stable items inside unstable modules
2020-03-19T21:11:04.6693525Z     │
2020-03-19T21:11:04.6693634Z 
2020-03-19T21:11:04.6693634Z 
2020-03-19T21:11:04.6694280Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/doc/unstable-book"
2020-03-19T21:11:04.6694964Z     ┌── stabilization_guide.md:17:38 ───
2020-03-19T21:11:04.6695308Z     │
2020-03-19T21:11:04.6695308Z     │
2020-03-19T21:11:04.6695746Z  17 │ in the [`Unstable Book`], located at [`src/doc/unstable-book`].
2020-03-19T21:11:04.6696769Z     │
2020-03-19T21:11:04.6696876Z 
2020-03-19T21:11:04.6697427Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/reference"
2020-03-19T21:11:04.6697723Z 
---
2020-03-19T21:11:04.6701857Z  28 │ - [The Book]: This may or may not need updating, depends.
2020-03-19T21:11:04.6702362Z     │   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6702728Z     │
2020-03-19T21:11:04.6702836Z 
2020-03-19T21:11:04.6703401Z error: The server responded with 429 Too Many Requests for "***-by-example"
2020-03-19T21:11:04.6704032Z     ┌── stabilization_guide.md:35:3 ───
2020-03-19T21:11:04.6704373Z     │
2020-03-19T21:11:04.6704738Z  35 │ - [Rust by Example]: As needed.
2020-03-19T21:11:04.6705229Z     │   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6705229Z     │   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6705579Z     │
2020-03-19T21:11:04.6705685Z 
2020-03-19T21:11:04.6706271Z error: The server responded with 429 Too Many Requests for "***/issues/32409"
2020-03-19T21:11:04.6706881Z     ┌── stabilization_guide.md:97:1 ───
2020-03-19T21:11:04.6707249Z     │
2020-03-19T21:11:04.6707586Z  97 │ [rust-lang/rust#32409]:
2020-03-19T21:11:04.6708020Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6708020Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6708378Z     │
2020-03-19T21:11:04.6708485Z 
2020-03-19T21:11:04.6709067Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc"
2020-03-19T21:11:04.6709394Z 
2020-03-19T21:11:04.6709788Z      ┌── compiler-debugging.md:258:5 ───
2020-03-19T21:11:04.6710136Z      │
2020-03-19T21:11:04.6710609Z  258 │ The [cargo-bisect-rustc][bisect] tool can be used as a quick and easy way to
2020-03-19T21:11:04.6711744Z      │
2020-03-19T21:11:04.6711845Z 
2020-03-19T21:11:04.6712443Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/cargo-bisect-rustc/blob/master/TUTORIAL.md"
2020-03-19T21:11:04.6712798Z 
2020-03-19T21:11:04.6712798Z 
2020-03-19T21:11:04.6713221Z      ┌── compiler-debugging.md:262:31 ───
2020-03-19T21:11:04.6713546Z      │
2020-03-19T21:11:04.6714002Z  262 │ on *why* it was changed.  See [this tutorial][bisect-tutorial] on how to use
2020-03-19T21:11:04.6715052Z      │
2020-03-19T21:11:04.6715154Z 
2020-03-19T21:11:04.6715716Z error: The server responded with 429 Too Many Requests for "https://github.com/kennytm/rustup-toolchain-install-master"
2020-03-19T21:11:04.6716025Z 
2020-03-19T21:11:04.6716025Z 
2020-03-19T21:11:04.6716376Z      ┌── compiler-debugging.md:270:5 ───
2020-03-19T21:11:04.6716717Z      │
2020-03-19T21:11:04.6717165Z  270 │ The [rustup-toolchain-install-master][rtim] tool by kennytm can be used to
2020-03-19T21:11:04.6718008Z      │
2020-03-19T21:11:04.6718109Z 
2020-03-19T21:11:04.6718628Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf"
2020-03-19T21:11:04.6719108Z 
2020-03-19T21:11:04.6719108Z 
2020-03-19T21:11:04.6719464Z    ┌── profiling.md:8:9 ───
2020-03-19T21:11:04.6719779Z    │
2020-03-19T21:11:04.6720474Z  8 │   - The [rustc-perf](https://github.com/rust-lang-nursery/rustc-perf) project makes this easy and can be triggered to run on a PR via the `@rustc-perf` bot.
2020-03-19T21:11:04.6721782Z    │
2020-03-19T21:11:04.6721888Z 
2020-03-19T21:11:04.6722425Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme"
2020-03-19T21:11:04.6722749Z 
2020-03-19T21:11:04.6722749Z 
2020-03-19T21:11:04.6723100Z     ┌── profiling.md:11:35 ───
2020-03-19T21:11:04.6723422Z     │
2020-03-19T21:11:04.6724157Z  11 │   - The `-Zself-profile` flag and [measureme](https://github.com/rust-lang/measureme) tools offer a query-based approach to profiling.
2020-03-19T21:11:04.6725524Z     │
2020-03-19T21:11:04.6725647Z 
2020-03-19T21:11:04.6726264Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/measureme/blob/master/summarize/Readme.md"
2020-03-19T21:11:04.6726614Z 
---
2020-03-19T21:11:04.6736422Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustc-perf/tree/master/collector/benchmarks"
2020-03-19T21:11:04.6736866Z 
2020-03-19T21:11:04.6737247Z     ┌── profiling/with_perf.md:93:14 ───
2020-03-19T21:11:04.6737601Z     │
2020-03-19T21:11:04.6738055Z  93 │ are found in [the `collector/benchmarks` directory][dir]. So let's go
2020-03-19T21:11:04.6738986Z     │
2020-03-19T21:11:04.6739092Z 
2020-03-19T21:11:04.6739638Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-19T21:11:04.6739942Z 
2020-03-19T21:11:04.6739942Z 
2020-03-19T21:11:04.6740334Z      ┌── profiling/with_perf.md:137:45 ───
2020-03-19T21:11:04.6740679Z      │
2020-03-19T21:11:04.6741342Z  137 │ helpful. For more detailed examination, the [`perf-focus` tool][pf]
2020-03-19T21:11:04.6742402Z      │
2020-03-19T21:11:04.6742511Z 
2020-03-19T21:11:04.6743083Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/perf-focus"
2020-03-19T21:11:04.6743388Z 
2020-03-19T21:11:04.6743388Z 
2020-03-19T21:11:04.6743764Z      ┌── profiling/with_perf.md:161:38 ───
2020-03-19T21:11:04.6744103Z      │
2020-03-19T21:11:04.6744576Z  161 │ about it. For this, I personally use [perf focus][pf]. It's a kind of
2020-03-19T21:11:04.6746351Z      │
2020-03-19T21:11:04.6746476Z 
2020-03-19T21:11:04.6747044Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/fmt-rfcs"
2020-03-19T21:11:04.6747352Z 
2020-03-19T21:11:04.6747352Z 
2020-03-19T21:11:04.6747720Z     ┌── conventions.md:10:36 ───
2020-03-19T21:11:04.6748071Z     │
2020-03-19T21:11:04.6748524Z  10 │ rustc is slowly moving towards the [Rust standard coding style][fmt];
2020-03-19T21:11:04.6749560Z     │
2020-03-19T21:11:04.6749666Z 
2020-03-19T21:11:04.6749666Z 
2020-03-19T21:11:04.6750427Z error: The server responded with 429 Too Many Requests for "***/blob/659994627234ce7d95a1a52ad8756ce661059adf/src/tools/tidy/src/deps.rs"
2020-03-19T21:11:04.6751128Z     ┌── crates-io.md:19:23 ───
2020-03-19T21:11:04.6751451Z     │
2020-03-19T21:11:04.6751451Z     │
2020-03-19T21:11:04.6751923Z  19 │ The `tidy` tool has a [whitelist] of crates that are allowed. To add a
2020-03-19T21:11:04.6752878Z     │
2020-03-19T21:11:04.6752986Z 
2020-03-19T21:11:04.6753625Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rls"
2020-03-19T21:11:04.6753920Z 
---
2020-03-19T21:11:04.6759009Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/rustfix"
2020-03-19T21:11:04.6759319Z 
2020-03-19T21:11:04.6759694Z     ┌── diagnostics.md:82:18 ───
2020-03-19T21:11:04.6760021Z     │
2020-03-19T21:11:04.6760397Z  82 │ Server][rls] and [`rustfix`][rustfix].
2020-03-19T21:11:04.6761295Z     │
2020-03-19T21:11:04.6761401Z 
2020-03-19T21:11:04.6763263Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/triagebot/wiki/Assignment"
2020-03-19T21:11:04.6763624Z 
---
2020-03-19T21:11:04.6779323Z  57 │ [rustbot] a [`ping`] command with the name of the ICE-breakers
2020-03-19T21:11:04.6779813Z     │             ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6780168Z     │
2020-03-19T21:11:04.6780267Z 
2020-03-19T21:11:04.6780870Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-Cleanup-Crew"
2020-03-19T21:11:04.6781488Z    ┌── ice-breaker/cleanup-crew.md:3:19 ───
2020-03-19T21:11:04.6781927Z    │
2020-03-19T21:11:04.6782305Z  3 │ **Github Label:** [ICEBreaker-Cleanup-Crew]
2020-03-19T21:11:04.6782803Z    │                   ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6791775Z  80 │ To learn to use [cargo-bisect-rustc], check out [this blog
2020-03-19T21:11:04.6792281Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6792626Z     │
2020-03-19T21:11:04.6792725Z 
2020-03-19T21:11:04.6793247Z error: The server responded with 429 Too Many Requests for "***/"
2020-03-19T21:11:04.6793825Z      ┌── ice-breaker/cleanup-crew.md:102:36 ───
2020-03-19T21:11:04.6794165Z      │
2020-03-19T21:11:04.6794574Z  102 │ 1. Go to an update checkout of the [rust-lang/rust] repository
2020-03-19T21:11:04.6795255Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6795255Z      │                                    ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6795663Z      │
2020-03-19T21:11:04.6795767Z 
2020-03-19T21:11:04.6796321Z error: The server responded with 429 Too Many Requests for "***/labels/ICEBreaker-LLVM"
2020-03-19T21:11:04.6796900Z    ┌── ice-breaker/llvm.md:3:19 ───
2020-03-19T21:11:04.6797199Z    │
2020-03-19T21:11:04.6797543Z  3 │ **Github Label:** [ICEBreaker-LLVM]
2020-03-19T21:11:04.6798025Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6798025Z    │                   ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6798367Z    │
2020-03-19T21:11:04.6798465Z 
2020-03-19T21:11:04.6798991Z error: The server responded with 429 Too Many Requests for "***c-guide"
2020-03-19T21:11:04.6799725Z     ┌── part-2-intro.md:10:17 ───
2020-03-19T21:11:04.6800072Z     │
2020-03-19T21:11:04.6800072Z     │
2020-03-19T21:11:04.6800580Z  10 │ an issue on the [rustc-guide repo](***c-guide)
2020-03-19T21:11:04.6804958Z     │
2020-03-19T21:11:04.6805074Z 
2020-03-19T21:11:04.6805074Z 
2020-03-19T21:11:04.6805673Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustdoc"
2020-03-19T21:11:04.6807124Z    ┌── rustdoc.md:6:50 ───
2020-03-19T21:11:04.6807423Z    │
2020-03-19T21:11:04.6807423Z    │
2020-03-19T21:11:04.6807861Z  6 │ Rustdoc is implemented entirely within the crate [`librustdoc`][rd]. It runs
2020-03-19T21:11:04.6808871Z    │
2020-03-19T21:11:04.6808969Z 
2020-03-19T21:11:04.6808969Z 
2020-03-19T21:11:04.6809700Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/tools/rustdoc"
2020-03-19T21:11:04.6810336Z     ┌── rustdoc.md:26:22 ───
2020-03-19T21:11:04.6810612Z     │
2020-03-19T21:11:04.6810998Z  26 │ using the project in [`src/tools/rustdoc`][bin]. Note that literally all that
2020-03-19T21:11:04.6811472Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6811472Z     │                      ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6811871Z     │
2020-03-19T21:11:04.6811958Z 
2020-03-19T21:11:04.6812426Z error: The server responded with 429 Too Many Requests for "***/issues/44136"
2020-03-19T21:11:04.6812911Z      ┌── rustdoc.md:115:1 ───
2020-03-19T21:11:04.6813377Z      │
2020-03-19T21:11:04.6813377Z      │
2020-03-19T21:11:04.6813904Z  115 │ [we're trying to deprecate that][44136]. If you need finer-grain control over
2020-03-19T21:11:04.6814629Z      │
2020-03-19T21:11:04.6814717Z 
2020-03-19T21:11:04.6814717Z 
2020-03-19T21:11:04.6815245Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_metadata"
2020-03-19T21:11:04.6815737Z      ┌── query.md:150:1 ───
2020-03-19T21:11:04.6816007Z      │
2020-03-19T21:11:04.6816390Z  150 │ [`rustc_metadata` crate][rustc_metadata], which loads the information from the
2020-03-19T21:11:04.6816814Z      │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6816814Z      │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6817119Z      │
2020-03-19T21:11:04.6817207Z 
2020-03-19T21:11:04.6817711Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/dep_graph"
2020-03-19T21:11:04.6818272Z     ┌── queries/incremental-compilation.md:84:1 ───
2020-03-19T21:11:04.6818562Z     │
2020-03-19T21:11:04.6818562Z     │
2020-03-19T21:11:04.6818929Z  84 │ [`src/librustc/dep_graph`][dep_graph]. Construction of the DAG is done
2020-03-19T21:11:04.6819633Z     │
2020-03-19T21:11:04.6819718Z 
2020-03-19T21:11:04.6819718Z 
2020-03-19T21:11:04.6820201Z error: The server responded with 429 Too Many Requests for "***/issues/42678"
2020-03-19T21:11:04.6820680Z    ┌── queries/profiling.md:8:9 ───
2020-03-19T21:11:04.6820952Z    │
2020-03-19T21:11:04.6820952Z    │
2020-03-19T21:11:04.6821344Z  8 │ address [issue 42678](***/issues/42678).
2020-03-19T21:11:04.6822242Z    │
2020-03-19T21:11:04.6822328Z 
2020-03-19T21:11:04.6822936Z error: The server responded with 429 Too Many Requests for "https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md"
2020-03-19T21:11:04.6823305Z 
2020-03-19T21:11:04.6823305Z 
2020-03-19T21:11:04.6823607Z      ┌── queries/profiling.md:335:3 ───
2020-03-19T21:11:04.6823880Z      │
2020-03-19T21:11:04.6824490Z  335 │   [On-demand Rustc incremental design doc](https://github.com/nikomatsakis/rustc-on-demand-incremental-design-doc/blob/master/0000-rustc-on-demand-and-incremental.md)
2020-03-19T21:11:04.6825916Z      │
2020-03-19T21:11:04.6826004Z 
2020-03-19T21:11:04.6826004Z 
2020-03-19T21:11:04.6826465Z error: The server responded with 429 Too Many Requests for "***/issues/42293"
2020-03-19T21:11:04.6826975Z      ┌── queries/profiling.md:337:3 ───
2020-03-19T21:11:04.6827246Z      │
2020-03-19T21:11:04.6827246Z      │
2020-03-19T21:11:04.6827707Z  337 │   ["Red/Green" dependency tracking in compiler](***/issues/42293)
2020-03-19T21:11:04.6828714Z      │
2020-03-19T21:11:04.6828815Z 
2020-03-19T21:11:04.6828815Z 
2020-03-19T21:11:04.6829273Z error: The server responded with 429 Too Many Requests for "***/issues/42633"
2020-03-19T21:11:04.6829854Z      ┌── queries/profiling.md:341:3 ───
2020-03-19T21:11:04.6830138Z      │
2020-03-19T21:11:04.6830138Z      │
2020-03-19T21:11:04.6830537Z  341 │ - [GitHub issue #42633](***/issues/42633)
2020-03-19T21:11:04.6831504Z      │
2020-03-19T21:11:04.6831591Z 
2020-03-19T21:11:04.6832027Z error: The server responded with 429 Too Many Requests for "https://github.com/salsa-rs/salsa"
2020-03-19T21:11:04.6832261Z 
2020-03-19T21:11:04.6832261Z 
2020-03-19T21:11:04.6832528Z    ┌── salsa.md:5:1 ───
2020-03-19T21:11:04.6832774Z    │
2020-03-19T21:11:04.6833102Z  5 │ [Salsa](https://github.com/salsa-rs/salsa).
2020-03-19T21:11:04.6833896Z    │
2020-03-19T21:11:04.6833981Z 
2020-03-19T21:11:04.6834474Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/index.html"
2020-03-19T21:11:04.6834898Z 
---
2020-03-19T21:11:04.6838454Z  49 │ - [`StringReader`] from [libsyntax] integrates `rustc_lexer` with `rustc`
2020-03-19T21:11:04.6838916Z     │                         ^ Server responded with 404 Not Found
2020-03-19T21:11:04.6839218Z     │
2020-03-19T21:11:04.6839303Z 
2020-03-19T21:11:04.6839816Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-19T21:11:04.6840330Z     ┌── test-implementation.md:34:1 ───
2020-03-19T21:11:04.6840606Z     │
2020-03-19T21:11:04.6840982Z  34 │ [`libsyntax` crate][libsyntax]. Essentially, it's a fancy macro, that
2020-03-19T21:11:04.6841404Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6847065Z  69 │ not stored as a string, but rather as an opaque [Symbol][Symbol] which is
2020-03-19T21:11:04.6847907Z     │                                                 ^ Server responded with 404 Not Found
2020-03-19T21:11:04.6848321Z     │
2020-03-19T21:11:04.6848427Z 
2020-03-19T21:11:04.6849345Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_expand/mbe"
2020-03-19T21:11:04.6850000Z     ┌── macro-expansion.md:13:1 ───
2020-03-19T21:11:04.6850348Z     │
2020-03-19T21:11:04.6850822Z  13 │ [`src/librustc_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2020-03-19T21:11:04.6851347Z     │ ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6867463Z  135 │ [`NodeId`]. This returns a `Option<Node<'tcx>>`, where [`Node`] is an enum
2020-03-19T21:11:04.6867966Z      │ ^ Server responded with 404 Not Found
2020-03-19T21:11:04.6868303Z      │
2020-03-19T21:11:04.6868412Z 
2020-03-19T21:11:04.6869605Z error: The server responded with 429 Too Many Requests for "***/blob/3ee936378662bd2e74be951d6a7011a95a6bd84d/src/librustc/ty/mod.rs"
2020-03-19T21:11:04.6870289Z      ┌── ty.md:199:50 ───
2020-03-19T21:11:04.6870623Z      │
2020-03-19T21:11:04.6870623Z      │
2020-03-19T21:11:04.6871177Z  199 │ comparison of types for equality: we implemented [`PartialEq for TyS`][peqimpl], so we can just
2020-03-19T21:11:04.6872324Z      │
2020-03-19T21:11:04.6872433Z 
2020-03-19T21:11:04.6872433Z 
2020-03-19T21:11:04.6873268Z error: The server responded with 429 Too Many Requests for "***/blob/597f432489f12a3f33419daa039ccef11a12c4fd/src/librustc_typeck/astconv.rs"
2020-03-19T21:11:04.6873948Z      ┌── ty.md:468:1 ───
2020-03-19T21:11:04.6874260Z      │
2020-03-19T21:11:04.6874260Z      │
2020-03-19T21:11:04.6874969Z  468 │ [Here is an example of actually using `subst` in the compiler][substex].  The exact details are not
2020-03-19T21:11:04.6875898Z      │
2020-03-19T21:11:04.6876022Z 
2020-03-19T21:11:04.6876022Z 
2020-03-19T21:11:04.6876700Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc_macros/src/type_foldable.rs"
2020-03-19T21:11:04.6877483Z      ┌── ty.md:573:1 ───
2020-03-19T21:11:04.6877795Z      │
2020-03-19T21:11:04.6877795Z      │
2020-03-19T21:11:04.6878345Z  573 │ [here](***/blob/master/src/librustc_macros/src/type_foldable.rs).
2020-03-19T21:11:04.6879681Z      │
2020-03-19T21:11:04.6879790Z 
2020-03-19T21:11:04.6879790Z 
2020-03-19T21:11:04.6880551Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-19T21:11:04.6881221Z      ┌── ty.md:575:46 ───
2020-03-19T21:11:04.6881551Z      │
2020-03-19T21:11:04.6881551Z      │
2020-03-19T21:11:04.6881969Z  575 │   **`subst`** In the case of substitutions the [actual
2020-03-19T21:11:04.6882499Z      │ ╭──────────────────────────────────────────────^
2020-03-19T21:11:04.6883258Z  576 │ │ folder](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L467-L482)
2020-03-19T21:11:04.6884797Z      │
2020-03-19T21:11:04.6884907Z 
2020-03-19T21:11:04.6884907Z 
2020-03-19T21:11:04.6885659Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-19T21:11:04.6886372Z      ┌── ty.md:579:1 ───
2020-03-19T21:11:04.6886684Z      │
2020-03-19T21:11:04.6886684Z      │
2020-03-19T21:11:04.6887420Z  579 │ [fold_ty](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L545-L573)
2020-03-19T21:11:04.6888935Z      │
2020-03-19T21:11:04.6889057Z 
2020-03-19T21:11:04.6889057Z 
2020-03-19T21:11:04.6889806Z error: The server responded with 429 Too Many Requests for "***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs"
2020-03-19T21:11:04.6890481Z      ┌── ty.md:583:1 ───
2020-03-19T21:11:04.6890794Z      │
2020-03-19T21:11:04.6890794Z      │
2020-03-19T21:11:04.6892268Z  583 │ [ty_for_param](***/blob/04e69e4f4234beb4f12cc76dcc53e2cc4247a9be/src/librustc/ty/subst.rs#L589-L624)
2020-03-19T21:11:04.6893768Z      │
2020-03-19T21:11:04.6893875Z 
2020-03-19T21:11:04.6893875Z 
2020-03-19T21:11:04.6894566Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc/infer/higher_ranked/README.md"
2020-03-19T21:11:04.6895236Z     ┌── traits/hrtb.md:35:62 ───
2020-03-19T21:11:04.6895557Z     │
2020-03-19T21:11:04.6896112Z  35 │ to the subtyping for higher-ranked types (which is described [here][hrsubtype]
2020-03-19T21:11:04.6896807Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6896807Z     │                                                              ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6897268Z     │
2020-03-19T21:11:04.6897375Z 
2020-03-19T21:11:04.6897962Z error: The server responded with 429 Too Many Requests for "***/issues/22019"
2020-03-19T21:11:04.6898624Z     ┌── traits/caching.md:57:30 ───
2020-03-19T21:11:04.6898951Z     │
2020-03-19T21:11:04.6898951Z     │
2020-03-19T21:11:04.6899435Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-19T21:11:04.6900414Z     │
2020-03-19T21:11:04.6900521Z 
2020-03-19T21:11:04.6900521Z 
2020-03-19T21:11:04.6901105Z error: The server responded with 429 Too Many Requests for "***/issues/18290"
2020-03-19T21:11:04.6901694Z     ┌── traits/caching.md:57:43 ───
2020-03-19T21:11:04.6902034Z     │
2020-03-19T21:11:04.6902034Z     │
2020-03-19T21:11:04.6902509Z  57 │ annoying and weird bugs like [#22019] and [#18290]. This simple rule seems
2020-03-19T21:11:04.6903565Z     │
2020-03-19T21:11:04.6903672Z 
2020-03-19T21:11:04.6903672Z 
2020-03-19T21:11:04.6904238Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-03-19T21:11:04.6904842Z    ┌── traits/index.md:4:3 ───
2020-03-19T21:11:04.6905156Z    │
2020-03-19T21:11:04.6905156Z    │
2020-03-19T21:11:04.6905619Z  4 │ > [process of being implemented][wg]; this chapter serves as a kind of
2020-03-19T21:11:04.6906474Z    │
2020-03-19T21:11:04.6906581Z 
2020-03-19T21:11:04.6906581Z 
2020-03-19T21:11:04.6907168Z error: The server responded with 429 Too Many Requests for "***/issues/48416"
2020-03-19T21:11:04.6907745Z     ┌── traits/index.md:11:3 ───
2020-03-19T21:11:04.6908078Z     │
2020-03-19T21:11:04.6908078Z     │
2020-03-19T21:11:04.6908475Z  11 │ > [Traits Working Group tracking issue][wg]!
2020-03-19T21:11:04.6909307Z     │
2020-03-19T21:11:04.6909412Z 
2020-03-19T21:11:04.6910001Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/tree/master/chalk-engine"
2020-03-19T21:11:04.6910345Z 
2020-03-19T21:11:04.6910345Z 
2020-03-19T21:11:04.6910707Z     ┌── traits/index.md:54:7 ───
2020-03-19T21:11:04.6911025Z     │
2020-03-19T21:11:04.6911424Z  54 │ * the [`chalk_engine`][chalk_engine] crate, which
2020-03-19T21:11:04.6911926Z     │       ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6912277Z     │
2020-03-19T21:11:04.6912384Z 
2020-03-19T21:11:04.6913014Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-03-19T21:11:04.6913625Z     ┌── traits/index.md:60:1 ───
2020-03-19T21:11:04.6913946Z     │
2020-03-19T21:11:04.6914328Z  60 │ [`librustc_traits`][librustc_traits].
2020-03-19T21:11:04.6914987Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6914987Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6915348Z     │
2020-03-19T21:11:04.6915470Z 
2020-03-19T21:11:04.6916109Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc/traits/mod.rs"
2020-03-19T21:11:04.6916795Z     ┌── traits/goals-and-clauses.md:41:1 ───
2020-03-19T21:11:04.6917142Z     │
2020-03-19T21:11:04.6917142Z     │
2020-03-19T21:11:04.6917558Z  41 │ [`librustc/traits/mod.rs`][traits_mod] in rustc, and in
2020-03-19T21:11:04.6918399Z     │
2020-03-19T21:11:04.6918506Z 
2020-03-19T21:11:04.6921248Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs"
2020-03-19T21:11:04.6921617Z 
2020-03-19T21:11:04.6921617Z 
2020-03-19T21:11:04.6922865Z     ┌── traits/goals-and-clauses.md:42:1 ───
2020-03-19T21:11:04.6923252Z     │
2020-03-19T21:11:04.6924418Z  42 │ [`chalk-ir/src/lib.rs`][chalk_ir] in chalk.
2020-03-19T21:11:04.6925371Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6925727Z     │
2020-03-19T21:11:04.6925835Z 
2020-03-19T21:11:04.6926506Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustc/ty/sty.rs"
2020-03-19T21:11:04.6927274Z     ┌── traits/associated-types.md:97:22 ───
2020-03-19T21:11:04.6927640Z     │
2020-03-19T21:11:04.6927640Z     │
2020-03-19T21:11:04.6928097Z  97 │ variant, declared in [`librustc/ty/sty.rs`][sty]. In chalk, we use an
2020-03-19T21:11:04.6929060Z     │
2020-03-19T21:11:04.6929167Z 
2020-03-19T21:11:04.6929797Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang-nursery/chalk/blob/master/chalk-ir/src/lib.rs"
2020-03-19T21:11:04.6930154Z 
---
2020-03-19T21:11:04.6932885Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/blob/master/text/2089-implied-bounds.md"
2020-03-19T21:11:04.6933255Z 
2020-03-19T21:11:04.6933636Z     ┌── traits/implied-bounds.md:89:34 ───
2020-03-19T21:11:04.6933976Z     │
2020-03-19T21:11:04.6934456Z  89 │ types. The full RFC can be found [here][RFC]. We'll give here a brief view
2020-03-19T21:11:04.6935459Z     │
2020-03-19T21:11:04.6935579Z 
2020-03-19T21:11:04.6935579Z 
2020-03-19T21:11:04.6936148Z error: The server responded with 429 Too Many Requests for "***/pull/43786"
2020-03-19T21:11:04.6936774Z      ┌── traits/implied-bounds.md:313:8 ───
2020-03-19T21:11:04.6937130Z      │
2020-03-19T21:11:04.6937130Z      │
2020-03-19T21:11:04.6937585Z  313 │ can be [exploited][bug]. Indeed, suppose that we define the following
2020-03-19T21:11:04.6938492Z      │
2020-03-19T21:11:04.6938600Z 
2020-03-19T21:11:04.6938600Z 
2020-03-19T21:11:04.6939236Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify"
2020-03-19T21:11:04.6939897Z     ┌── traits/lowering-module.md:25:27 ───
2020-03-19T21:11:04.6940238Z     │
2020-03-19T21:11:04.6940699Z  25 │ Unit tests are located in [`src/test/ui/chalkify`][chalkify]. A good
2020-03-19T21:11:04.6941271Z     │                           ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6941271Z     │                           ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6941769Z     │
2020-03-19T21:11:04.6941881Z 
2020-03-19T21:11:04.6942490Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify/lower_impl.rs"
2020-03-19T21:11:04.6943135Z     ┌── traits/lowering-module.md:26:17 ───
2020-03-19T21:11:04.6943454Z     │
2020-03-19T21:11:04.6944238Z  26 │ example test is [the `lower_impl` test][lower_impl]. At the time of
2020-03-19T21:11:04.6944766Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6944766Z     │                 ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6945113Z     │
2020-03-19T21:11:04.6945221Z 
2020-03-19T21:11:04.6945860Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/ui/chalkify/lower_impl.stderr"
2020-03-19T21:11:04.6946493Z     ┌── traits/lowering-module.md:49:38 ───
2020-03-19T21:11:04.6946811Z     │
2020-03-19T21:11:04.6947227Z  49 │ `#[rustc_dump_program_clauses]`, but [the stderr file] contains
2020-03-19T21:11:04.6947774Z     │                                      ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6950376Z  31 │ [`chalk/chalk-solve/src/clauses.rs`][chalk_rules]. They are also ported in
2020-03-19T21:11:04.6950924Z     │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6951259Z     │
2020-03-19T21:11:04.6951360Z 
2020-03-19T21:11:04.6951932Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_traits"
2020-03-19T21:11:04.6952912Z     ┌── traits/lowering-rules.md:32:14 ───
2020-03-19T21:11:04.6953230Z     │
2020-03-19T21:11:04.6953630Z  32 │ rustc in the [`librustc_traits`][librustc_traits] crate.
2020-03-19T21:11:04.6954358Z     │              ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6958897Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/master/tests/test/wf_lowering.rs"
2020-03-19T21:11:04.6959413Z 
2020-03-19T21:11:04.6959769Z     ┌── traits/wf.md:16:36 ───
2020-03-19T21:11:04.6960084Z     │
2020-03-19T21:11:04.6960592Z  16 │ an extended set of examples in the [`chalk/tests/test/wf_lowering.rs`][wf_test] submodule.
2020-03-19T21:11:04.6961655Z     │
2020-03-19T21:11:04.6961763Z 
2020-03-19T21:11:04.6962493Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/chalk/blob/239e4ae4e69b2785b5f99e0f2b41fc16b0b4e65e/chalk-engine/src/README.md"
2020-03-19T21:11:04.6962924Z 
2020-03-19T21:11:04.6962924Z 
2020-03-19T21:11:04.6963286Z      ┌── traits/slg.md:293:3 ───
2020-03-19T21:11:04.6963608Z      │
2020-03-19T21:11:04.6964082Z  293 │ - [chalk_solve README][readme], which contains links to papers used and
2020-03-19T21:11:04.6964956Z      │
2020-03-19T21:11:04.6965065Z 
2020-03-19T21:11:04.6965065Z 
2020-03-19T21:11:04.6965658Z error: The server responded with 429 Too Many Requests for "***c-guide/issues"
2020-03-19T21:11:04.6966264Z    ┌── traits/chalk-overview.md:5:3 ───
2020-03-19T21:11:04.6966612Z    │
2020-03-19T21:11:04.6966612Z    │
2020-03-19T21:11:04.6967075Z  5 │ > [open an issue][rustc-issues] so we can fix it. If you are able to fix the
2020-03-19T21:11:04.6967966Z    │
2020-03-19T21:11:04.6968071Z 
2020-03-19T21:11:04.6968071Z 
2020-03-19T21:11:04.6968635Z error: The server responded with 429 Too Many Requests for "***/pull/62474"
2020-03-19T21:11:04.6969267Z      ┌── codegen/updating-llvm.md:130:1 ───
2020-03-19T21:11:04.6969623Z      │
2020-03-19T21:11:04.6969623Z      │
2020-03-19T21:11:04.6970083Z  130 │ [#62474](***/pull/62474)
2020-03-19T21:11:04.6971097Z      │
2020-03-19T21:11:04.6971204Z 
2020-03-19T21:11:04.6971204Z 
2020-03-19T21:11:04.6971780Z error: The server responded with 429 Too Many Requests for "***/pull/62592"
2020-03-19T21:11:04.6972388Z      ┌── codegen/updating-llvm.md:131:1 ───
2020-03-19T21:11:04.6972746Z      │
2020-03-19T21:11:04.6972746Z      │
2020-03-19T21:11:04.6973255Z  131 │ [#62592](***/pull/62592). Note that sometimes it's
2020-03-19T21:11:04.6974438Z      │
2020-03-19T21:11:04.6974545Z 
2020-03-19T21:11:04.6975089Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/llvm-project/"
2020-03-19T21:11:04.6975407Z 
---
2020-03-19T21:11:04.6978098Z error: The server responded with 429 Too Many Requests for "https://github.com/CraneStation/cranelift"
2020-03-19T21:11:04.6978399Z 
2020-03-19T21:11:04.6978886Z    ┌── codegen/backend-agnostic.md:4:1 ───
2020-03-19T21:11:04.6979212Z    │
2020-03-19T21:11:04.6979644Z  4 │ [Cranelift][cranelift]). To this end, `librustc_codegen_ssa` provides an
2020-03-19T21:11:04.6980446Z    │
2020-03-19T21:11:04.6980543Z 
2020-03-19T21:11:04.6980543Z 
2020-03-19T21:11:04.6981142Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-19T21:11:04.6981803Z     ┌── profile-guided-optimization.md:65:33 ───
2020-03-19T21:11:04.6982136Z     │
2020-03-19T21:11:04.6982136Z     │
2020-03-19T21:11:04.6982579Z  65 │ `rustc` instructs LLVM to do so [by setting the appropriate][pgo-gen-passmanager]
2020-03-19T21:11:04.6983526Z     │
2020-03-19T21:11:04.6983625Z 
2020-03-19T21:11:04.6983625Z 
2020-03-19T21:11:04.6984278Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/librustc_codegen_ssa/back/symbol_export.rs"
2020-03-19T21:11:04.6985011Z     ┌── profile-guided-optimization.md:77:25 ───
2020-03-19T21:11:04.6985312Z     │
2020-03-19T21:11:04.6985312Z     │
2020-03-19T21:11:04.6985714Z  77 │ runtime are not removed [by marking the with the right export level][pgo-gen-symbols].
2020-03-19T21:11:04.6986526Z     │
2020-03-19T21:11:04.6986622Z 
2020-03-19T21:11:04.6986622Z 
2020-03-19T21:11:04.6987141Z error: The server responded with 429 Too Many Requests for "***/blob/1.34.1/src/rustllvm/PassWrapper.cpp"
2020-03-19T21:11:04.6987706Z     ┌── profile-guided-optimization.md:88:11 ───
2020-03-19T21:11:04.6988025Z     │
2020-03-19T21:11:04.6988025Z     │
2020-03-19T21:11:04.6988400Z  88 │ basically [just telling][pgo-use-passmanager] the LLVM `PassManagerBuilder`
2020-03-19T21:11:04.6989143Z     │
2020-03-19T21:11:04.6989229Z 
2020-03-19T21:11:04.6989752Z error: The server responded with 429 Too Many Requests for "https://github.com/llvm/llvm-project/tree/master/compiler-rt/lib/profile"
2020-03-19T21:11:04.6990038Z 
2020-03-19T21:11:04.6990038Z 
2020-03-19T21:11:04.6990369Z      ┌── profile-guided-optimization.md:109:1 ───
2020-03-19T21:11:04.6990662Z      │
2020-03-19T21:11:04.6991060Z  109 │ [compiler-rt][compiler-rt-profile] and statically linked into any instrumented
2020-03-19T21:11:04.6991485Z      │ ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.6991776Z      │
2020-03-19T21:11:04.6991875Z 
2020-03-19T21:11:04.6992387Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/test/run-make-fulldeps"
2020-03-19T21:11:04.6992948Z      ┌── profile-guided-optimization.md:122:4 ───
2020-03-19T21:11:04.6993242Z      │
2020-03-19T21:11:04.6993242Z      │
2020-03-19T21:11:04.6993628Z  122 │ in [run-make tests][rmake-tests] (the relevant tests have `pgo` in their name).
2020-03-19T21:11:04.6994361Z      │
2020-03-19T21:11:04.6994449Z 
2020-03-19T21:11:04.6994449Z 
2020-03-19T21:11:04.6995242Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/test/codegen/pgo-instrumentation.rs"
2020-03-19T21:11:04.6995842Z      ┌── profile-guided-optimization.md:123:17 ───
2020-03-19T21:11:04.6996153Z      │
2020-03-19T21:11:04.6996535Z  123 │ There is also a [codegen test][codegen-test] that checks that some expected
2020-03-19T21:11:04.6997054Z      │                 ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.6999152Z  24 │ 1. The sanitizer runtime libraries are part of the [compiler-rt] project, and
2020-03-19T21:11:04.6999700Z     │                                                    ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7000049Z     │
2020-03-19T21:11:04.7000134Z 
2020-03-19T21:11:04.7001218Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/native.rs"
2020-03-19T21:11:04.7002420Z     ┌── sanitizers.md:25:4 ───
2020-03-19T21:11:04.7002757Z     │
2020-03-19T21:11:04.7002757Z     │
2020-03-19T21:11:04.7003235Z  25 │    [will be built as an LLVM subproject][sanitizer-build] when enabled in
2020-03-19T21:11:04.7004106Z     │
2020-03-19T21:11:04.7004225Z 
2020-03-19T21:11:04.7004225Z 
2020-03-19T21:11:04.7004967Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/bootstrap/compile.rs"
2020-03-19T21:11:04.7005669Z     ┌── sanitizers.md:33:21 ───
2020-03-19T21:11:04.7005993Z     │
2020-03-19T21:11:04.7006433Z  33 │    The runtimes are [placed into target libdir][sanitizer-copy].
2020-03-19T21:11:04.7007008Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7007008Z     │                     ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7007389Z     │
2020-03-19T21:11:04.7007496Z 
2020-03-19T21:11:04.7008174Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_llvm/declare.rs"
2020-03-19T21:11:04.7008828Z     ┌── sanitizers.md:36:4 ───
2020-03-19T21:11:04.7009166Z     │
2020-03-19T21:11:04.7009166Z     │
2020-03-19T21:11:04.7009638Z  36 │    [marked][sanitizer-attribute] with `SanitizeAddress`, `SanitizeMemory`, or
2020-03-19T21:11:04.7010515Z     │
2020-03-19T21:11:04.7010635Z 
2020-03-19T21:11:04.7010635Z 
2020-03-19T21:11:04.7011301Z error: The server responded with 429 Too Many Requests for "***/blob/1.38.0/src/librustc_codegen_ssa/back/write.rs"
2020-03-19T21:11:04.7011963Z     ┌── sanitizers.md:41:54 ───
2020-03-19T21:11:04.7012287Z     │
2020-03-19T21:11:04.7012746Z  41 │ 3. The LLVM IR generated by rustc is instrumented by [dedicated LLVM
2020-03-19T21:11:04.7013404Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7013404Z     │                                                      ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7013842Z     │
2020-03-19T21:11:04.7013950Z 
2020-03-19T21:11:04.7014731Z error: The server responded with 429 Too Many Requests for "***/blob/87c3eedffba64830b67e54e75dd479f9fd83cc7d/src/librustc_codegen_ssa/back/link.rs"
2020-03-19T21:11:04.7015444Z     ┌── sanitizers.md:46:4 ───
2020-03-19T21:11:04.7015779Z     │
2020-03-19T21:11:04.7016256Z  46 │    [linked in][sanitizer-link]. The libraries are searched for in target libdir
2020-03-19T21:11:04.7016789Z     │    ^ Server responded with 429 Too Many Requests
---
2020-03-19T21:11:04.7022591Z  45 │ We have our own fork of GDB - [https://github.com/rust-dev-tools/gdb]
2020-03-19T21:11:04.7023197Z     │                               ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7023593Z     │
2020-03-19T21:11:04.7023699Z 
2020-03-19T21:11:04.7024295Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-19T21:11:04.7024943Z     ┌── debugging-support-in-rustc.md:80:1 ───
2020-03-19T21:11:04.7025315Z     │
2020-03-19T21:11:04.7025315Z     │
2020-03-19T21:11:04.7026139Z  80 │ [This comment by Tom](***c-guide/pull/316#discussion_r284027340)
2020-03-19T21:11:04.7027405Z     │
2020-03-19T21:11:04.7027512Z 
2020-03-19T21:11:04.7027512Z 
2020-03-19T21:11:04.7028100Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-19T21:11:04.7028736Z     ┌── debugging-support-in-rustc.md:86:1 ───
2020-03-19T21:11:04.7029094Z     │
2020-03-19T21:11:04.7029094Z     │
2020-03-19T21:11:04.7029667Z  86 │ [This question by Aman](***c-guide/pull/316#discussion_r285401353)
2020-03-19T21:11:04.7030994Z     │
2020-03-19T21:11:04.7031206Z 
2020-03-19T21:11:04.7031629Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb"
2020-03-19T21:11:04.7031862Z 
---
2020-03-19T21:11:04.7034225Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/lldb/wiki"
2020-03-19T21:11:04.7034466Z 
2020-03-19T21:11:04.7034898Z      ┌── debugging-support-in-rustc.md:121:43 ───
2020-03-19T21:11:04.7035200Z      │
2020-03-19T21:11:04.7035630Z  121 │ * None of the LLDB work is upstream. This [rust-lang/lldb wiki page] explains a few details.
2020-03-19T21:11:04.7036578Z      │
2020-03-19T21:11:04.7036680Z 
2020-03-19T21:11:04.7036680Z 
2020-03-19T21:11:04.7037145Z error: The server responded with 429 Too Many Requests for "***/issues/34457"
2020-03-19T21:11:04.7037741Z      ┌── debugging-support-in-rustc.md:174:17 ───
2020-03-19T21:11:04.7038038Z      │
2020-03-19T21:11:04.7038038Z      │
2020-03-19T21:11:04.7038422Z  174 │ Tracking issue: [***/issues/34457]
2020-03-19T21:11:04.7039152Z      │
2020-03-19T21:11:04.7039239Z 
2020-03-19T21:11:04.7039239Z 
2020-03-19T21:11:04.7039711Z error: The server responded with 429 Too Many Requests for "***/issues/33014"
2020-03-19T21:11:04.7040232Z      ┌── debugging-support-in-rustc.md:229:18 ───
2020-03-19T21:11:04.7040525Z      │
2020-03-19T21:11:04.7040525Z      │
2020-03-19T21:11:04.7040923Z  229 │ Issue on Github: [***/issues/33014]
2020-03-19T21:11:04.7041666Z      │
2020-03-19T21:11:04.7041753Z 
2020-03-19T21:11:04.7042197Z error: The server responded with 429 Too Many Requests for "https://github.com/rust-lang/rfcs/pull/2117"
2020-03-19T21:11:04.7042443Z 
2020-03-19T21:11:04.7042443Z 
2020-03-19T21:11:04.7042780Z      ┌── debugging-support-in-rustc.md:265:6 ───
2020-03-19T21:11:04.7043083Z      │
2020-03-19T21:11:04.7043424Z  265 │ RFC: [https://github.com/rust-lang/rfcs/pull/2117]
2020-03-19T21:11:04.7043844Z      │      ^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7044133Z      │
2020-03-19T21:11:04.7044220Z 
2020-03-19T21:11:04.7044869Z error: The server responded with 429 Too Many Requests for "***c-guide/pull/316"
2020-03-19T21:11:04.7045465Z      ┌── debugging-support-in-rustc.md:279:1 ───
2020-03-19T21:11:04.7045801Z      │
2020-03-19T21:11:04.7045801Z      │
2020-03-19T21:11:04.7046327Z  279 │ [Question on Github](***c-guide/pull/316#discussion_r283062536).
2020-03-19T21:11:04.7047499Z      │
2020-03-19T21:11:04.7047599Z 
2020-03-19T21:11:04.7048090Z error: The server responded with 429 Too Many Requests for "https://github.com/nrc/stupid-stats"
2020-03-19T21:11:04.7048372Z 
2020-03-19T21:11:04.7048372Z 
2020-03-19T21:11:04.7048732Z    ┌── appendix/stupid-stats.md:3:48 ───
2020-03-19T21:11:04.7049043Z    │
2020-03-19T21:11:04.7049486Z  3 │ > **Note:** This is a copy of `@nrc`'s amazing [stupid-stats]. You should find
2020-03-19T21:11:04.7050483Z    │
2020-03-19T21:11:04.7050581Z 
2020-03-19T21:11:04.7050581Z 
2020-03-19T21:11:04.7051150Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/libsyntax"
2020-03-19T21:11:04.7051740Z     ┌── appendix/stupid-stats.md:64:43 ───
2020-03-19T21:11:04.7052064Z     │
2020-03-19T21:11:04.7052064Z     │
2020-03-19T21:11:04.7052648Z  64 │ The code for these first two phases is in [libsyntax](***/tree/master/src/libsyntax).
2020-03-19T21:11:04.7053932Z     │
2020-03-19T21:11:04.7054031Z 
2020-03-19T21:11:04.7054031Z 
2020-03-19T21:11:04.7054586Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc"
2020-03-19T21:11:04.7055185Z     ┌── appendix/stupid-stats.md:76:25 ───
2020-03-19T21:11:04.7055500Z     │
2020-03-19T21:11:04.7055500Z     │
2020-03-19T21:11:04.7056025Z  76 │ The analysis code is in [librustc](***/tree/master/src/librustc)
2020-03-19T21:11:04.7057173Z     │
2020-03-19T21:11:04.7057271Z 
2020-03-19T21:11:04.7057271Z 
2020-03-19T21:11:04.7057906Z error: The server responded with 429 Too Many Requests for "***/tree/master/src/librustc_llvm"
2020-03-19T21:11:04.7058515Z     ┌── appendix/stupid-stats.md:87:40 ───
2020-03-19T21:11:04.7058850Z     │
2020-03-19T21:11:04.7058850Z     │
2020-03-19T21:11:04.7059427Z  87 │ interface between LLVM and rustc is in [librustc_llvm](***/tree/master/src/librustc_llvm).
2020-03-19T21:11:04.7060787Z     │
2020-03-19T21:11:04.7060872Z 
2020-03-19T21:11:04.7061361Z error: The server responded with 429 Too Many Requests for "https://github.com/nick29581/stupid-stats/blob/master/src"
2020-03-19T21:11:04.7061797Z 
---
2020-03-19T21:11:04.7066395Z  205 │ [multirust](https://github.com/brson/multirust) to get around all the PATH stuff
2020-03-19T21:11:04.7067022Z      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 429 Too Many Requests
2020-03-19T21:11:04.7067417Z      │
2020-03-19T21:11:04.7067519Z 
2020-03-19T21:11:04.7068047Z error: The server responded with 429 Too Many Requests for "***/issues"
2020-03-19T21:11:04.7068622Z      ┌── appendix/stupid-stats.md:402:33 ───
2020-03-19T21:11:04.7068957Z      │
2020-03-19T21:11:04.7068957Z      │
2020-03-19T21:11:04.7069558Z  402 │ do, let me know in a comment or [GitHub issue](***/issues).
2020-03-19T21:11:04.7070561Z      │
2020-03-19T21:11:04.7070649Z 
2020-03-19T21:11:04.7071480Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html"
2020-03-19T21:11:04.7071837Z 
2020-03-19T21:11:04.7071837Z 
2020-03-19T21:11:04.7072212Z     ┌── appendix/code-index.md:11:90 ───
2020-03-19T21:11:04.7072548Z     │
2020-03-19T21:11:04.7073343Z  11 │ `ast::Crate` | struct | A syntax-level representation of a parsed crate | [The parser] | [src/libsyntax/ast.rs](https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ast/struct.Crate.html)
2020-03-19T21:11:04.7074540Z     │                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 404 Not Found
2020-03-19T21:11:04.7075523Z     │
2020-03-19T21:11:04.7075636Z 
2020-03-19T21:11:04.7076268Z error: The server responded with 429 Too Many Requests for "***/blob/master/src/librustdoc/core.rs"
2020-03-19T21:11:04.7076942Z     ┌── appendix/code-index.md:15:131 ───
2020-03-19T21:11:04.7077278Z     │
2020-03-19T21:11:04.7077278Z     │
2020-03-19T21:11:04.7078161Z  15 │ `DocContext` | struct | A state container used by rustdoc when crawling through a crate to gather its documentation | [Rustdoc] | [src/librustdoc/core.rs](***/blob/master/src/librustdoc/core.rs)
2020-03-19T21:11:04.7080274Z     │
2020-03-19T21:11:04.7080382Z 
2020-03-19T21:11:04.7081004Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/node_id/struct.NodeId.html"
2020-03-19T21:11:04.7081371Z 
---
2020-03-19T21:28:38.0419576Z 
2020-03-19T21:28:38.0419937Z If you do intend to update 'miri', please check the error messages above and
2020-03-19T21:28:38.0420182Z commit another update.
2020-03-19T21:28:38.0420290Z 
2020-03-19T21:28:38.0420652Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2020-03-19T21:28:38.0421128Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2020-03-19T21:28:38.0421344Z proper steps.
2020-03-19T21:28:38.0430063Z Build completed unsuccessfully in 0:00:01
2020-03-19T21:28:38.0483768Z == clock drift check ==
2020-03-19T21:28:38.0504901Z   local time: Thu Mar 19 21:28:38 UTC 2020
2020-03-19T21:28:38.6051109Z   network time: Thu, 19 Mar 2020 21:28:38 GMT
2020-03-19T21:28:38.6051109Z   network time: Thu, 19 Mar 2020 21:28:38 GMT
2020-03-19T21:28:38.6052523Z == end clock drift check ==
2020-03-19T21:28:39.4272475Z 
2020-03-19T21:28:39.4333610Z ##[error]Bash exited with code '1'.
2020-03-19T21:28:39.4344957Z ##[section]Finishing: Run build
2020-03-19T21:28:39.4425407Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70077/merge to s
2020-03-19T21:28:39.4429635Z Task         : Get sources
2020-03-19T21:28:39.4429925Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T21:28:39.4430182Z Version      : 1.0.0
2020-03-19T21:28:39.4430480Z Author       : Microsoft
2020-03-19T21:28:39.4430480Z Author       : Microsoft
2020-03-19T21:28:39.4430740Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T21:28:39.4431018Z ==============================================================================
2020-03-19T21:28:39.7063441Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T21:28:39.7104033Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70077/merge to s
2020-03-19T21:28:39.7183541Z Cleaning up task key
2020-03-19T21:28:39.7184825Z Start cleaning up orphan processes.
2020-03-19T21:28:39.7341040Z Terminate orphan process: pid (4302) (python)
2020-03-19T21:28:39.7585761Z ##[section]Finishing: Finalize Job
