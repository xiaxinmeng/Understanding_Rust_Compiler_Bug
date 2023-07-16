plain
2019-10-07T14:07:59.4287802Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T14:07:59.4453052Z ##[command]git config gc.auto 0
2019-10-07T14:07:59.4532462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T14:07:59.4588796Z ##[command]git config --get-all http.proxy
2019-10-07T14:07:59.4740782Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65112/merge:refs/remotes/pull/65112/merge
---
2019-10-07T14:41:01.9541939Z    Compiling rls-data v0.19.0
2019-10-07T14:41:11.2086414Z    Compiling synstructure v0.12.1
2019-10-07T14:41:47.2521278Z    Compiling rand v0.7.0
2019-10-07T14:41:50.8461096Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-10-07T14:41:51.0412595Z error: unnecessary parentheses around type
2019-10-07T14:41:51.0413895Z   --> src/librustc_data_structures/graph/scc/mod.rs:40:24
2019-10-07T14:41:51.0414883Z    |
2019-10-07T14:41:51.0415820Z 40 |     pub fn new(graph: &(impl DirectedGraph<Node = N> + WithNumNodes + WithSuccessors)) -> Self {
2019-10-07T14:41:51.0416531Z    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-07T14:41:51.0417556Z    = note: `-D unused-parens` implied by `-D warnings`
2019-10-07T14:41:51.0421007Z 
2019-10-07T14:41:51.0462072Z error: unnecessary parentheses around type
2019-10-07T14:41:51.0462692Z     --> src/librustc_data_structures/owning_ref/mod.rs:1049:28
2019-10-07T14:41:51.0462692Z     --> src/librustc_data_structures/owning_ref/mod.rs:1049:28
2019-10-07T14:41:51.0463133Z      |
2019-10-07T14:41:51.0463621Z 1049 |     where O: Send, for<'a> (&'a T): Send {}
2019-10-07T14:41:51.0464413Z      |                            ^^^^^^^ help: remove these parentheses
2019-10-07T14:41:51.0497995Z error: unnecessary parentheses around type
2019-10-07T14:41:51.0498647Z     --> src/librustc_data_structures/owning_ref/mod.rs:1051:28
2019-10-07T14:41:51.0499065Z      |
2019-10-07T14:41:51.0499065Z      |
2019-10-07T14:41:51.0499522Z 1051 |     where O: Sync, for<'a> (&'a T): Sync {}
2019-10-07T14:41:51.0500063Z      |                            ^^^^^^^ help: remove these parentheses
2019-10-07T14:41:51.0542274Z error: unnecessary parentheses around type
2019-10-07T14:41:51.0543342Z     --> src/librustc_data_structures/owning_ref/mod.rs:1054:28
2019-10-07T14:41:51.0543796Z      |
2019-10-07T14:41:51.0543796Z      |
2019-10-07T14:41:51.0545761Z 1054 |     where O: Send, for<'a> (&'a mut T): Send {}
2019-10-07T14:41:51.0546132Z      |                            ^^^^^^^^^^^ help: remove these parentheses
2019-10-07T14:41:51.0581229Z error: unnecessary parentheses around type
2019-10-07T14:41:51.0581562Z     --> src/librustc_data_structures/owning_ref/mod.rs:1056:28
2019-10-07T14:41:51.0581784Z      |
2019-10-07T14:41:51.0581784Z      |
2019-10-07T14:41:51.0582095Z 1056 |     where O: Sync, for<'a> (&'a mut T): Sync {}
2019-10-07T14:41:51.0582504Z      |                            ^^^^^^^^^^^ help: remove these parentheses
2019-10-07T14:41:51.9470741Z error: aborting due to 5 previous errors
2019-10-07T14:41:51.9475238Z 
2019-10-07T14:41:51.9679223Z error: could not compile `rustc_data_structures`.
2019-10-07T14:41:51.9703183Z warning: build failed, waiting for other jobs to finish...
---
2019-10-07T14:41:58.8124025Z == clock drift check ==
2019-10-07T14:41:58.8155131Z   local time: Mon Oct  7 14:41:58 UTC 2019
2019-10-07T14:41:58.9709670Z   network time: Mon, 07 Oct 2019 14:41:58 GMT
2019-10-07T14:41:58.9709798Z == end clock drift check ==
2019-10-07T14:42:00.0702027Z ##[error]Bash exited with code '1'.
2019-10-07T14:42:00.0752846Z ##[section]Starting: Checkout
2019-10-07T14:42:00.0754993Z ==============================================================================
2019-10-07T14:42:00.0755047Z Task         : Get sources
2019-10-07T14:42:00.0755114Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
