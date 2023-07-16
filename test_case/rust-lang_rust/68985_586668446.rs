plain
2020-02-16T03:43:41.1207255Z ========================== Starting Command Output ===========================
2020-02-16T03:43:41.1209072Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a50cf1b6-01b1-47ea-949a-66e8b7d743a2.sh
2020-02-16T03:43:41.1209155Z 
2020-02-16T03:43:41.1211685Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T03:43:41.1217168Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T03:43:41.1218725Z Task         : Get sources
2020-02-16T03:43:41.1218766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T03:43:41.1218802Z Version      : 1.0.0
2020-02-16T03:43:41.1218839Z Author       : Microsoft
---
2020-02-16T03:43:41.8844786Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T03:43:41.8899303Z ##[command]git config gc.auto 0
2020-02-16T03:43:41.8941853Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T03:43:41.8989805Z ##[command]git config --get-all http.proxy
2020-02-16T03:43:41.9256892Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68985/merge:refs/remotes/pull/68985/merge
---
2020-02-16T04:34:07.6673465Z .................................................................................................... 1700/9651
2020-02-16T04:34:11.8677352Z .................................................................................................... 1800/9651
2020-02-16T04:34:21.9022409Z ..................................i................................................................. 1900/9651
2020-02-16T04:34:28.3838321Z .................................................................................................... 2000/9651
2020-02-16T04:34:40.5191493Z ........................iiiii....................................................................... 2100/9651
2020-02-16T04:34:49.0782791Z .................................................................................................... 2300/9651
2020-02-16T04:34:51.1685333Z .................................................................................................... 2400/9651
2020-02-16T04:34:55.1152826Z .................................................................................................... 2500/9651
2020-02-16T04:35:12.8195360Z .................................................................................................... 2600/9651
---
2020-02-16T04:38:00.7017348Z .................................................................................................... 5600/9651
2020-02-16T04:38:09.3606582Z .......................................................................................i............ 5700/9651
2020-02-16T04:38:16.1562348Z .................................................................................................... 5800/9651
2020-02-16T04:38:20.7003691Z .....................................................................................i.............. 5900/9651
2020-02-16T04:38:28.8226577Z ...............................................................................ii...i..ii........... 6000/9651
2020-02-16T04:38:39.5239956Z i................................................................................................... 6100/9651
2020-02-16T04:38:52.4404449Z .................................................................................................... 6300/9651
2020-02-16T04:38:55.7399748Z .................................................................................................... 6400/9651
2020-02-16T04:38:55.7399748Z .................................................................................................... 6400/9651
2020-02-16T04:39:06.9961741Z .......i..ii........................................................................................ 6500/9651
2020-02-16T04:39:23.1183979Z ...............................................................................................i.... 6700/9651
2020-02-16T04:39:25.0029660Z ..................................................................................................F. 6800/9651
2020-02-16T04:39:26.8492759Z .................................................................................................... 6900/9651
2020-02-16T04:39:28.8479675Z ......i............................................................................................. 7000/9651
---
2020-02-16T04:40:47.0708925Z .................................................................................................... 7600/9651
2020-02-16T04:40:51.0235803Z .................................................................................................... 7700/9651
2020-02-16T04:40:56.0336130Z .................................................................................................... 7800/9651
2020-02-16T04:41:01.7220306Z .................................................................................................... 7900/9651
2020-02-16T04:41:09.8026753Z ........................................................................................iiiiiii.i... 8000/9651
2020-02-16T04:41:23.1178314Z ............................i......i................................................................ 8200/9651
2020-02-16T04:41:27.3227311Z .................................................................................................... 8300/9651
2020-02-16T04:41:36.7076855Z .................................................................................................... 8400/9651
2020-02-16T04:41:46.4568590Z .................................................................................................... 8500/9651
---
2020-02-16T04:43:22.7355485Z diff of stderr:
2020-02-16T04:43:22.7355604Z 
2020-02-16T04:43:22.7355738Z 91    |              help: maybe write a path separator here: `::`
2020-02-16T04:43:22.7355883Z 92    |
2020-02-16T04:43:22.7356019Z 93    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T04:43:22.7356667Z -    = note: for more information, see ***/issues/23416
2020-02-16T04:43:22.7357106Z +    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T04:43:22.7357394Z 96 error: casts cannot be followed by a function call
2020-02-16T04:43:22.7357712Z 97   --> $DIR/issue-35813-postfix-after-cast.rs:115:5
2020-02-16T04:43:22.7357951Z 
2020-02-16T04:43:22.7358102Z 120    |                     help: maybe write a path separator here: `::`
2020-02-16T04:43:22.7358102Z 120    |                     help: maybe write a path separator here: `::`
2020-02-16T04:43:22.7358227Z 121    |
2020-02-16T04:43:22.7358375Z 122    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T04:43:22.7358755Z -    = note: for more information, see ***/issues/23416
2020-02-16T04:43:22.7359158Z +    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T04:43:22.7359427Z 125 error: casts cannot be followed by a field access
2020-02-16T04:43:22.7359758Z 126   --> $DIR/issue-35813-postfix-after-cast.rs:137:5
2020-02-16T04:43:22.7359889Z 
2020-02-16T04:43:22.7359993Z 
2020-02-16T04:43:22.7359993Z 
2020-02-16T04:43:22.7360132Z The actual stderr differed from the expected stderr.
2020-02-16T04:43:22.7360540Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast/issue-35813-postfix-after-cast.stderr
2020-02-16T04:43:22.7360891Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T04:43:22.7361271Z To only update this specific test, also pass `--test-args parser/issue-35813-postfix-after-cast.rs`
2020-02-16T04:43:22.7361532Z error: 1 errors occurred comparing output.
2020-02-16T04:43:22.7361657Z status: exit code: 1
2020-02-16T04:43:22.7361657Z status: exit code: 1
2020-02-16T04:43:22.7362564Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast/auxiliary" "-A" "unused"
2020-02-16T04:43:22.7363078Z ------------------------------------------
2020-02-16T04:43:22.7363199Z 
2020-02-16T04:43:22.7363494Z ------------------------------------------
2020-02-16T04:43:22.7363735Z stderr:
2020-02-16T04:43:22.7363735Z stderr:
2020-02-16T04:43:22.7364000Z ------------------------------------------
2020-02-16T04:43:22.7364122Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7364416Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:10:5
2020-02-16T04:43:22.7364539Z    |
2020-02-16T04:43:22.7364650Z LL |     vec![1, 2, 3] as Vec<i32>[0];
2020-02-16T04:43:22.7364788Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(vec![1, 2, 3] as Vec<i32>)`
2020-02-16T04:43:22.7364996Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7365294Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:12:5
2020-02-16T04:43:22.7365412Z    |
2020-02-16T04:43:22.7365412Z    |
2020-02-16T04:43:22.7365526Z LL |     vec![1, 2, 3]: Vec<i32>[0];
2020-02-16T04:43:22.7365658Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(vec![1, 2, 3]: Vec<i32>)`
2020-02-16T04:43:22.7366039Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7366381Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:17:5
2020-02-16T04:43:22.7366518Z    |
2020-02-16T04:43:22.7366518Z    |
2020-02-16T04:43:22.7366653Z LL |     (&[0]) as &[i32][0];
2020-02-16T04:43:22.7366784Z    |     ^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `((&[0]) as &[i32])`
2020-02-16T04:43:22.7367095Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7367689Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:19:5
2020-02-16T04:43:22.7367836Z    |
2020-02-16T04:43:22.7367836Z    |
2020-02-16T04:43:22.7367982Z LL |     (&[0i32]): &[i32; 1][0];
2020-02-16T04:43:22.7368128Z    |     ^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `((&[0i32]): &[i32; 1])`
2020-02-16T04:43:22.7368465Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7368824Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:40:18
2020-02-16T04:43:22.7369273Z    |
2020-02-16T04:43:22.7369273Z    |
2020-02-16T04:43:22.7369426Z LL |     let x: i32 = &vec![1, 2, 3] as &Vec<i32>[0];
2020-02-16T04:43:22.7369577Z    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&vec![1, 2, 3] as &Vec<i32>)`
2020-02-16T04:43:22.7369852Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7370235Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:45:5
2020-02-16T04:43:22.7370383Z    |
2020-02-16T04:43:22.7370529Z LL |     0 as i32.max(0);
2020-02-16T04:43:22.7370529Z LL |     0 as i32.max(0);
2020-02-16T04:43:22.7370670Z    |     ^^^^^^^^ help: try surrounding the expression in parentheses: `(0 as i32)`
2020-02-16T04:43:22.7370933Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7371294Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:47:5
2020-02-16T04:43:22.7371438Z    |
2020-02-16T04:43:22.7373192Z LL |     0: i32.max(0);
2020-02-16T04:43:22.7373192Z LL |     0: i32.max(0);
2020-02-16T04:43:22.7373378Z    |     ^^^^^^ help: try surrounding the expression in parentheses: `(0: i32)`
2020-02-16T04:43:22.7373843Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7374290Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:62:8
2020-02-16T04:43:22.7375418Z    |
2020-02-16T04:43:22.7375418Z    |
2020-02-16T04:43:22.7375469Z LL |     if 5u64 as i32.max(0) == 0 {
2020-02-16T04:43:22.7375557Z    |        ^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64 as i32)`
2020-02-16T04:43:22.7375642Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7375937Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:65:8
2020-02-16T04:43:22.7376002Z    |
2020-02-16T04:43:22.7376002Z    |
2020-02-16T04:43:22.7376058Z LL |     if 5u64: u64.max(0) == 0 {
2020-02-16T04:43:22.7376115Z    |        ^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64: u64)`
2020-02-16T04:43:22.7376211Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7376470Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:72:9
2020-02-16T04:43:22.7376531Z    |
2020-02-16T04:43:22.7376531Z    |
2020-02-16T04:43:22.7376577Z LL |         5u64 as u32.max(0) == 0
2020-02-16T04:43:22.7376633Z    |         ^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64 as u32)`
2020-02-16T04:43:22.7376739Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7376996Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:76:9
2020-02-16T04:43:22.7377044Z    |
2020-02-16T04:43:22.7377044Z    |
2020-02-16T04:43:22.7377104Z LL |         5u64: u64.max(0) == 0
2020-02-16T04:43:22.7377161Z    |         ^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64: u64)`
2020-02-16T04:43:22.7377253Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7377523Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:81:24
2020-02-16T04:43:22.7377573Z    |
2020-02-16T04:43:22.7377573Z    |
2020-02-16T04:43:22.7377622Z LL | static bar: &[i32] = &(&[1,2,3] as &[i32][0..1]);
2020-02-16T04:43:22.7377697Z    |                        ^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&[1,2,3] as &[i32])`
2020-02-16T04:43:22.7377781Z error: casts cannot be followed by indexing
2020-02-16T04:43:22.7378170Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:84:25
2020-02-16T04:43:22.7378224Z    |
2020-02-16T04:43:22.7378224Z    |
2020-02-16T04:43:22.7378276Z LL | static bar2: &[i32] = &(&[1i32,2,3]: &[i32; 3][0..1]);
2020-02-16T04:43:22.7378339Z    |                         ^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&[1i32,2,3]: &[i32; 3])`
2020-02-16T04:43:22.7378503Z error: casts cannot be followed by ?
2020-02-16T04:43:22.7378769Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:89:5
2020-02-16T04:43:22.7378927Z    |
2020-02-16T04:43:22.7378927Z    |
2020-02-16T04:43:22.7378975Z LL |     Err(0u64) as Result<u64,u64>?;
2020-02-16T04:43:22.7379035Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Err(0u64) as Result<u64,u64>)`
2020-02-16T04:43:22.7379137Z error: casts cannot be followed by ?
2020-02-16T04:43:22.7379399Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:91:5
2020-02-16T04:43:22.7379449Z    |
2020-02-16T04:43:22.7379449Z    |
2020-02-16T04:43:22.7379521Z LL |     Err(0u64): Result<u64,u64>?;
2020-02-16T04:43:22.7379853Z    |     ^^^^^^^^^-^^^^^^^^^^^^^^^^
2020-02-16T04:43:22.7379960Z    |              help: maybe write a path separator here: `::`
2020-02-16T04:43:22.7380005Z    |
2020-02-16T04:43:22.7380005Z    |
2020-02-16T04:43:22.7380054Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T04:43:22.7380405Z    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T04:43:22.7380486Z error: casts cannot be followed by a function call
2020-02-16T04:43:22.7380833Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:115:5
2020-02-16T04:43:22.7380875Z    |
2020-02-16T04:43:22.7380875Z    |
2020-02-16T04:43:22.7380911Z LL |     drop as fn(u8)(0);
2020-02-16T04:43:22.7380956Z    |     ^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(drop as fn(u8))`
2020-02-16T04:43:22.7381044Z error: casts cannot be followed by a function call
2020-02-16T04:43:22.7381252Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:117:5
2020-02-16T04:43:22.7381306Z    |
2020-02-16T04:43:22.7381306Z    |
2020-02-16T04:43:22.7381344Z LL |     drop_ptr: fn(u8)(0);
2020-02-16T04:43:22.7381389Z    |     ^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(drop_ptr: fn(u8))`
2020-02-16T04:43:22.7381477Z error: casts cannot be followed by `.await`
2020-02-16T04:43:22.7381683Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:122:5
2020-02-16T04:43:22.7381723Z    |
2020-02-16T04:43:22.7381723Z    |
2020-02-16T04:43:22.7381777Z LL |     Box::pin(noop()) as Pin<Box<dyn Future<Output = ()>>>.await;
2020-02-16T04:43:22.7381835Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Box::pin(noop()) as Pin<Box<dyn Future<Output = ()>>>)`
2020-02-16T04:43:22.7381926Z error: casts cannot be followed by `.await`
2020-02-16T04:43:22.7382139Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:125:5
2020-02-16T04:43:22.7382180Z    |
2020-02-16T04:43:22.7382180Z    |
2020-02-16T04:43:22.7382232Z LL |     Box::pin(noop()): Pin<Box<_>>.await;
2020-02-16T04:43:22.7382408Z    |     ^^^^^^^^^^^^^^^^-^^^^^^^^^^^^
2020-02-16T04:43:22.7382506Z    |                     help: maybe write a path separator here: `::`
2020-02-16T04:43:22.7382552Z    |
2020-02-16T04:43:22.7382552Z    |
2020-02-16T04:43:22.7382596Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T04:43:22.7382854Z    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T04:43:22.7382921Z error: casts cannot be followed by a field access
2020-02-16T04:43:22.7383131Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:137:5
2020-02-16T04:43:22.7383185Z    |
2020-02-16T04:43:22.7383222Z LL |     Foo::default() as Foo.bar;
2020-02-16T04:43:22.7383222Z LL |     Foo::default() as Foo.bar;
2020-02-16T04:43:22.7383327Z    |     ^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Foo::default() as Foo)`
2020-02-16T04:43:22.7383411Z error: casts cannot be followed by a field access
2020-02-16T04:43:22.7383624Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:139:5
2020-02-16T04:43:22.7383676Z    |
2020-02-16T04:43:22.7383676Z    |
2020-02-16T04:43:22.7383876Z LL |     Foo::default(): Foo.bar;
2020-02-16T04:43:22.7383924Z    |     ^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Foo::default(): Foo)`
2020-02-16T04:43:22.7384005Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7384218Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:54:9
2020-02-16T04:43:22.7384258Z    |
2020-02-16T04:43:22.7384258Z    |
2020-02-16T04:43:22.7384311Z LL |         if true { 33 } else { 44 } as i32.max(0),
2020-02-16T04:43:22.7384369Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(if true { 33 } else { 44 } as i32)`
2020-02-16T04:43:22.7384452Z error: casts cannot be followed by a method call
2020-02-16T04:43:22.7384853Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:56:9
2020-02-16T04:43:22.7384900Z    |
2020-02-16T04:43:22.7384900Z    |
2020-02-16T04:43:22.7384938Z LL |         if true { 33 } else { 44 }: i32.max(0)
2020-02-16T04:43:22.7385013Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(if true { 33 } else { 44 }: i32)`
2020-02-16T04:43:22.7385088Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2020-02-16T04:43:22.7385326Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:101:13
2020-02-16T04:43:22.7385367Z    |
2020-02-16T04:43:22.7385403Z LL |     drop as F();
2020-02-16T04:43:22.7385403Z LL |     drop as F();
2020-02-16T04:43:22.7385458Z    |             ^^^ only `Fn` traits may use parentheses
2020-02-16T04:43:22.7385531Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2020-02-16T04:43:22.7385742Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:103:15
2020-02-16T04:43:22.7385797Z    |
2020-02-16T04:43:22.7385797Z    |
2020-02-16T04:43:22.7385833Z LL |     drop_ptr: F();
2020-02-16T04:43:22.7385874Z    |               ^^^ only `Fn` traits may use parentheses
2020-02-16T04:43:22.7385958Z error: aborting due to 25 previous errors
2020-02-16T04:43:22.7385983Z 
2020-02-16T04:43:22.7386180Z For more information about this error, try `rustc --explain E0214`.
2020-02-16T04:43:22.7386209Z 
---
2020-02-16T04:43:22.7387217Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T04:43:22.7387269Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T04:43:22.7390449Z 
2020-02-16T04:43:22.7390509Z 
2020-02-16T04:43:22.7391927Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T04:43:22.7392181Z 
2020-02-16T04:43:22.7392206Z 
2020-02-16T04:43:22.7397842Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T04:43:22.7397917Z Build completed unsuccessfully in 0:53:40
2020-02-16T04:43:22.7397917Z Build completed unsuccessfully in 0:53:40
2020-02-16T04:43:22.7464275Z == clock drift check ==
2020-02-16T04:43:22.7480073Z   local time: Sun Feb 16 04:43:22 UTC 2020
2020-02-16T04:43:23.0385140Z   network time: Sun, 16 Feb 2020 04:43:23 GMT
2020-02-16T04:43:23.0397379Z == end clock drift check ==
2020-02-16T04:43:23.5583446Z 
2020-02-16T04:43:23.5679492Z ##[error]Bash exited with code '1'.
2020-02-16T04:43:23.5689699Z ##[section]Finishing: Run build
2020-02-16T04:43:23.5708809Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T04:43:23.5710403Z Task         : Get sources
2020-02-16T04:43:23.5710457Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T04:43:23.5710500Z Version      : 1.0.0
2020-02-16T04:43:23.5710535Z Author       : Microsoft
2020-02-16T04:43:23.5710535Z Author       : Microsoft
2020-02-16T04:43:23.5710591Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T04:43:23.5710635Z ==============================================================================
2020-02-16T04:43:23.9218961Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T04:43:23.9254918Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T04:43:23.9355282Z Cleaning up task key
2020-02-16T04:43:23.9355974Z Start cleaning up orphan processes.
2020-02-16T04:43:23.9617176Z Terminate orphan process: pid (4153) (python)
2020-02-16T04:43:23.9640025Z ##[section]Finishing: Finalize Job
