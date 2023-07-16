plain
2020-02-21T23:54:10.6408621Z ========================== Starting Command Output ===========================
2020-02-21T23:54:10.6412219Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/56382f04-6c99-4c55-8ee9-f0455aca9df2.sh
2020-02-21T23:54:10.6412604Z 
2020-02-21T23:54:10.6415582Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T23:54:10.6430511Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-21T23:54:10.6434651Z Task         : Get sources
2020-02-21T23:54:10.6435005Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T23:54:10.6435772Z Version      : 1.0.0
2020-02-21T23:54:10.6436140Z Author       : Microsoft
---
2020-02-21T23:54:11.6496291Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T23:54:11.6503333Z ##[command]git config gc.auto 0
2020-02-21T23:54:11.6527416Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T23:54:11.6532380Z ##[command]git config --get-all http.proxy
2020-02-21T23:54:11.6553485Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69363/merge:refs/remotes/pull/69363/merge
---
2020-02-21T23:57:51.0490654Z Successfully built 62611c4d3db2
2020-02-21T23:57:51.0529934Z Successfully tagged rust-ci:latest
2020-02-21T23:57:51.0819226Z Built container sha256:62611c4d3db2797ec0fae96806923e5b0e9cca408f09a11e60bee542b0fd9835
2020-02-21T23:57:51.0834851Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/d42805294727747964a78ce70c906cffcf151e0ac7404a3765966e44959ef24b29c11dc98d21205141b0951a747cfe58c33808b212e0a5b1f239c2c509aa3b06
2020-02-21T23:58:28.4208019Z upload failed: - to s3://rust-lang-ci-sccache2/docker/d42805294727747964a78ce70c906cffcf151e0ac7404a3765966e44959ef24b29c11dc98d21205141b0951a747cfe58c33808b212e0a5b1f239c2c509aa3b06 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-02-21T23:58:28.8681350Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-02-21T23:58:28.8704632Z == clock drift check ==
2020-02-21T23:58:28.8715381Z   local time: Fri Feb 21 23:58:28 UTC 2020
2020-02-21T23:58:29.1611741Z   network time: Fri, 21 Feb 2020 23:58:29 GMT
---
2020-02-22T00:25:29.3830170Z     |          ^^^^^
2020-02-22T00:25:29.3831175Z     |          |
2020-02-22T00:25:29.3832063Z     |          type annotation requires that `'a` must outlive `'static`
2020-02-22T00:25:29.3832985Z     |          in this macro invocation
2020-02-22T00:25:29.3834512Z 204 | enum DifferenceInner<'a, T: 'a> {
2020-02-22T00:25:29.3837877Z     | 
2020-02-22T00:25:29.3838819Z    ::: <::core::fmt::macros::Debug macros>:1:1
2020-02-22T00:25:29.3840334Z     |
2020-02-22T00:25:29.3840973Z 1   | ($ item : item) => { }
---
2020-02-22T00:25:29.4006256Z     |
2020-02-22T00:25:29.4007133Z 369 |     pub fn difference<'a>(&'a self, other: &'a BTreeSet<T>) -> Difference<'a, T> {
2020-02-22T00:25:29.4007995Z     |                       -- lifetime `'a` defined here
2020-02-22T00:25:29.4008498Z ...
2020-02-22T00:25:29.4009169Z 396 |                     DifferenceInner::Search { self_iter: self.iter(), other_set: other }
2020-02-22T00:25:29.4010221Z     |                                                                                  ^^^^^ requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4011422Z     = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4014918Z 
2020-02-22T00:25:29.4053427Z error: lifetime may not live long enough
2020-02-22T00:25:29.4054016Z    --> src/liballoc/collections/btree/set.rs:477:86
2020-02-22T00:25:29.4054016Z    --> src/liballoc/collections/btree/set.rs:477:86
2020-02-22T00:25:29.4054450Z     |
2020-02-22T00:25:29.4055173Z 455 |     pub fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T> {
2020-02-22T00:25:29.4055983Z     |                         -- lifetime `'a` defined here
2020-02-22T00:25:29.4056467Z ...
2020-02-22T00:25:29.4057126Z 477 |                     IntersectionInner::Search { small_iter: other.iter(), large_set: self }
2020-02-22T00:25:29.4058103Z     |                                                                                      ^^^^ requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4059902Z     = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4064021Z 
2020-02-22T00:25:29.4336760Z error: lifetime may not live long enough
2020-02-22T00:25:29.4337363Z     --> src/liballoc/collections/btree/set.rs:1280:17
2020-02-22T00:25:29.4337363Z     --> src/liballoc/collections/btree/set.rs:1280:17
2020-02-22T00:25:29.4337809Z      |
2020-02-22T00:25:29.4338329Z 1277 |     fn clone(&self) -> Self {
2020-02-22T00:25:29.4339134Z      |              ----- has type `&collections::btree::set::Difference<'1, T>`
2020-02-22T00:25:29.4339759Z ...
2020-02-22T00:25:29.4340576Z 1280 |                 DifferenceInner::Stitch { self_iter, other_iter } => DifferenceInner::Stitch {
2020-02-22T00:25:29.4341626Z      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'1` must outlive `'static`
2020-02-22T00:25:29.4395657Z error: lifetime may not live long enough
2020-02-22T00:25:29.4396303Z     --> src/liballoc/collections/btree/set.rs:1298:13
2020-02-22T00:25:29.4396773Z      |
2020-02-22T00:25:29.4396773Z      |
2020-02-22T00:25:29.4397359Z 1293 | impl<'a, T: Ord> Iterator for Difference<'a, T> {
2020-02-22T00:25:29.4398804Z ...
2020-02-22T00:25:29.4398804Z ...
2020-02-22T00:25:29.4399441Z 1298 |             DifferenceInner::Stitch { self_iter, other_iter } => {
2020-02-22T00:25:29.4400402Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4401597Z      = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4405229Z 
2020-02-22T00:25:29.4434915Z error: lifetime may not live long enough
2020-02-22T00:25:29.4435510Z     --> src/liballoc/collections/btree/set.rs:1325:13
2020-02-22T00:25:29.4435510Z     --> src/liballoc/collections/btree/set.rs:1325:13
2020-02-22T00:25:29.4435975Z      |
2020-02-22T00:25:29.4436567Z 1293 | impl<'a, T: Ord> Iterator for Difference<'a, T> {
2020-02-22T00:25:29.4437752Z ...
2020-02-22T00:25:29.4437752Z ...
2020-02-22T00:25:29.4438357Z 1325 |             DifferenceInner::Stitch { self_iter, other_iter } => {
2020-02-22T00:25:29.4439281Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4440429Z      = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4443850Z 
2020-02-22T00:25:29.4492961Z error: lifetime may not live long enough
2020-02-22T00:25:29.4493750Z     --> src/liballoc/collections/btree/set.rs:1374:17
2020-02-22T00:25:29.4493750Z     --> src/liballoc/collections/btree/set.rs:1374:17
2020-02-22T00:25:29.4494320Z      |
2020-02-22T00:25:29.4494932Z 1371 |     fn clone(&self) -> Self {
2020-02-22T00:25:29.4495979Z      |              ----- has type `&collections::btree::set::Intersection<'1, T>`
2020-02-22T00:25:29.4496728Z ...
2020-02-22T00:25:29.4497444Z 1374 |                 IntersectionInner::Stitch { a, b } => {
2020-02-22T00:25:29.4498574Z      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'1` must outlive `'static`
2020-02-22T00:25:29.4530863Z error: lifetime may not live long enough
2020-02-22T00:25:29.4531528Z     --> src/liballoc/collections/btree/set.rs:1391:13
2020-02-22T00:25:29.4531991Z      |
2020-02-22T00:25:29.4531991Z      |
2020-02-22T00:25:29.4532602Z 1386 | impl<'a, T: Ord> Iterator for Intersection<'a, T> {
2020-02-22T00:25:29.4533819Z ...
2020-02-22T00:25:29.4533819Z ...
2020-02-22T00:25:29.4534409Z 1391 |             IntersectionInner::Stitch { a, b } => {
2020-02-22T00:25:29.4535305Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4536592Z      = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4540547Z 
2020-02-22T00:25:29.4568695Z error: lifetime may not live long enough
2020-02-22T00:25:29.4569299Z     --> src/liballoc/collections/btree/set.rs:1414:13
2020-02-22T00:25:29.4569299Z     --> src/liballoc/collections/btree/set.rs:1414:13
2020-02-22T00:25:29.4569745Z      |
2020-02-22T00:25:29.4570339Z 1386 | impl<'a, T: Ord> Iterator for Intersection<'a, T> {
2020-02-22T00:25:29.4571523Z ...
2020-02-22T00:25:29.4571523Z ...
2020-02-22T00:25:29.4572164Z 1414 |             IntersectionInner::Stitch { a, b } => (0, Some(min(a.len(), b.len()))),
2020-02-22T00:25:29.4573112Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
2020-02-22T00:25:29.4574247Z      = help: consider replacing `'a` with `'static`
2020-02-22T00:25:29.4577578Z 
2020-02-22T00:25:29.5943134Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-02-22T00:25:29.7668761Z    Compiling backtrace v0.3.44
2020-02-22T00:25:29.7668761Z    Compiling backtrace v0.3.44
2020-02-22T00:25:29.8854904Z error: aborting due to 10 previous errors
2020-02-22T00:25:29.8858997Z 
2020-02-22T00:25:29.8914582Z error: could not compile `alloc`.
2020-02-22T00:25:29.8915388Z warning: build failed, waiting for other jobs to finish...
2020-02-22T00:25:29.8965731Z error[E0308]: mismatched types
2020-02-22T00:25:29.8966954Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:74:13
2020-02-22T00:25:29.8967525Z    |
2020-02-22T00:25:29.8968015Z 73 |         match *self {
2020-02-22T00:25:29.8968851Z    |               ----- this expression has type `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.8969761Z 74 |             Symbol::Syminfo { symname, .. } => symbol(symname),
2020-02-22T00:25:29.8971006Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
2020-02-22T00:25:29.8972230Z    = note: expected enum `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.8972230Z    = note: expected enum `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.8972910Z               found enum `symbolize::libbacktrace::Symbol<'static>`
2020-02-22T00:25:29.8973487Z note: the lifetime `'_` as defined on the impl at 60:13...
2020-02-22T00:25:29.8974142Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:60:13
2020-02-22T00:25:29.8975042Z 60 | impl Symbol<'_> {
2020-02-22T00:25:29.8975515Z    |             ^^
2020-02-22T00:25:29.8975515Z    |             ^^
2020-02-22T00:25:29.8976198Z    = note: ...does not necessarily outlive the static lifetime
2020-02-22T00:25:29.9000453Z error[E0308]: mismatched types
2020-02-22T00:25:29.9001255Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:98:13
2020-02-22T00:25:29.9001844Z    |
2020-02-22T00:25:29.9001844Z    |
2020-02-22T00:25:29.9002507Z 97 |         let pc = match *self {
2020-02-22T00:25:29.9003688Z    |                        ----- this expression has type `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9004688Z 98 |             Symbol::Syminfo { pc, .. } => pc,
2020-02-22T00:25:29.9005376Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
2020-02-22T00:25:29.9006437Z    = note: expected enum `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9006437Z    = note: expected enum `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9007471Z               found enum `symbolize::libbacktrace::Symbol<'static>`
2020-02-22T00:25:29.9008319Z note: the lifetime `'_` as defined on the impl at 60:13...
2020-02-22T00:25:29.9009050Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:60:13
2020-02-22T00:25:29.9010056Z 60 | impl Symbol<'_> {
2020-02-22T00:25:29.9010921Z    |             ^^
2020-02-22T00:25:29.9010921Z    |             ^^
2020-02-22T00:25:29.9011505Z    = note: ...does not necessarily outlive the static lifetime
2020-02-22T00:25:29.9031966Z error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
2020-02-22T00:25:29.9032771Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:110:15
2020-02-22T00:25:29.9033255Z     |
2020-02-22T00:25:29.9033684Z 110 |         match *self {
2020-02-22T00:25:29.9033684Z 110 |         match *self {
2020-02-22T00:25:29.9034193Z     |               ^^^^^
2020-02-22T00:25:29.9034573Z     |
2020-02-22T00:25:29.9035111Z note: first, the lifetime cannot outlive the lifetime `'_` as defined on the impl at 60:13...
2020-02-22T00:25:29.9035866Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:60:13
2020-02-22T00:25:29.9036961Z 60  | impl Symbol<'_> {
2020-02-22T00:25:29.9037437Z     |             ^^
2020-02-22T00:25:29.9037896Z note: ...so that the types are compatible
2020-02-22T00:25:29.9038529Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:110:15
2020-02-22T00:25:29.9038529Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:110:15
2020-02-22T00:25:29.9039000Z     |
2020-02-22T00:25:29.9039422Z 110 |         match *self {
2020-02-22T00:25:29.9039930Z     |               ^^^^^
2020-02-22T00:25:29.9040648Z     = note: expected  `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9041255Z                found  `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9041879Z     = note: but, the lifetime must be valid for the static lifetime...
2020-02-22T00:25:29.9042414Z note: ...so that the types are compatible
2020-02-22T00:25:29.9043048Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:111:13
2020-02-22T00:25:29.9044017Z 111 |             Symbol::Syminfo { .. } => None,
2020-02-22T00:25:29.9044613Z     |             ^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T00:25:29.9044613Z     |             ^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T00:25:29.9046793Z     = note: expected  `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9048100Z                found  `symbolize::libbacktrace::Symbol<'static>`
2020-02-22T00:25:29.9059330Z error[E0308]: mismatched types
2020-02-22T00:25:29.9060997Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:149:13
2020-02-22T00:25:29.9061779Z     |
2020-02-22T00:25:29.9062532Z 148 |         match *self {
2020-02-22T00:25:29.9062532Z 148 |         match *self {
2020-02-22T00:25:29.9065227Z     |               ----- this expression has type `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9066421Z 149 |             Symbol::Syminfo { .. } => None,
2020-02-22T00:25:29.9067451Z     |             ^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
2020-02-22T00:25:29.9068016Z     |
2020-02-22T00:25:29.9068957Z     = note: expected enum `symbolize::libbacktrace::Symbol<'_>`
2020-02-22T00:25:29.9069639Z                found enum `symbolize::libbacktrace::Symbol<'static>`
2020-02-22T00:25:29.9070303Z note: the lifetime `'_` as defined on the impl at 60:13...
2020-02-22T00:25:29.9071607Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/symbolize/libbacktrace.rs:60:13
2020-02-22T00:25:29.9072858Z 60  | impl Symbol<'_> {
2020-02-22T00:25:29.9073687Z     |             ^^
2020-02-22T00:25:29.9073687Z     |             ^^
2020-02-22T00:25:29.9074737Z     = note: ...does not necessarily outlive the static lifetime
2020-02-22T00:25:29.9142125Z error: aborting due to 4 previous errors
2020-02-22T00:25:29.9142394Z 
2020-02-22T00:25:29.9142855Z Some errors have detailed explanations: E0308, E0495.
2020-02-22T00:25:29.9143835Z For more information about an error, try `rustc --explain E0308`.
---
2020-02-22T00:25:29.9345554Z   local time: Sat Feb 22 00:25:29 UTC 2020
2020-02-22T00:25:30.2244256Z   network time: Sat, 22 Feb 2020 00:25:30 GMT
2020-02-22T00:25:30.2246977Z == end clock drift check ==
2020-02-22T00:25:31.5691494Z 
2020-02-22T00:25:31.5757338Z ##[error]Bash exited with code '1'.
2020-02-22T00:25:31.5769359Z ##[section]Finishing: Run build
2020-02-22T00:25:31.5815843Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-22T00:25:31.5819932Z Task         : Get sources
2020-02-22T00:25:31.5820227Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T00:25:31.5820486Z Version      : 1.0.0
2020-02-22T00:25:31.5820665Z Author       : Microsoft
2020-02-22T00:25:31.5820665Z Author       : Microsoft
2020-02-22T00:25:31.5820965Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T00:25:31.5821297Z ==============================================================================
2020-02-22T00:25:31.8763056Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T00:25:31.8803412Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-22T00:25:31.8897618Z Cleaning up task key
2020-02-22T00:25:31.8898679Z Start cleaning up orphan processes.
2020-02-22T00:25:31.9077877Z Terminate orphan process: pid (4754) (python)
2020-02-22T00:25:31.9301284Z ##[section]Finishing: Finalize Job
