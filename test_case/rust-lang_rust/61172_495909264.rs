plain
travis_time:end:03dabdcc:start=1558778262430384724,finish=1558778263416338206,duration=985953482
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:19] .................................................................................................... 4400/5585
[01:14:22] .................................................................................................... 4500/5585
[01:14:26] .................................................................................................... 4600/5585
[01:14:30] .................................................................................................... 4700/5585
[01:14:37] .........................................F.F........................................................ 4800/5585
[01:14:46] .................................................................................................... 5000/5585
[01:14:51] .................................................................................................... 5100/5585
[01:14:54] .................................................................................................... 5200/5585
[01:14:58] .................................................................................................... 5300/5585
---
[01:15:07] 1 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:13:47
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:18:61
[01:15:07] 3    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b> {
[01:15:07] -    |                                               ^^^^^     ^^^^^
[01:15:07] + LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
[01:15:07] 6    |
[01:15:07] 7 note: lint level defined here
[01:15:07] 8   --> $DIR/edition-lint-infer-outlives-multispan.rs:2:9
[01:15:07] 
[01:15:07] 
[01:15:07] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:15:07] 12 help: remove these bounds
[01:15:07] 13    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:18:61
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] 24 LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: Debug {
[01:15:07] 26 
[01:15:07] 
[01:15:07] 27 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:23:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:23:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:41:54
[01:15:07] 29    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:29:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:35:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:41:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] 60 LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U> where U: 'b {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T, U> {
[01:15:07] +    |                                                      ^^^^^^^^^^^^ help: remove this bound
[01:15:07] 66 
[01:15:07] 67 error: outlives requirements can be inferred
[01:15:07] 68   --> $DIR/edition-lint-infer-outlives-multispan.rs:47:67
[01:15:07] 68   --> $DIR/edition-lint-infer-outlives-multispan.rs:47:67
[01:15:07] 
[01:15:07] 75    |                                                                  --   --
[01:15:07] 76 
[01:15:07] 77 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:53:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:53:71
[01:15:07] 79    |
[01:15:07] 80 LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U> where U: 'b + Debug {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where U: Debug {
[01:15:07] +    |                                                                       ^^^^^ help: remove this bound
[01:15:07] 86 
[01:15:07] 87 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:59:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:59:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:59:76
[01:15:07] 89    |
[01:15:07] 90 LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U> where U: Debug + 'b {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where U: Debug {
[01:15:07] +    |                                                                            ^^^^^ help: remove this bound
[01:15:07] 96 
[01:15:07] 97 error: outlives requirements can be inferred
[01:15:07] 98   --> $DIR/edition-lint-infer-outlives-multispan.rs:65:69
[01:15:07] 98   --> $DIR/edition-lint-infer-outlives-multispan.rs:65:69
[01:15:07] 
[01:15:07] 115    |                                                                    --      --
[01:15:07] 116 
[01:15:07] 117 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:77:38
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeBee<'a, 'b, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:82:40
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:87:55
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 148   --> $DIR/edition-lint-infer-outlives-multispan.rs:92:68
[01:15:07] 149    |
[01:15:07] 149    |
[01:15:07] 150 LL |     struct BeeWhereAyTeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where 'b: 'a, T: 'a + Debug + 'b {
[01:15:07] 155    |                                                                   -- --   --
[01:15:07] 156 
[01:15:07] 157 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:97:58
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:97:58
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 168   --> $DIR/edition-lint-infer-outlives-multispan.rs:104:18
[01:15:07] 169    |
[01:15:07] 169    |
[01:15:07] 170 LL |         where U: 'a + Debug + 'b, 'b: 'a
[01:15:07] 175    |                 --   ----
[01:15:07] 176 
[01:15:07] 177 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:115:47
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:115:47
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b>(&'a &'b T);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 188   --> $DIR/edition-lint-infer-outlives-multispan.rs:118:72
[01:15:07] 189    |
[01:15:07] 189    |
[01:15:07] 190 LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T>(&'a &'b T) where T: 'a + Debug + 'b;
[01:15:07] 195    |                                                                       --   --
[01:15:07] 196 
[01:15:07] 197 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:121:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:121:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:130:69
[01:15:07] 199    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b>(T, &'a &'b U);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug>(T, &'a &'b U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:124:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug>(&'a T, &'b U);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug>(&'a T, &'b U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:127:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b>(&'a T, &'b U);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug>(&'a T, &'b U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:130:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] 230 LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b;
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T, U>(&'a T, &'b U) ;
[01:15:07] +    |                                                                     ^^^^^^^^^^^ help: remove this bound
[01:15:07] 236 
[01:15:07] 237 error: outlives requirements can be inferred
[01:15:07] 238   --> $DIR/edition-lint-infer-outlives-multispan.rs:133:81
[01:15:07] 238   --> $DIR/edition-lint-infer-outlives-multispan.rs:133:81
[01:15:07] 
[01:15:07] 245    |                                                                                --   --
[01:15:07] 246 
[01:15:07] 247 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:136:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:136:85
[01:15:07] 249    |
[01:15:07] 250 LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b + Debug;
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;
[01:15:07] +    |                                                                                     ^^^^^ help: remove this bound
[01:15:07] 256 
[01:15:07] 257 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:139:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:139:53
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:139:90
[01:15:07] 259    |
[01:15:07] 260 LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: Debug + 'b;
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;
[01:15:07] +    |                                                                                          ^^^^^ help: remove this bound
[01:15:07] 266 
[01:15:07] 267 error: outlives requirements can be inferred
[01:15:07] 268   --> $DIR/edition-lint-infer-outlives-multispan.rs:142:75
[01:15:07] 268   --> $DIR/edition-lint-infer-outlives-multispan.rs:142:75
[01:15:07] 
[01:15:07] 285    |                                                                          --      --
[01:15:07] 286 
[01:15:07] 287 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:148:38
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b>(&'a &'b T);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeBee<'a, 'b, T>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:151:40
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b>(&'a &'b T);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b, T>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:154:55
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b>(&'a &'b T);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b, T: Debug>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 318   --> $DIR/edition-lint-infer-outlives-multispan.rs:157:71
[01:15:07] 319    |
[01:15:07] 319    |
[01:15:07] 320 LL |     struct BeeWhereAyTeeWhereAyIsDebugBee<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'a + Debug + 'b;
[01:15:07] 325    |                                                                      -- --   --
[01:15:07] 326 
[01:15:07] 327 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:160:58
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:160:58
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b>(T, &'a &'b U);
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug>(T, &'a &'b U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 338   --> $DIR/edition-lint-infer-outlives-multispan.rs:164:18
[01:15:07] 339    |
[01:15:07] 339    |
[01:15:07] 340 LL |         where U: 'a + Debug + 'b, 'b: 'a;
[01:15:07] 345    |                 --   ----
[01:15:07] 346 
[01:15:07] 347 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:171:45
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:171:45
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 358   --> $DIR/edition-lint-infer-outlives-multispan.rs:176:59
[01:15:07] 359    |
[01:15:07] 359    |
[01:15:07] 360 LL |     enum TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
[01:15:07] 365    |                                                          --   --
[01:15:07] 366 
[01:15:07] 367 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:181:51
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:181:51
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:199:52
[01:15:07] 369    |
[01:15:07] - LL |     enum TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:187:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:193:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:199:44
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] 400 LL |     enum TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U> where U: 'b {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooWhereBee<'a, 'b, T, U> {
[01:15:07] +    |                                                    ^^^^^^^^^^^^ help: remove this bound
[01:15:07] 406 
[01:15:07] 407 error: outlives requirements can be inferred
[01:15:07] 408   --> $DIR/edition-lint-infer-outlives-multispan.rs:205:65
[01:15:07] 408   --> $DIR/edition-lint-infer-outlives-multispan.rs:205:65
[01:15:07] 
[01:15:07] 415    |                                                                --   --
[01:15:07] 416 
[01:15:07] 417 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:211:51
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:211:69
[01:15:07] 419    |
[01:15:07] 420 LL |     enum TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U> where U: 'b + Debug {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where U: Debug {
[01:15:07] +    |                                                                     ^^^^^ help: remove this bound
[01:15:07] 426 
[01:15:07] 427 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:217:51
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:217:51
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:217:74
[01:15:07] 429    |
[01:15:07] 430 LL |     enum TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U> where U: Debug + 'b {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where U: Debug {
[01:15:07] +    |                                                                          ^^^^^ help: remove this bound
[01:15:07] 436 
[01:15:07] 437 error: outlives requirements can be inferred
[01:15:07] 438   --> $DIR/edition-lint-infer-outlives-multispan.rs:223:67
[01:15:07] 438   --> $DIR/edition-lint-infer-outlives-multispan.rs:223:67
[01:15:07] 
[01:15:07] 455    |                                                                  --      --
[01:15:07] 456 
[01:15:07] 457 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:235:36
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeBee<'a, 'b, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:240:38
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeAyBee<'a, 'b, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:246:53
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 488   --> $DIR/edition-lint-infer-outlives-multispan.rs:251:66
[01:15:07] 489    |
[01:15:07] 489    |
[01:15:07] 490 LL |     enum BeeWhereAyTeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where 'b: 'a, T: 'a + Debug + 'b {
[01:15:07] 495    |                                                                 -- --   --
[01:15:07] 496 
[01:15:07] 497 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:256:56
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:256:56
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 508   --> $DIR/edition-lint-infer-outlives-multispan.rs:262:75
[01:15:07] 509    |
[01:15:07] 509    |
[01:15:07] 510 LL |     enum BeeWhereAyTeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U> where U: 'a + Debug + 'b, 'b: 'a {
[01:15:07] 515    |                                                                          --   ----
[01:15:07] 516 
[01:15:07] 517 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:271:46
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:271:46
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 528   --> $DIR/edition-lint-infer-outlives-multispan.rs:276:60
[01:15:07] 529    |
[01:15:07] 529    |
[01:15:07] 530 LL |     union TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
[01:15:07] 535    |                                                           --   --
[01:15:07] 536 
[01:15:07] 537 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:281:52
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:281:52
[01:15:07] +   --> $DIR/edition-lint-infer-outlives-multispan.rs:299:53
[01:15:07] 539    |
[01:15:07] - LL |     union TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:287:47
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives-multispan.rs:293:47
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b> {
[01:15:07] - help: remove these bounds
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     union TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug> {
[01:15:07] - 
---
[01:15:07] 1 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:26:31
[01:15:07] +   --> $DIR/edition-lint-infer-outlives.rs:56:37
[01:15:07] 3    |
[01:15:07] - LL |     struct TeeOutlivesAy<'a, T: 'a> {
[01:15:07] -    |                               ^^^^ help: remove this bound
[01:15:07] + LL |     struct TeeWhereOutlivesAy<'a, T> where T: 'a {
[01:15:07] 6    |
[01:15:07] 7 note: lint level defined here
[01:15:07] 8   --> $DIR/edition-lint-infer-outlives.rs:4:9
[01:15:07] 
[01:15:07] 
[01:15:07] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:15:07] 12 
[01:15:07] 13 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:31:40
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:36:45
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:41:38
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
[01:15:07] -    |                                      ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:46:47
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
[01:15:07] -    |                                               ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:51:52
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
[01:15:07] -    |                                                    ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:56:37
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeWhereOutlivesAy<'a, T> where T: 'a {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 50   --> $DIR/edition-lint-infer-outlives.rs:61:54
[01:15:07] 51    |
[01:15:07] 51    |
[01:15:07] 52 LL |     struct TeeWhereOutlivesAyIsDebug<'a, T> where T: 'a + Debug {
[01:15:07] 77    |                                                                  ^^^^^^^^^^ help: remove these bounds
[01:15:07] 78 
[01:15:07] 79 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:86:37
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:86:37
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAy<'a, T, U: 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:92:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:98:51
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:104:41
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:110:44
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
[01:15:07] -    |                                            ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:116:53
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
[01:15:07] -    |                                                     ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:122:58
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
[01:15:07] -    |                                                          ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:128:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
[01:15:07] -    |                                                ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 128   --> $DIR/edition-lint-infer-outlives.rs:134:43
[01:15:07] 129    |
[01:15:07] 129    |
[01:15:07] 130 LL |     struct TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {
[01:15:07] 143    |                                                                 ^^^^^ help: remove this bound
[01:15:07] 144 
[01:15:07] 145 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:152:46
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:152:46
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 152   --> $DIR/edition-lint-infer-outlives.rs:158:50
[01:15:07] 153    |
[01:15:07] 153    |
[01:15:07] 154 LL |     struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
[01:15:07] 167    |                                                                        ^^^^^^^^^^ help: remove these bounds
[01:15:07] 168 
[01:15:07] 169 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:176:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:176:53
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
[01:15:07] -    |                                                     ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 176   --> $DIR/edition-lint-infer-outlives.rs:182:62
[01:15:07] 177    |
[01:15:07] 177    |
[01:15:07] 178 LL |     struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {
[01:15:07] 185    |                                                                     ^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 186 
[01:15:07] 187 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:194:32
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:194:32
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAy<'a, 'b: 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 194   --> $DIR/edition-lint-infer-outlives.rs:199:38
[01:15:07] 195    |
[01:15:07] 195    |
[01:15:07] 196 LL |     struct BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {
[01:15:07] 197    |                                      ^^^^^^^^^^^^^ help: remove this bound
[01:15:07] 198 
[01:15:07] 199 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:204:35
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:204:35
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTee<'a, 'b: 'a, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 206   --> $DIR/edition-lint-infer-outlives.rs:209:44
[01:15:07] 207    |
[01:15:07] 207    |
[01:15:07] 208 LL |     struct BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {
[01:15:07] 221    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 222 
[01:15:07] 223 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:224:40
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:224:40
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 230   --> $DIR/edition-lint-infer-outlives.rs:229:61
[01:15:07] 231    |
[01:15:07] 231    |
[01:15:07] 232 LL |     struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T> where 'b: 'a, T: Debug {
[01:15:07] 233    |                                                             ^^^^^^^^ help: remove this bound
[01:15:07] 234 
[01:15:07] 235 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:238:31
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:238:31
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAy<'a, T: 'a>(&'a T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:241:40
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug>(&'a T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:244:45
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a>(&'a T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:247:38
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b>(&'a &'b T);
[01:15:07] -    |                                      ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:250:47
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug>(&'a &'b T);
[01:15:07] -    |                                               ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:253:52
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b>(&'a &'b T);
[01:15:07] -    |                                                    ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 272   --> $DIR/edition-lint-infer-outlives.rs:256:45
[01:15:07] 273    |
[01:15:07] 273    |
[01:15:07] 274 LL |     struct TeeWhereOutlivesAy<'a, T>(&'a T) where T: 'a;
[01:15:07] 305    |                                                                             ^^^^^^^^^^ help: remove these bounds
[01:15:07] 306 
[01:15:07] 307 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:274:37
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:274:37
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAy<'a, T, U: 'a>(T, &'a U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:277:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug>(T, &'a U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:280:51
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a>(T, &'a U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:283:41
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug>(&'a T, U);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:286:44
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b>(T, &'a &'b U);
[01:15:07] -    |                                            ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:289:53
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug>(T, &'a &'b U);
[01:15:07] -    |                                                     ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:292:58
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b>(T, &'a &'b U);
[01:15:07] -    |                                                          ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:295:48
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug>(&'a &'b T, U);
[01:15:07] -    |                                                ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 356   --> $DIR/edition-lint-infer-outlives.rs:298:54
[01:15:07] 357    |
[01:15:07] 357    |
[01:15:07] 358 LL |     struct TeeYooWhereOutlivesAy<'a, T, U>(T, &'a U) where U: 'a;
[01:15:07] 371    |                                                                           ^^^^^ help: remove this bound
[01:15:07] 372 
[01:15:07] 373 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:307:46
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:307:46
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U>(&'a T, U) where U: Debug;
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 380   --> $DIR/edition-lint-infer-outlives.rs:310:65
[01:15:07] 381    |
[01:15:07] 381    |
[01:15:07] 382 LL |     struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + 'b;
[01:15:07] 395    |                                                                                      ^^^^^^^^^^ help: remove these bounds
[01:15:07] 396 
[01:15:07] 397 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:319:53
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:319:53
[01:15:07] -    |
[01:15:07] - LL |     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U>(&'a &'b T, U) where U: Debug;
[01:15:07] -    |                                                     ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 404   --> $DIR/edition-lint-infer-outlives.rs:322:72
[01:15:07] 405    |
[01:15:07] 405    |
[01:15:07] 406 LL |     struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U>(&'a T, U) where T: 'a, U: Debug;
[01:15:07] 413    |                                                                           ^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 414 
[01:15:07] 415 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:328:32
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:328:32
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAy<'a, 'b: 'a>(&'a &'b ());
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 422   --> $DIR/edition-lint-infer-outlives.rs:331:51
[01:15:07] 423    |
[01:15:07] 423    |
[01:15:07] 424 LL |     struct BeeWhereOutlivesAy<'a, 'b>(&'a &'b ()) where 'b: 'a;
[01:15:07] 425    |                                                   ^^^^^^^^^^^^ help: remove this bound
[01:15:07] 426 
[01:15:07] 427 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:334:35
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:334:35
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTee<'a, 'b: 'a, T>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 434   --> $DIR/edition-lint-infer-outlives.rs:337:56
[01:15:07] 435    |
[01:15:07] 435    |
[01:15:07] 436 LL |     struct BeeWhereOutlivesAyTee<'a, 'b, T>(&'a &'b T) where 'b: 'a;
[01:15:07] 449    |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 450 
[01:15:07] 451 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:346:40
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:346:40
[01:15:07] -    |
[01:15:07] - LL |     struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug>(&'a &'b T);
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 458   --> $DIR/edition-lint-infer-outlives.rs:349:72
[01:15:07] 459    |
[01:15:07] 459    |
[01:15:07] 460 LL |     struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: Debug;
[01:15:07] 461    |                                                                        ^^^^^^^^ help: remove this bound
[01:15:07] 462 
[01:15:07] 463 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:356:29
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:356:29
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAy<'a, T: 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:361:38
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:366:43
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:372:36
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
[01:15:07] -    |                                    ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:378:45
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
[01:15:07] -    |                                             ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:383:50
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
[01:15:07] -    |                                                  ^^^^^^^^^^ help: remove these bounds
[01:15:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 500   --> $DIR/edition-lint-infer-outlives.rs:388:35
[01:15:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:07] 501    |
[01:15:07] 502 LL |     enum TeeWhereOutlivesAy<'a, T> where T: 'a {
[01:15:07] 533    |                                                                ^^^^^^^^^^ help: remove these bounds
[01:15:07] 534 
[01:15:07] 535 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:422:35
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:422:35
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooOutlivesAy<'a, T, U: 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:428:44
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:434:49
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:440:39
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:446:42
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
[01:15:07] -    |                                          ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:452:51
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
[01:15:07] -    |                                                   ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:458:56
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
[01:15:07] -    |                                                        ^^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:464:46
[01:15:07] -    |
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
[01:15:07] -    |                                              ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 584   --> $DIR/edition-lint-infer-outlives.rs:470:41
[01:15:07] 585    |
[01:15:07] 585    |
[01:15:07] 586 LL |     enum TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {
[01:15:07] 599    |                                                               ^^^^^ help: remove this bound
[01:15:07] 600 
[01:15:07] 601 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:488:44
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:488:44
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 608   --> $DIR/edition-lint-infer-outlives.rs:494:48
[01:15:07] 609    |
[01:15:07] 609    |
[01:15:07] 610 LL |     enum TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
[01:15:07] 623    |                                                                      ^^^^^^^^^^ help: remove these bounds
[01:15:07] 624 
[01:15:07] 625 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:512:51
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:512:51
[01:15:07] -    |
[01:15:07] - LL |     enum TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
[01:15:07] -    |                                                   ^^^^^^^^^ help: remove these bounds
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 632   --> $DIR/edition-lint-infer-outlives.rs:518:60
[01:15:07] 633    |
[01:15:07] 633    |
[01:15:07] 634 LL |     enum TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {
[01:15:07] 641    |                                                                   ^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 642 
[01:15:07] 643 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:530:30
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:530:30
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAy<'a, 'b: 'a> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 650   --> $DIR/edition-lint-infer-outlives.rs:535:36
[01:15:07] 651    |
[01:15:07] 651    |
[01:15:07] 652 LL |     enum BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {
[01:15:07] 653    |                                    ^^^^^^^^^^^^^ help: remove this bound
[01:15:07] 654 
[01:15:07] 655 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:540:33
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:540:33
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTee<'a, 'b: 'a, T> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 662   --> $DIR/edition-lint-infer-outlives.rs:546:42
[01:15:07] 663    |
[01:15:07] 663    |
[01:15:07] 664 LL |     enum BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {
[01:15:07] 677    |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these bounds
[01:15:07] 678 
[01:15:07] 679 error: outlives requirements can be inferred
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:563:38
[01:15:07] -   --> $DIR/edition-lint-infer-outlives.rs:563:38
[01:15:07] -    |
[01:15:07] - LL |     enum BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
[01:15:07] - 
[01:15:07] - error: outlives requirements can be inferred
[01:15:07] 686   --> $DIR/edition-lint-infer-outlives.rs:568:59
[01:15:07] 687    |
---
[01:15:07] 
[01:15:07] 23 mod structs {
[01:15:07] 24     use std::fmt::Debug;
[01:15:07] 25 
[01:15:07] -     struct TeeOutlivesAy<'a, T> {
[01:15:07] +     struct TeeOutlivesAy<'a, T: 'a> {
[01:15:07] 27         //~^ ERROR outlives requirements can be inferred
[01:15:07] 28         tee: &'a T
[01:15:07] 
[01:15:07] 30 
[01:15:07] 30 
[01:15:07] -     struct TeeOutlivesAyIsDebug<'a, T: Debug> {
[01:15:07] +     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
[01:15:07] 32         //~^ ERROR outlives requirements can be inferred
[01:15:07] 33         tee: &'a T
[01:15:07] 
[01:15:07] 35 
[01:15:07] 35 
[01:15:07] -     struct TeeIsDebugOutlivesAy<'a, T: Debug> {
[01:15:07] +     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
[01:15:07] 37         //~^ ERROR outlives requirements can be inferred
[01:15:07] 38         tee: &'a T
[01:15:07] 
[01:15:07] 40 
[01:15:07] 40 
[01:15:07] -     struct TeeOutlivesAyBee<'a, 'b, T> {
[01:15:07] +     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
[01:15:07] 42         //~^ ERROR outlives requirements can be inferred
[01:15:07] 43         tee: &'a &'b T
[01:15:07] 
[01:15:07] 45 
[01:15:07] 45 
[01:15:07] -     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: Debug> {
[01:15:07] +     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
[01:15:07] 47         //~^ ERROR outlives requirements can be inferred
[01:15:07] 48         tee: &'a &'b T
[01:15:07] 
[01:15:07] 50 
[01:15:07] 50 
[01:15:07] -     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug> {
[01:15:07] +     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
[01:15:07] 52         //~^ ERROR outlives requirements can be inferred
[01:15:07] 53         tee: &'a &'b T
[01:15:07] 
[01:15:07] 
[01:15:07] 83         tee: &'a &'b T
[01:15:07] 85 
[01:15:07] 85 
[01:15:07] -     struct TeeYooOutlivesAy<'a, T, U> {
[01:15:07] +     struct TeeYooOutlivesAy<'a, T, U: 'a> {
[01:15:07] 87         //~^ ERROR outlives requirements can be inferred
[01:15:07] 88         tee: T,
[01:15:07] 89         yoo: &'a U
[01:15:07] 90     }
[01:15:07] 91 
[01:15:07] 91 
[01:15:07] -     struct TeeYooOutlivesAyIsDebug<'a, T, U: Debug> {
[01:15:07] +     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
[01:15:07] 93         //~^ ERROR outlives requirements can be inferred
[01:15:07] 94         tee: T,
[01:15:07] 95         yoo: &'a U
[01:15:07] 96     }
[01:15:07] 97 
[01:15:07] 97 
[01:15:07] -     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug> {
[01:15:07] +     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
[01:15:07] 99         //~^ ERROR outlives requirements can be inferred
[01:15:07] 100         tee: T,
[01:15:07] 101         yoo: &'a U
[01:15:07] 102     }
[01:15:07] 103 
[01:15:07] 103 
[01:15:07] -     struct TeeOutlivesAyYooIsDebug<'a, T, U: Debug> {
[01:15:07] +     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
[01:15:07] 105         //~^ ERROR outlives requirements can be inferred
[01:15:07] 106         tee: &'a T,
[01:15:07] 107         yoo: U
[01:15:07] 108     }
[01:15:07] 109 
[01:15:07] 109 
[01:15:07] -     struct TeeYooOutlivesAyBee<'a, 'b, T, U> {
[01:15:07] +     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
[01:15:07] 111         //~^ ERROR outlives requirements can be inferred
[01:15:07] 112         tee: T,
[01:15:07] 113         yoo: &'a &'b U
[01:15:07] 114     }
[01:15:07] 115 
[01:15:07] 115 
[01:15:07] -     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] +     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
[01:15:07] 117         //~^ ERROR outlives requirements can be inferred
[01:15:07] 118         tee: T,
[01:15:07] 119         yoo: &'a &'b U
[01:15:07] 120     }
[01:15:07] 121 
[01:15:07] 121 
[01:15:07] -     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug> {
[01:15:07] +     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
[01:15:07] 123         //~^ ERROR outlives requirements can be inferred
[01:15:07] 124         tee: T,
[01:15:07] 125         yoo: &'a &'b U
[01:15:07] 126     }
[01:15:07] 127 
[01:15:07] 127 
[01:15:07] -     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] +     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
[01:15:07] 129         //~^ ERROR outlives requirements can be inferred
[01:15:07] 130         tee: &'a &'b T,
[01:15:07] 131         yoo: U
[01:15:07] 
[01:15:07] 149         yoo: &'a U
[01:15:07] 151 
[01:15:07] 151 
[01:15:07] -     struct TeeOutlivesAyYooWhereIsDebug<'a, T, U> where U: Debug {
[01:15:07] +     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
[01:15:07] 153         //~^ ERROR outlives requirements can be inferred
[01:15:07] 154         tee: &'a T,
[01:15:07] 155         yoo: U
[01:15:07] 
[01:15:07] 173         yoo: &'a &'b U
[01:15:07] 175 
[01:15:07] 175 
[01:15:07] -     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where U: Debug {
[01:15:07] +     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
[01:15:07] 177         //~^ ERROR outlives requirements can be inferred
[01:15:07] 178         tee: &'a &'b T,
[01:15:07] 179         yoo: U
[01:15:07] 191         yoo: U
[01:15:07] 192     }
[01:15:07] 193 
[01:15:07] 193 
[01:15:07] -     struct BeeOutlivesAy<'a, 'b> {
[01:15:07] +     struct BeeOutlivesAy<'a, 'b: 'a> {
[01:15:07] 195         //~^ ERROR outlives requirements can be inferred
[01:15:07] 196         tee: &'a &'b (),
[01:15:07] 
[01:15:07] 
[01:15:07] 201         tee: &'a &'b (),
[01:15:07] 203 
[01:15:07] 203 
[01:15:07] -     struct BeeOutlivesAyTee<'a, 'b, T> {
[01:15:07] +     struct BeeOutlivesAyTee<'a, 'b: 'a, T> {
[01:15:07] 205         //~^ ERROR outlives requirements can be inferred
[01:15:07] 206         tee: &'a &'b T,
[01:15:07] 
[01:15:07] 
[01:15:07] 221         tee: &'a &'b T,
[01:15:07] 223 
[01:15:07] 223 
[01:15:07] -     struct BeeOutlivesAyTeeDebug<'a, 'b, T: Debug> {
[01:15:07] +     struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
[01:15:07] 225         //~^ ERROR outlives requirements can be inferred
[01:15:07] 226         tee: &'a &'b T,
[01:15:07] 
[01:15:07] 235 mod tuple_structs {
[01:15:07] 236     use std::fmt::Debug;
[01:15:07] 237 
[01:15:07] 237 
[01:15:07] -     struct TeeOutlivesAy<'a, T>(&'a T);
[01:15:07] +     struct TeeOutlivesAy<'a, T: 'a>(&'a T);
[01:15:07] 239     //~^ ERROR outlives requirements can be inferred
[01:15:07] 240 
[01:15:07] -     struct TeeOutlivesAyIsDebug<'a, T: Debug>(&'a T);
[01:15:07] +     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug>(&'a T);
[01:15:07] 242     //~^ ERROR outlives requirements can be inferred
[01:15:07] 243 
[01:15:07] -     struct TeeIsDebugOutlivesAy<'a, T: Debug>(&'a T);
[01:15:07] +     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a>(&'a T);
[01:15:07] 245     //~^ ERROR outlives requirements can be inferred
[01:15:07] 246 
[01:15:07] -     struct TeeOutlivesAyBee<'a, 'b, T>(&'a &'b T);
[01:15:07] +     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b>(&'a &'b T);
[01:15:07] 248     //~^ ERROR outlives requirements can be inferred
[01:15:07] 249 
[01:15:07] -     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: Debug>(&'a &'b T);
[01:15:07] +     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug>(&'a &'b T);
[01:15:07] 251     //~^ ERROR outlives requirements can be inferred
[01:15:07] 252 
[01:15:07] -     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug>(&'a &'b T);
[01:15:07] +     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b>(&'a &'b T);
[01:15:07] 254     //~^ ERROR outlives requirements can be inferred
[01:15:07] 255 
[01:15:07] 256     struct TeeWhereOutlivesAy<'a, T>(&'a T) ;
[01:15:07] 
[01:15:07] 271     struct TeeWhereIsDebugOutlivesAyBee<'a, 'b, T>(&'a &'b T) where T: Debug;
[01:15:07] 272     //~^ ERROR outlives requirements can be inferred
[01:15:07] 273 
[01:15:07] -     struct TeeYooOutlivesAy<'a, T, U>(T, &'a U);
[01:15:07] +     struct TeeYooOutlivesAy<'a, T, U: 'a>(T, &'a U);
[01:15:07] 275     //~^ ERROR outlives requirements can be inferred
[01:15:07] 276 
[01:15:07] -     struct TeeYooOutlivesAyIsDebug<'a, T, U: Debug>(T, &'a U);
[01:15:07] +     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug>(T, &'a U);
[01:15:07] 278     //~^ ERROR outlives requirements can be inferred
[01:15:07] 279 
[01:15:07] -     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug>(T, &'a U);
[01:15:07] +     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a>(T, &'a U);
[01:15:07] 281     //~^ ERROR outlives requirements can be inferred
[01:15:07] 282 
[01:15:07] -     struct TeeOutlivesAyYooIsDebug<'a, T, U: Debug>(&'a T, U);
[01:15:07] +     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug>(&'a T, U);
[01:15:07] 284     //~^ ERROR outlives requirements can be inferred
[01:15:07] 285 
[01:15:07] -     struct TeeYooOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U);
[01:15:07] +     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b>(T, &'a &'b U);
[01:15:07] 287     //~^ ERROR outlives requirements can be inferred
[01:15:07] 288 
[01:15:07] -     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: Debug>(T, &'a &'b U);
[01:15:07] +     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug>(T, &'a &'b U);
[01:15:07] 290     //~^ ERROR outlives requirements can be inferred
[01:15:07] 291 
[01:15:07] -     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug>(T, &'a &'b U);
[01:15:07] +     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b>(T, &'a &'b U);
[01:15:07] 293     //~^ ERROR outlives requirements can be inferred
[01:15:07] 294 
[01:15:07] -     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T, U: Debug>(&'a &'b T, U);
[01:15:07] +     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug>(&'a &'b T, U);
[01:15:07] 296     //~^ ERROR outlives requirements can be inferred
[01:15:07] 297 
[01:15:07] 298     struct TeeYooWhereOutlivesAy<'a, T, U>(T, &'a U) ;
[01:15:07] 
[01:15:07] 304     struct TeeYooWhereIsDebugOutlivesAy<'a, T, U>(T, &'a U) where U: Debug;
[01:15:07] 305     //~^ ERROR outlives requirements can be inferred
[01:15:07] 306 
[01:15:07] -     struct TeeOutlivesAyYooWhereIsDebug<'a, T, U>(&'a T, U) where U: Debug;
[01:15:07] +     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U>(&'a T, U) where U: Debug;
[01:15:07] 308     //~^ ERROR outlives requirements can be inferred
[01:15:07] 309 
[01:15:07] 310     struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) ;
[01:15:07] 
[01:15:07] 316     struct TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: Debug;
[01:15:07] 317     //~^ ERROR outlives requirements can be inferred
[01:15:07] 318 
[01:15:07] -     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U>(&'a &'b T, U) where U: Debug;
[01:15:07] +     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U>(&'a &'b T, U) where U: Debug;
[01:15:07] 320     //~^ ERROR outlives requirements can be inferred
[01:15:07] 321 
[01:15:07] 322     struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U>(&'a T, U) where U: Debug;
[01:15:07] 
[01:15:07] 325     struct TeeWhereAyBeeYooWhereIsDebug<'a, 'b, T, U>(&'a &'b T, U) where U: Debug;
[01:15:07] 326     //~^ ERROR outlives requirements can be inferred
[01:15:07] 327 
[01:15:07] -     struct BeeOutlivesAy<'a, 'b>(&'a &'b ());
[01:15:07] +     struct BeeOutlivesAy<'a, 'b: 'a>(&'a &'b ());
[01:15:07] 329     //~^ ERROR outlives requirements can be inferred
[01:15:07] 330 
[01:15:07] 331     struct BeeWhereOutlivesAy<'a, 'b>(&'a &'b ()) ;
[01:15:07] 332     //~^ ERROR outlives requirements can be inferred
[01:15:07] 333 
[01:15:07] 333 
[01:15:07] -     struct BeeOutlivesAyTee<'a, 'b, T>(&'a &'b T);
[01:15:07] +     struct BeeOutlivesAyTee<'a, 'b: 'a, T>(&'a &'b T);
[01:15:07] 335     //~^ ERROR outlives requirements can be inferred
[01:15:07] 336 
[01:15:07] 337     struct BeeWhereOutlivesAyTee<'a, 'b, T>(&'a &'b T) ;
[01:15:07] 
[01:15:07] 343     struct BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T>(&'a &'b T) ;
[01:15:07] 344     //~^ ERROR outlives requirements can be inferred
[01:15:07] 345 
[01:15:07] -     struct BeeOutlivesAyTeeDebug<'a, 'b, T: Debug>(&'a &'b T);
[01:15:07] +     struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug>(&'a &'b T);
[01:15:07] 347     //~^ ERROR outlives requirements can be inferred
[01:15:07] 348 
[01:15:07] 349     struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T>(&'a &'b T) where T: Debug;
[01:15:07] 353 mod enums {
[01:15:07] 354     use std::fmt::Debug;
[01:15:07] 355 
[01:15:07] 355 
[01:15:07] -     enum TeeOutlivesAy<'a, T> {
[01:15:07] +     enum TeeOutlivesAy<'a, T: 'a> {
[01:15:07] 357         //~^ ERROR outlives requirements can be inferred
[01:15:07] 358         V { tee: &'a T },
[01:15:07] 
[01:15:07] 360 
[01:15:07] 360 
[01:15:07] -     enum TeeOutlivesAyIsDebug<'a, T: Debug> {
[01:15:07] +     enum TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
[01:15:07] 362         //~^ ERROR outlives requirements can be inferred
[01:15:07] 363         V(&'a T),
[01:15:07] 
[01:15:07] 365 
[01:15:07] 365 
[01:15:07] -     enum TeeIsDebugOutlivesAy<'a, T: Debug> {
[01:15:07] +     enum TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
[01:15:07] 367         //~^ ERROR outlives requirements can be inferred
[01:15:07] 368         V { tee: &'a T },
[01:15:07] 369         W,
[01:15:07] 370     }
[01:15:07] 371 
[01:15:07] 371 
[01:15:07] -     enum TeeOutlivesAyBee<'a, 'b, T> {
[01:15:07] +     enum TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
[01:15:07] 373         //~^ ERROR outlives requirements can be inferred
[01:15:07] 374         V(&'a &'b T),
[01:15:07] 375         W,
[01:15:07] 376     }
[01:15:07] 377 
[01:15:07] 377 
[01:15:07] -     enum TeeOutlivesAyBeeIsDebug<'a, 'b, T: Debug> {
[01:15:07] +     enum TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
[01:15:07] 379         //~^ ERROR outlives requirements can be inferred
[01:15:07] 380         V { tee: &'a &'b T },
[01:15:07] 
[01:15:07] 382 
[01:15:07] 382 
[01:15:07] -     enum TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug> {
[01:15:07] make: *** [check] Error 1
[01:15:07] +     enum TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
[01:15:07] 384         //~^ ERROR outlives requirements can be inferred
[01:15:07] 385         V(&'a &'b T),
[01:15:07] 
[01:15:07] 419         W,
[01:15:07] 420     }
[01:15:07] 421 
[01:15:07] 421 
[01:15:07] -     enum TeeYooOutlivesAy<'a, T, U> {
[01:15:07] +     enum TeeYooOutlivesAy<'a, T, U: 'a> {
[01:15:07] 423         //~^ ERROR outlives requirements can be inferred
[01:15:07] 424         V { tee: T },
[01:15:07] 425         W(&'a U),
[01:15:07] 426     }
[01:15:07] 427 
[01:15:07] 427 
[01:15:07] -     enum TeeYooOutlivesAyIsDebug<'a, T, U: Debug> {
[01:15:07] +     enum TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
[01:15:07] 429         //~^ ERROR outlives requirements can be inferred
[01:15:07] 430         V { tee: T, yoo: &'a U },
[01:15:07] 431         W,
[01:15:07] 432     }
[01:15:07] 433 
[01:15:07] 433 
[01:15:07] -     enum TeeYooIsDebugOutlivesAy<'a, T, U: Debug> {
[01:15:07] +     enum TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
[01:15:07] 435         //~^ ERROR outlives requirements can be inferred
[01:15:07] 436         V(T, &'a U),
[01:15:07] 437         W,
[01:15:07] 438     }
[01:15:07] 439 
[01:15:07] 439 
[01:15:07] -     enum TeeOutlivesAyYooIsDebug<'a, T, U: Debug> {
[01:15:07] +     enum TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
[01:15:07] 441         //~^ ERROR outlives requirements can be inferred
[01:15:07] 442         V { tee: &'a T },
[01:15:07] 443         W(U),
[01:15:07] 444     }
[01:15:07] 445 
[01:15:07] 445 
[01:15:07] -     enum TeeYooOutlivesAyBee<'a, 'b, T, U> {
[01:15:07] +     enum TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
[01:15:07] 447         //~^ ERROR outlives requirements can be inferred
[01:15:07] 448         V { tee: T, yoo: &'a &'b U },
[01:15:07] 449         W,
[01:15:07] 450     }
[01:15:07] 451 
[01:15:07] 451 
[01:15:07] -     enum TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] +     enum TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
[01:15:07] 453         //~^ ERROR outlives requirements can be inferred
[01:15:07] 454         V(T, &'a &'b U),
[01:15:07] 455         W,
[01:15:07] 456     }
[01:15:07] 457 
[01:15:07] 457 
[01:15:07] -     enum TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug> {
[01:15:07] +     enum TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
[01:15:07] 459         //~^ ERROR outlives requirements can be inferred
[01:15:07] 460         V { tee: T, yoo: &'a &'b U },
[01:15:07] 461         W,
[01:15:07] 462     }
[01:15:07] 463 
[01:15:07] 463 
[01:15:07] -     enum TeeOutlivesAyBeeYooIsDebug<'a, 'b, T, U: Debug> {
[01:15:07] +     enum TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
[01:15:07] 465         //~^ ERROR outlives requirements can be inferred
[01:15:07] 466         V(&'a &'b T, U),
[01:15:07] 467         W,
[01:15:07] 485         W,
[01:15:07] 486     }
[01:15:07] 487 
[01:15:07] 487 
[01:15:07] -     enum TeeOutlivesAyYooWhereIsDebug<'a, T, U> where U: Debug {
[01:15:07] +     enum TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
[01:15:07] 489         //~^ ERROR outlives requirements can be inferred
[01:15:07] 490         V { tee: &'a T },
[01:15:07] 491         W(U),
[01:15:07] 
[01:15:07] 509         W(&'a &'b U),
[01:15:07] 511 
[01:15:07] 511 
[01:15:07] -     enum TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where U: Debug {
[01:15:07] +     enum TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
[01:15:07] 513         //~^ ERROR outlives requirements can be inferred
[01:15:07] 514         V { tee: &'a &'b T, yoo: U },
[01:15:07] 515         W,
[01:15:07] 527         W(U),
[01:15:07] 528     }
[01:15:07] 529 
[01:15:07] 529 
[01:15:07] -     enum BeeOutlivesAy<'a, 'b> {
[01:15:07] +     enum BeeOutlivesAy<'a, 'b: 'a> {
[01:15:07] 531         //~^ ERROR outlives requirements can be inferred
[01:15:07] 532         V { tee: &'a &'b () },
[01:15:07] 
[01:15:07] 
[01:15:07] 537         V(&'a &'b ()),
[01:15:07] 539 
[01:15:07] 539 
[01:15:07] -     enum BeeOutlivesAyTee<'a, 'b, T> {
[01:15:07] +     enum BeeOutlivesAyTee<'a, 'b: 'a, T> {
[01:15:07] 541         //~^ ERROR outlives requirements can be inferred
[01:15:07] 542         V { tee: &'a &'b T },
[01:15:07] 543         W,
[01:15:07] 560         W,
[01:15:07] 561     }
[01:15:07] 562 
[01:15:07] 562 
[01:15:07] -     enum BeeOutlivesAyTeeDebug<'a, 'b, T: Debug> {
[01:15:07] +     enum BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
[01:15:07] 564         //~^ ERROR outlives requirements can be inferred
[01:15:07] 565         V { tee: &'a &'b T },
[01:15:07] 
[01:15:07] 574 mod unions {
[01:15:07] 575     use std::fmt::Debug;
[01:15:07] 576 
[01:15:07] 576 
[01:15:07] -     union TeeOutlivesAy<'a, T> {
[01:15:07] +     union TeeOutlivesAy<'a, T: 'a> {
[01:15:07] 578         //~^ ERROR outlives requirements can be inferred
[01:15:07] 579         tee: &'a T
[01:15:07] 
[01:15:07] 581 
[01:15:07] 581 
[01:15:07] -     union TeeOutlivesAyIsDebug<'a, T: Debug> {
[01:15:07] +     union TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
[01:15:07] 583         //~^ ERROR outlives requirements can be inferred
[01:15:07] 584         tee: &'a T
[01:15:07] 
[01:15:07] 586 
[01:15:07] 586 
[01:15:07] -     union TeeIsDebugOutlivesAy<'a, T: Debug> {
[01:15:07] +     union TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
[01:15:07] 588         //~^ ERROR outlives requirements can be inferred
[01:15:07] 589         tee: &'a T
[01:15:07] 
[01:15:07] 591 
[01:15:07] 591 
[01:15:07] -     union TeeOutlivesAyBee<'a, 'b, T> {
[01:15:07] +     union TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
[01:15:07] 593         //~^ ERROR outlives requirements can be inferred
[01:15:07] 594         tee: &'a &'b T
[01:15:07] 
[01:15:07] 596 
[01:15:07] 596 
[01:15:07] -     union TeeOutlivesAyBeeIsDebug<'a, 'b, T: Debug> {
[01:15:07] +     union TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
[01:15:07] 598         //~^ ERROR outlives requirements can be inferred
[01:15:07] 599         tee: &'a &'b T
[01:15:07] 
[01:15:07] 601 
[01:15:07] 601 
[01:15:07] -     union TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug> {
[01:15:07] +     union TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
[01:15:07] 603         //~^ ERROR outlives requirements can be inferred
[01:15:07] 604         tee: &'a &'b T
[01:15:07] 
[01:15:07] 
[01:15:07] 634         tee: &'a &'b T
[01:15:07] 636 
[01:15:07] 636 
[01:15:07] -     union TeeYooOutlivesAy<'a, T, U> {
[01:15:07] +     union TeeYooOutlivesAy<'a, T, U: 'a> {
[01:15:07] 638         //~^ ERROR outlives requirements can be inferred
[01:15:07] 639         tee: *const T,
[01:15:07] 640         yoo: &'a U
[01:15:07] 641     }
[01:15:07] 642 
[01:15:07] 642 
[01:15:07] -     union TeeYooOutlivesAyIsDebug<'a, T, U: Debug> {
[01:15:07] +     union TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
[01:15:07] 644         //~^ ERROR outlives requirements can be inferred
[01:15:07] 645         tee: *const T,
[01:15:07] 646         yoo: &'a U
[01:15:07] 647     }
[01:15:07] 648 
[01:15:07] 648 
[01:15:07] -     union TeeYooIsDebugOutlivesAy<'a, T, U: Debug> {
[01:15:07] +     union TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
[01:15:07] 650         //~^ ERROR outlives requirements can be inferred
[01:15:07] 651         tee: *const T,
[01:15:07] 652         yoo: &'a U
[01:15:07] 653     }
[01:15:07] 654 
[01:15:07] 654 
[01:15:07] -     union TeeOutlivesAyYooIsDebug<'a, T, U: Debug> {
[01:15:07] +     union TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
[01:15:07] 656         //~^ ERROR outlives requirements can be inferred
[01:15:07] 657         tee: &'a T,
[01:15:07] 658         yoo: *const U
[01:15:07] 659     }
[01:15:07] 660 
---
[01:15:07] test result: FAILED. 5562 passed; 2 failed; 21 ignored; 0 measured; 0 filtered out
[01:15:07] 
[01:15:07] 
[01:15:07] 
[01:15:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:07] 
[01:15:07] 
[01:15:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:07] Build completed unsuccessfully in 0:04:56
---
travis_time:end:02de5340:start=1558782782370462998,finish=1558782782376138980,duration=5675982
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10087b21
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18236751
travis_time:start:18236751
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03086670
$ dmesg | grep -i kill
