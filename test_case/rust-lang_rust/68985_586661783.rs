plain
2020-02-16T01:21:35.6294088Z ========================== Starting Command Output ===========================
2020-02-16T01:21:35.6295787Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/acea18c0-63fc-4325-b9d1-3be1c4277348.sh
2020-02-16T01:21:35.6295827Z 
2020-02-16T01:21:35.6301818Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T01:21:35.6309022Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T01:21:35.6311042Z Task         : Get sources
2020-02-16T01:21:35.6311080Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T01:21:35.6311117Z Version      : 1.0.0
2020-02-16T01:21:35.6311153Z Author       : Microsoft
---
2020-02-16T01:21:36.6960613Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T01:21:36.6969578Z ##[command]git config gc.auto 0
2020-02-16T01:21:36.6971644Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T01:21:36.6973344Z ##[command]git config --get-all http.proxy
2020-02-16T01:21:36.6978697Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68985/merge:refs/remotes/pull/68985/merge
---
2020-02-16T02:19:26.1544236Z .................................................................................................... 1700/9651
2020-02-16T02:19:31.1091911Z .................................................................................................... 1800/9651
2020-02-16T02:19:42.8468100Z ..................................i................................................................. 1900/9651
2020-02-16T02:19:50.3344483Z .................................................................................................... 2000/9651
2020-02-16T02:20:04.0365974Z ........................iiiii....................................................................... 2100/9651
2020-02-16T02:20:13.3964959Z .................................................................................................... 2300/9651
2020-02-16T02:20:15.7977568Z .................................................................................................... 2400/9651
2020-02-16T02:20:20.2837318Z .................................................................................................... 2500/9651
2020-02-16T02:20:40.1937497Z .................................................................................................... 2600/9651
---
2020-02-16T02:23:53.5026987Z .................................................................................................... 5600/9651
2020-02-16T02:24:03.4936108Z .......................................................................................i............ 5700/9651
2020-02-16T02:24:10.9050833Z .................................................................................................... 5800/9651
2020-02-16T02:24:15.9582460Z .....................................................................................i.............. 5900/9651
2020-02-16T02:24:25.2330405Z ...............................................................................ii...i...ii.......... 6000/9651
2020-02-16T02:24:37.3048045Z i................................................................................................... 6100/9651
2020-02-16T02:24:53.4284808Z .................................................................................................... 6300/9651
2020-02-16T02:24:59.6712688Z .................................................................................................... 6400/9651
2020-02-16T02:24:59.6712688Z .................................................................................................... 6400/9651
2020-02-16T02:25:12.0321815Z .......i..ii........................................................................................ 6500/9651
2020-02-16T02:25:30.8132572Z ...............................................................................................i.... 6700/9651
2020-02-16T02:25:33.0708122Z .................................................................................................... 6800/9651
2020-02-16T02:25:35.3550461Z F................................................................................................... 6900/9651
2020-02-16T02:25:37.6807845Z ......i............................................................................................. 7000/9651
---
2020-02-16T02:27:09.5547039Z .................................................................................................... 7600/9651
2020-02-16T02:27:14.2825166Z .................................................................................................... 7700/9651
2020-02-16T02:27:20.1262375Z .................................................................................................... 7800/9651
2020-02-16T02:27:27.0455303Z .................................................................................................... 7900/9651
2020-02-16T02:27:35.7805438Z ........................................................................................iiiiiii.i... 8000/9651
2020-02-16T02:27:51.1497265Z ............................i......i................................................................ 8200/9651
2020-02-16T02:27:56.1991417Z .................................................................................................... 8300/9651
2020-02-16T02:28:07.2167214Z .................................................................................................... 8400/9651
2020-02-16T02:28:18.3299240Z .................................................................................................... 8500/9651
---
2020-02-16T02:30:12.9769102Z diff of stderr:
2020-02-16T02:30:12.9769228Z 
2020-02-16T02:30:12.9769548Z 91    |              help: maybe write a path separator here: `::`
2020-02-16T02:30:12.9769729Z 92    |
2020-02-16T02:30:12.9769890Z 93    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T02:30:12.9770603Z -    = note: for more information, see ***/issues/23416
2020-02-16T02:30:12.9771103Z +    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T02:30:12.9771407Z 96 error: casts cannot be followed by a function call
2020-02-16T02:30:12.9771797Z 97   --> $DIR/issue-35813-postfix-after-cast.rs:115:5
2020-02-16T02:30:12.9771943Z 
2020-02-16T02:30:12.9772070Z 120    |                     help: maybe write a path separator here: `::`
2020-02-16T02:30:12.9772070Z 120    |                     help: maybe write a path separator here: `::`
2020-02-16T02:30:12.9772217Z 121    |
2020-02-16T02:30:12.9772345Z 122    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T02:30:12.9772748Z -    = note: for more information, see ***/issues/23416
2020-02-16T02:30:12.9773200Z +    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T02:30:12.9773520Z 125 error: casts cannot be followed by a field access
2020-02-16T02:30:12.9773868Z 126   --> $DIR/issue-35813-postfix-after-cast.rs:137:5
2020-02-16T02:30:12.9774035Z 
2020-02-16T02:30:12.9774140Z 
2020-02-16T02:30:12.9774140Z 
2020-02-16T02:30:12.9774264Z The actual stderr differed from the expected stderr.
2020-02-16T02:30:12.9774699Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast/issue-35813-postfix-after-cast.stderr
2020-02-16T02:30:12.9775081Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T02:30:12.9775528Z To only update this specific test, also pass `--test-args parser/issue-35813-postfix-after-cast.rs`
2020-02-16T02:30:12.9775838Z error: 1 errors occurred comparing output.
2020-02-16T02:30:12.9775967Z status: exit code: 1
2020-02-16T02:30:12.9775967Z status: exit code: 1
2020-02-16T02:30:12.9776911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-35813-postfix-after-cast/auxiliary" "-A" "unused"
2020-02-16T02:30:12.9777495Z ------------------------------------------
2020-02-16T02:30:12.9777645Z 
2020-02-16T02:30:12.9778004Z ------------------------------------------
2020-02-16T02:30:12.9778164Z stderr:
2020-02-16T02:30:12.9778164Z stderr:
2020-02-16T02:30:12.9778492Z ------------------------------------------
2020-02-16T02:30:12.9778680Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9779063Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:10:5
2020-02-16T02:30:12.9779241Z    |
2020-02-16T02:30:12.9779396Z LL |     vec![1, 2, 3] as Vec<i32>[0];
2020-02-16T02:30:12.9779538Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(vec![1, 2, 3] as Vec<i32>)`
2020-02-16T02:30:12.9779805Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9780181Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:12:5
2020-02-16T02:30:12.9780371Z    |
2020-02-16T02:30:12.9780371Z    |
2020-02-16T02:30:12.9780497Z LL |     vec![1, 2, 3]: Vec<i32>[0];
2020-02-16T02:30:12.9780657Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(vec![1, 2, 3]: Vec<i32>)`
2020-02-16T02:30:12.9780923Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9781282Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:17:5
2020-02-16T02:30:12.9781466Z    |
2020-02-16T02:30:12.9781466Z    |
2020-02-16T02:30:12.9781704Z LL |     (&[0]) as &[i32][0];
2020-02-16T02:30:12.9781880Z    |     ^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `((&[0]) as &[i32])`
2020-02-16T02:30:12.9782250Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9782653Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:19:5
2020-02-16T02:30:12.9782844Z    |
2020-02-16T02:30:12.9782844Z    |
2020-02-16T02:30:12.9782970Z LL |     (&[0i32]): &[i32; 1][0];
2020-02-16T02:30:12.9783124Z    |     ^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `((&[0i32]): &[i32; 1])`
2020-02-16T02:30:12.9783368Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9783738Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:40:18
2020-02-16T02:30:12.9783920Z    |
2020-02-16T02:30:12.9783920Z    |
2020-02-16T02:30:12.9784050Z LL |     let x: i32 = &vec![1, 2, 3] as &Vec<i32>[0];
2020-02-16T02:30:12.9784219Z    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&vec![1, 2, 3] as &Vec<i32>)`
2020-02-16T02:30:12.9784479Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9784874Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:45:5
2020-02-16T02:30:12.9785036Z    |
2020-02-16T02:30:12.9785160Z LL |     0 as i32.max(0);
2020-02-16T02:30:12.9785160Z LL |     0 as i32.max(0);
2020-02-16T02:30:12.9785310Z    |     ^^^^^^^^ help: try surrounding the expression in parentheses: `(0 as i32)`
2020-02-16T02:30:12.9785558Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9785944Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:47:5
2020-02-16T02:30:12.9786108Z    |
2020-02-16T02:30:12.9786323Z LL |     0: i32.max(0);
2020-02-16T02:30:12.9786323Z LL |     0: i32.max(0);
2020-02-16T02:30:12.9786482Z    |     ^^^^^^ help: try surrounding the expression in parentheses: `(0: i32)`
2020-02-16T02:30:12.9786723Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9787136Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:62:8
2020-02-16T02:30:12.9787312Z    |
2020-02-16T02:30:12.9787312Z    |
2020-02-16T02:30:12.9787439Z LL |     if 5u64 as i32.max(0) == 0 {
2020-02-16T02:30:12.9787593Z    |        ^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64 as i32)`
2020-02-16T02:30:12.9787834Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9788220Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:65:8
2020-02-16T02:30:12.9788381Z    |
2020-02-16T02:30:12.9788381Z    |
2020-02-16T02:30:12.9788507Z LL |     if 5u64: u64.max(0) == 0 {
2020-02-16T02:30:12.9788662Z    |        ^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64: u64)`
2020-02-16T02:30:12.9788924Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9789312Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:72:9
2020-02-16T02:30:12.9789476Z    |
2020-02-16T02:30:12.9789476Z    |
2020-02-16T02:30:12.9789612Z LL |         5u64 as u32.max(0) == 0
2020-02-16T02:30:12.9789771Z    |         ^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64 as u32)`
2020-02-16T02:30:12.9790021Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9790409Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:76:9
2020-02-16T02:30:12.9790570Z    |
2020-02-16T02:30:12.9790570Z    |
2020-02-16T02:30:12.9790713Z LL |         5u64: u64.max(0) == 0
2020-02-16T02:30:12.9790848Z    |         ^^^^^^^^^ help: try surrounding the expression in parentheses: `(5u64: u64)`
2020-02-16T02:30:12.9793233Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9795353Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:81:24
2020-02-16T02:30:12.9795418Z    |
2020-02-16T02:30:12.9795418Z    |
2020-02-16T02:30:12.9795489Z LL | static bar: &[i32] = &(&[1,2,3] as &[i32][0..1]);
2020-02-16T02:30:12.9795679Z    |                        ^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&[1,2,3] as &[i32])`
2020-02-16T02:30:12.9795794Z error: casts cannot be followed by indexing
2020-02-16T02:30:12.9796178Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:84:25
2020-02-16T02:30:12.9796228Z    |
2020-02-16T02:30:12.9796228Z    |
2020-02-16T02:30:12.9796295Z LL | static bar2: &[i32] = &(&[1i32,2,3]: &[i32; 3][0..1]);
2020-02-16T02:30:12.9796356Z    |                         ^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(&[1i32,2,3]: &[i32; 3])`
2020-02-16T02:30:12.9796436Z error: casts cannot be followed by ?
2020-02-16T02:30:12.9796725Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:89:5
2020-02-16T02:30:12.9796773Z    |
2020-02-16T02:30:12.9796773Z    |
2020-02-16T02:30:12.9796817Z LL |     Err(0u64) as Result<u64,u64>?;
2020-02-16T02:30:12.9796891Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Err(0u64) as Result<u64,u64>)`
2020-02-16T02:30:12.9796982Z error: casts cannot be followed by ?
2020-02-16T02:30:12.9797246Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:91:5
2020-02-16T02:30:12.9797323Z    |
2020-02-16T02:30:12.9797323Z    |
2020-02-16T02:30:12.9797369Z LL |     Err(0u64): Result<u64,u64>?;
2020-02-16T02:30:12.9797591Z    |     ^^^^^^^^^-^^^^^^^^^^^^^^^^
2020-02-16T02:30:12.9797709Z    |              help: maybe write a path separator here: `::`
2020-02-16T02:30:12.9797756Z    |
2020-02-16T02:30:12.9797756Z    |
2020-02-16T02:30:12.9797826Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T02:30:12.9798175Z    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T02:30:12.9798280Z error: casts cannot be followed by a function call
2020-02-16T02:30:12.9798552Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:115:5
2020-02-16T02:30:12.9798601Z    |
2020-02-16T02:30:12.9798601Z    |
2020-02-16T02:30:12.9798670Z LL |     drop as fn(u8)(0);
2020-02-16T02:30:12.9798724Z    |     ^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(drop as fn(u8))`
2020-02-16T02:30:12.9798811Z error: casts cannot be followed by a function call
2020-02-16T02:30:12.9799095Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:117:5
2020-02-16T02:30:12.9799144Z    |
2020-02-16T02:30:12.9799144Z    |
2020-02-16T02:30:12.9799187Z LL |     drop_ptr: fn(u8)(0);
2020-02-16T02:30:12.9799259Z    |     ^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(drop_ptr: fn(u8))`
2020-02-16T02:30:12.9799339Z error: casts cannot be followed by `.await`
2020-02-16T02:30:12.9799599Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:122:5
2020-02-16T02:30:12.9799665Z    |
2020-02-16T02:30:12.9799665Z    |
2020-02-16T02:30:12.9799713Z LL |     Box::pin(noop()) as Pin<Box<dyn Future<Output = ()>>>.await;
2020-02-16T02:30:12.9799790Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Box::pin(noop()) as Pin<Box<dyn Future<Output = ()>>>)`
2020-02-16T02:30:12.9799913Z error: casts cannot be followed by `.await`
2020-02-16T02:30:12.9800183Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:125:5
2020-02-16T02:30:12.9800252Z    |
2020-02-16T02:30:12.9800252Z    |
2020-02-16T02:30:12.9800297Z LL |     Box::pin(noop()): Pin<Box<_>>.await;
2020-02-16T02:30:12.9800514Z    |     ^^^^^^^^^^^^^^^^-^^^^^^^^^^^^
2020-02-16T02:30:12.9800632Z    |                     help: maybe write a path separator here: `::`
2020-02-16T02:30:12.9800677Z    |
2020-02-16T02:30:12.9800677Z    |
2020-02-16T02:30:12.9800746Z    = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
2020-02-16T02:30:12.9801061Z    = note: see issue #23416 <***/issues/23416> for more information
2020-02-16T02:30:12.9801163Z error: casts cannot be followed by a field access
2020-02-16T02:30:12.9801523Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:137:5
2020-02-16T02:30:12.9801654Z    |
2020-02-16T02:30:12.9801698Z LL |     Foo::default() as Foo.bar;
2020-02-16T02:30:12.9801698Z LL |     Foo::default() as Foo.bar;
2020-02-16T02:30:12.9801774Z    |     ^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Foo::default() as Foo)`
2020-02-16T02:30:12.9801857Z error: casts cannot be followed by a field access
2020-02-16T02:30:12.9802174Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:139:5
2020-02-16T02:30:12.9802222Z    |
2020-02-16T02:30:12.9802222Z    |
2020-02-16T02:30:12.9802265Z LL |     Foo::default(): Foo.bar;
2020-02-16T02:30:12.9802338Z    |     ^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(Foo::default(): Foo)`
2020-02-16T02:30:12.9802418Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9802677Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:54:9
2020-02-16T02:30:12.9802742Z    |
2020-02-16T02:30:12.9802742Z    |
2020-02-16T02:30:12.9802797Z LL |         if true { 33 } else { 44 } as i32.max(0),
2020-02-16T02:30:12.9802857Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(if true { 33 } else { 44 } as i32)`
2020-02-16T02:30:12.9802965Z error: casts cannot be followed by a method call
2020-02-16T02:30:12.9803230Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:56:9
2020-02-16T02:30:12.9803296Z    |
2020-02-16T02:30:12.9803296Z    |
2020-02-16T02:30:12.9803342Z LL |         if true { 33 } else { 44 }: i32.max(0)
2020-02-16T02:30:12.9803402Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try surrounding the expression in parentheses: `(if true { 33 } else { 44 }: i32)`
2020-02-16T02:30:12.9803507Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2020-02-16T02:30:12.9803945Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:101:13
2020-02-16T02:30:12.9803998Z    |
2020-02-16T02:30:12.9804072Z LL |     drop as F();
2020-02-16T02:30:12.9804072Z LL |     drop as F();
2020-02-16T02:30:12.9804120Z    |             ^^^ only `Fn` traits may use parentheses
2020-02-16T02:30:12.9804207Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2020-02-16T02:30:12.9804520Z   --> /checkout/src/test/ui/parser/issue-35813-postfix-after-cast.rs:103:15
2020-02-16T02:30:12.9804569Z    |
2020-02-16T02:30:12.9804569Z    |
2020-02-16T02:30:12.9804611Z LL |     drop_ptr: F();
2020-02-16T02:30:12.9804678Z    |               ^^^ only `Fn` traits may use parentheses
2020-02-16T02:30:12.9804753Z error: aborting due to 25 previous errors
2020-02-16T02:30:12.9804782Z 
2020-02-16T02:30:12.9805051Z For more information about this error, try `rustc --explain E0214`.
2020-02-16T02:30:12.9805085Z 
---
2020-02-16T02:30:12.9881677Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T02:30:12.9882027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T02:30:12.9882068Z 
2020-02-16T02:30:12.9882098Z 
2020-02-16T02:30:12.9884481Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T02:30:12.9884873Z 
2020-02-16T02:30:12.9884908Z 
2020-02-16T02:30:12.9884971Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T02:30:12.9885029Z Build completed unsuccessfully in 1:01:42
2020-02-16T02:30:12.9885029Z Build completed unsuccessfully in 1:01:42
2020-02-16T02:30:12.9885099Z == clock drift check ==
2020-02-16T02:30:12.9897440Z   local time: Sun Feb 16 02:30:12 UTC 2020
2020-02-16T02:30:13.2874840Z   network time: Sun, 16 Feb 2020 02:30:13 GMT
2020-02-16T02:30:13.2876364Z == end clock drift check ==
2020-02-16T02:30:13.7594765Z 
2020-02-16T02:30:13.7679419Z ##[error]Bash exited with code '1'.
2020-02-16T02:30:13.7690193Z ##[section]Finishing: Run build
2020-02-16T02:30:13.7707828Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T02:30:13.7709517Z Task         : Get sources
2020-02-16T02:30:13.7709560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T02:30:13.7709618Z Version      : 1.0.0
2020-02-16T02:30:13.7709654Z Author       : Microsoft
2020-02-16T02:30:13.7709654Z Author       : Microsoft
2020-02-16T02:30:13.7709698Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T02:30:13.7709757Z ==============================================================================
2020-02-16T02:30:14.1719973Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T02:30:14.1766771Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T02:30:14.1865857Z Cleaning up task key
2020-02-16T02:30:14.1866598Z Start cleaning up orphan processes.
2020-02-16T02:30:14.1963256Z Terminate orphan process: pid (4205) (python)
2020-02-16T02:30:14.2183247Z ##[section]Finishing: Finalize Job
