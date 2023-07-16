plain
2020-01-09T06:54:59.2977731Z 
2020-01-09T06:54:59.2977800Z 1 error: cannot find derive macro `Eqr` in this scope
2020-01-09T06:54:59.2981114Z 2   --> $DIR/deriving-meta-unknown-trait.rs:4:10
2020-01-09T06:54:59.2981393Z 3    |
2020-01-09T06:54:59.2981771Z - LL | #[derive(Eqr)]
2020-01-09T06:54:59.2982073Z -    |          ^^^ help: a derive macro with a similar name exists: `Eq`
2020-01-09T06:54:59.2982304Z -    | 
2020-01-09T06:54:59.2982516Z -   ::: $SRC_DIR/libcore/cmp.rs:LL:COL
2020-01-09T06:54:59.2982727Z -    |
2020-01-09T06:54:59.2982934Z - LL | pub macro Eq($item:item) {
2020-01-09T06:54:59.2983220Z -    | ------------------------ similarly named derive macro `Eq` defined here
2020-01-09T06:54:59.2983317Z + LL |   #[derive(Eqr)]
2020-01-09T06:54:59.2983391Z +    |            ^^^ help: a derive macro with a similar name exists: `Eq`
2020-01-09T06:54:59.2983541Z 12 error: aborting due to previous error
2020-01-09T06:54:59.2983620Z 13 
2020-01-09T06:54:59.2983653Z 
2020-01-09T06:54:59.2983689Z 
2020-01-09T06:54:59.2983689Z 
2020-01-09T06:54:59.2983770Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.2984591Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/deriving-meta-unknown-trait.stderr
2020-01-09T06:54:59.2984951Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.2985293Z To only update this specific test, also pass `--test-args derives/deriving-meta-unknown-trait.rs`
2020-01-09T06:54:59.2985443Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.2985516Z status: exit code: 1
2020-01-09T06:54:59.2985516Z status: exit code: 1
2020-01-09T06:54:59.2986663Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/auxiliary" "-A" "unused"
2020-01-09T06:54:59.2987236Z ------------------------------------------
2020-01-09T06:54:59.2987304Z 
2020-01-09T06:54:59.2987696Z ------------------------------------------
2020-01-09T06:54:59.2987775Z stderr:
2020-01-09T06:54:59.2987775Z stderr:
2020-01-09T06:54:59.2987975Z ------------------------------------------
2020-01-09T06:54:59.2988062Z error: cannot find derive macro `Eqr` in this scope
2020-01-09T06:54:59.2988309Z   --> /checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs:4:10
2020-01-09T06:54:59.2988397Z    |
2020-01-09T06:54:59.2988469Z LL |   #[derive(Eqr)]
2020-01-09T06:54:59.2988539Z    |            ^^^ help: a derive macro with a similar name exists: `Eq`
2020-01-09T06:54:59.2988660Z error: aborting due to previous error
2020-01-09T06:54:59.2988706Z 
2020-01-09T06:54:59.2988752Z 
2020-01-09T06:54:59.2988965Z ------------------------------------------
---
2020-01-09T06:54:59.2989383Z 
2020-01-09T06:54:59.2989460Z 1 error[E0573]: expected type, found variant `NoResult`
2020-01-09T06:54:59.2989671Z 2   --> $DIR/issue-17546.rs:15:17
2020-01-09T06:54:59.2989750Z 3    |
2020-01-09T06:54:59.2989961Z - LL |     fn new() -> NoResult<MyEnum, String> {
2020-01-09T06:54:59.2990551Z -    | 
2020-01-09T06:54:59.2990551Z -    | 
2020-01-09T06:54:59.2990773Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2020-01-09T06:54:59.2991001Z + LL |       fn new() -> NoResult<MyEnum, String> {
2020-01-09T06:54:59.2991154Z 8    |
2020-01-09T06:54:59.2991368Z - LL | pub enum Result<T, E> {
2020-01-09T06:54:59.2991819Z -    | --------------------- similarly named enum `Result` defined here
2020-01-09T06:54:59.2992063Z -    |
2020-01-09T06:54:59.2992063Z -    |
2020-01-09T06:54:59.2992286Z 12 help: try using the variant's enum
2020-01-09T06:54:59.2992351Z 13    |
2020-01-09T06:54:59.2992571Z 14 LL |     fn new() -> foo::MyEnum {
2020-01-09T06:54:59.2992676Z 57 error[E0573]: expected type, found variant `NoResult`
2020-01-09T06:54:59.2992912Z 58   --> $DIR/issue-17546.rs:36:15
2020-01-09T06:54:59.2992992Z 59    |
2020-01-09T06:54:59.2992992Z 59    |
2020-01-09T06:54:59.2993211Z - LL | fn newer() -> NoResult<foo::MyEnum, String> {
2020-01-09T06:54:59.2993645Z -    | 
2020-01-09T06:54:59.2993645Z -    | 
2020-01-09T06:54:59.2993865Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2020-01-09T06:54:59.2995027Z - LL | pub enum Result<T, E> {
2020-01-09T06:54:59.2995304Z -    | --------------------- similarly named enum `Result` defined here
2020-01-09T06:54:59.2995304Z -    | --------------------- similarly named enum `Result` defined here
2020-01-09T06:54:59.2995607Z + LL |   fn newer() -> NoResult<foo::MyEnum, String> {
2020-01-09T06:54:59.2999490Z 67    |
2020-01-09T06:54:59.2999826Z 68 help: try using the variant's enum
2020-01-09T06:54:59.3000015Z 69    |
2020-01-09T06:54:59.3000049Z 
2020-01-09T06:54:59.3000049Z 
2020-01-09T06:54:59.3000097Z 
2020-01-09T06:54:59.3000157Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3000500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
2020-01-09T06:54:59.3000761Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3001046Z To only update this specific test, also pass `--test-args issues/issue-17546.rs`
2020-01-09T06:54:59.3001382Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3001447Z status: exit code: 1
2020-01-09T06:54:59.3001447Z status: exit code: 1
2020-01-09T06:54:59.3003207Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17546.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3004296Z ------------------------------------------
2020-01-09T06:54:59.3004379Z 
2020-01-09T06:54:59.3004657Z ------------------------------------------
2020-01-09T06:54:59.3004749Z stderr:
2020-01-09T06:54:59.3004749Z stderr:
2020-01-09T06:54:59.3004977Z ------------------------------------------
2020-01-09T06:54:59.3005076Z error[E0573]: expected type, found variant `NoResult`
2020-01-09T06:54:59.3005381Z   --> /checkout/src/test/ui/issues/issue-17546.rs:15:17
2020-01-09T06:54:59.3005470Z    |
2020-01-09T06:54:59.3005734Z LL |       fn new() -> NoResult<MyEnum, String> {
2020-01-09T06:54:59.3005903Z    |
2020-01-09T06:54:59.3006128Z help: try using the variant's enum
2020-01-09T06:54:59.3006217Z    |
2020-01-09T06:54:59.3006217Z    |
2020-01-09T06:54:59.3006440Z LL |     fn new() -> foo::MyEnum {
2020-01-09T06:54:59.3006602Z help: an enum with a similar name exists
2020-01-09T06:54:59.3006687Z    |
2020-01-09T06:54:59.3006687Z    |
2020-01-09T06:54:59.3006922Z LL |     fn new() -> Result<MyEnum, String> {
2020-01-09T06:54:59.3007058Z 
2020-01-09T06:54:59.3007145Z error[E0573]: expected type, found variant `Result`
2020-01-09T06:54:59.3007411Z   --> /checkout/src/test/ui/issues/issue-17546.rs:25:17
2020-01-09T06:54:59.3007658Z    |
2020-01-09T06:54:59.3007658Z    |
2020-01-09T06:54:59.3007870Z LL |     fn new() -> Result<foo::MyEnum, String> {
2020-01-09T06:54:59.3008240Z    |
2020-01-09T06:54:59.3008309Z help: possible better candidates are found in other modules, you can import them into scope
2020-01-09T06:54:59.3008401Z    |
2020-01-09T06:54:59.3008457Z LL |     use std::fmt::Result;
---
2020-01-09T06:54:59.3009023Z 
2020-01-09T06:54:59.3009081Z error[E0573]: expected type, found variant `Result`
2020-01-09T06:54:59.3009377Z   --> /checkout/src/test/ui/issues/issue-17546.rs:31:13
2020-01-09T06:54:59.3009442Z    |
2020-01-09T06:54:59.3009667Z LL | fn new() -> Result<foo::MyEnum, String> {
2020-01-09T06:54:59.3009836Z    |
2020-01-09T06:54:59.3010187Z help: possible better candidates are found in other modules, you can import them into scope
2020-01-09T06:54:59.3010268Z    |
2020-01-09T06:54:59.3010338Z LL | use std::fmt::Result;
---
2020-01-09T06:54:59.3010882Z 
2020-01-09T06:54:59.3010957Z error[E0573]: expected type, found variant `NoResult`
2020-01-09T06:54:59.3011234Z   --> /checkout/src/test/ui/issues/issue-17546.rs:36:15
2020-01-09T06:54:59.3011317Z    |
2020-01-09T06:54:59.3011531Z LL |   fn newer() -> NoResult<foo::MyEnum, String> {
2020-01-09T06:54:59.3011698Z    |
2020-01-09T06:54:59.3011909Z help: try using the variant's enum
2020-01-09T06:54:59.3011969Z    |
2020-01-09T06:54:59.3011969Z    |
2020-01-09T06:54:59.3012175Z LL | fn newer() -> foo::MyEnum {
2020-01-09T06:54:59.3012315Z help: an enum with a similar name exists
2020-01-09T06:54:59.3012391Z    |
2020-01-09T06:54:59.3012391Z    |
2020-01-09T06:54:59.3012599Z LL | fn newer() -> Result<foo::MyEnum, String> {
2020-01-09T06:54:59.3012717Z 
2020-01-09T06:54:59.3012772Z error: aborting due to 4 previous errors
2020-01-09T06:54:59.3012827Z 
2020-01-09T06:54:59.3013059Z For more information about this error, try `rustc --explain E0573`.
---
2020-01-09T06:54:59.3014536Z 3    |
2020-01-09T06:54:59.3014742Z - LL | impl Fo {
2020-01-09T06:54:59.3015024Z -    |      ^^ help: a trait with a similar name exists: `Fn`
2020-01-09T06:54:59.3015234Z -    | 
2020-01-09T06:54:59.3015491Z -   ::: $SRC_DIR/libcore/ops/function.rs:LL:COL
2020-01-09T06:54:59.3015696Z -    |
2020-01-09T06:54:59.3015944Z - LL | pub trait Fn<Args>: FnMut<Args> {
2020-01-09T06:54:59.3016245Z -    | ------------------------------- similarly named trait `Fn` defined here
2020-01-09T06:54:59.3016331Z + LL |   impl Fo {
2020-01-09T06:54:59.3016425Z +    |        ^^ help: a trait with a similar name exists: `Fn`
2020-01-09T06:54:59.3016582Z 12 error: aborting due to previous error
2020-01-09T06:54:59.3016781Z 13 
2020-01-09T06:54:59.3016834Z 
2020-01-09T06:54:59.3016869Z 
2020-01-09T06:54:59.3016869Z 
2020-01-09T06:54:59.3017015Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3017578Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/issue-7607-1.stderr
2020-01-09T06:54:59.3017839Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3018119Z To only update this specific test, also pass `--test-args issues/issue-7607-1.rs`
2020-01-09T06:54:59.3018245Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3018307Z status: exit code: 1
2020-01-09T06:54:59.3018307Z status: exit code: 1
2020-01-09T06:54:59.3019243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7607-1.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3019738Z ------------------------------------------
2020-01-09T06:54:59.3019783Z 
2020-01-09T06:54:59.3020002Z ------------------------------------------
2020-01-09T06:54:59.3020064Z stderr:
2020-01-09T06:54:59.3020064Z stderr:
2020-01-09T06:54:59.3020279Z ------------------------------------------
2020-01-09T06:54:59.3020348Z error[E0412]: cannot find type `Fo` in this scope
2020-01-09T06:54:59.3020593Z   --> /checkout/src/test/ui/issues/issue-7607-1.rs:8:6
2020-01-09T06:54:59.3020659Z    |
2020-01-09T06:54:59.3020737Z LL |   impl Fo { //~ ERROR cannot find type `Fo` in this scope
2020-01-09T06:54:59.3020814Z    |        ^^ help: a trait with a similar name exists: `Fn`
2020-01-09T06:54:59.3020944Z error: aborting due to previous error
2020-01-09T06:54:59.3021000Z 
2020-01-09T06:54:59.3021238Z For more information about this error, try `rustc --explain E0412`.
2020-01-09T06:54:59.3021287Z 
---
2020-01-09T06:54:59.3021923Z 
2020-01-09T06:54:59.3021980Z 1 error: cannot find macro `printlx` in this scope
2020-01-09T06:54:59.3022204Z 2   --> $DIR/macro-name-typo.rs:5:5
2020-01-09T06:54:59.3022267Z 3    |
2020-01-09T06:54:59.3022476Z - LL |     printlx!("oh noes!");
2020-01-09T06:54:59.3022937Z -    | 
2020-01-09T06:54:59.3022937Z -    | 
2020-01-09T06:54:59.3023168Z -   ::: $SRC_DIR/libstd/macros.rs:LL:COL
2020-01-09T06:54:59.3023370Z -    |
2020-01-09T06:54:59.3023589Z - LL | macro_rules! println {
2020-01-09T06:54:59.3023856Z -    | -------------------- similarly named macro `println` defined here
2020-01-09T06:54:59.3024632Z + LL |       printlx!("oh noes!");
2020-01-09T06:54:59.3024812Z 11 
2020-01-09T06:54:59.3024874Z 12 error: aborting due to previous error
2020-01-09T06:54:59.3024958Z 13 
2020-01-09T06:54:59.3024995Z 
2020-01-09T06:54:59.3024995Z 
2020-01-09T06:54:59.3025048Z 
2020-01-09T06:54:59.3025116Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3025522Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/macro-name-typo.stderr
2020-01-09T06:54:59.3025826Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3026147Z To only update this specific test, also pass `--test-args macros/macro-name-typo.rs`
2020-01-09T06:54:59.3026406Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3026532Z status: exit code: 1
2020-01-09T06:54:59.3026532Z status: exit code: 1
2020-01-09T06:54:59.3027643Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-name-typo.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3028328Z ------------------------------------------
2020-01-09T06:54:59.3028390Z 
2020-01-09T06:54:59.3028798Z ------------------------------------------
2020-01-09T06:54:59.3028884Z stderr:
2020-01-09T06:54:59.3028884Z stderr:
2020-01-09T06:54:59.3029086Z ------------------------------------------
2020-01-09T06:54:59.3029171Z error: cannot find macro `printlx` in this scope
2020-01-09T06:54:59.3029414Z   --> /checkout/src/test/ui/macros/macro-name-typo.rs:5:5
2020-01-09T06:54:59.3029479Z    |
2020-01-09T06:54:59.3029554Z LL |       printlx!("oh noes!"); //~ ERROR cannot find
2020-01-09T06:54:59.3029691Z 
2020-01-09T06:54:59.3029743Z error: aborting due to previous error
2020-01-09T06:54:59.3029780Z 
2020-01-09T06:54:59.3029827Z 
---
2020-01-09T06:54:59.3030824Z 3    |
2020-01-09T06:54:59.3031001Z - LL |     inline!();
2020-01-09T06:54:59.3031247Z -    |     ^^^^^^ help: a macro with a similar name exists: `line`
2020-01-09T06:54:59.3031424Z -    | 
2020-01-09T06:54:59.3031637Z -   ::: $SRC_DIR/libcore/macros/mod.rs:LL:COL
2020-01-09T06:54:59.3032010Z - LL |     macro_rules! line {
2020-01-09T06:54:59.3032257Z -    |     ----------------- similarly named macro `line` defined here
2020-01-09T06:54:59.3032326Z + LL |       inline!();
2020-01-09T06:54:59.3032410Z +    |       ^^^^^^ help: a macro with a similar name exists: `line`
2020-01-09T06:54:59.3032410Z +    |       ^^^^^^ help: a macro with a similar name exists: `line`
2020-01-09T06:54:59.3032474Z 11 
2020-01-09T06:54:59.3032544Z 12 error: aborting due to previous error
2020-01-09T06:54:59.3032602Z 13 
2020-01-09T06:54:59.3032650Z 
2020-01-09T06:54:59.3032679Z 
2020-01-09T06:54:59.3032737Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3033091Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/macro-path-prelude-fail-3.stderr
2020-01-09T06:54:59.3033371Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3033638Z To only update this specific test, also pass `--test-args macros/macro-path-prelude-fail-3.rs`
2020-01-09T06:54:59.3033764Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3033841Z status: exit code: 1
2020-01-09T06:54:59.3033841Z status: exit code: 1
2020-01-09T06:54:59.3035556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-path-prelude-fail-3.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3036237Z ------------------------------------------
2020-01-09T06:54:59.3036288Z 
2020-01-09T06:54:59.3036539Z ------------------------------------------
2020-01-09T06:54:59.3036611Z stderr:
---
2020-01-09T06:54:59.3038598Z ---- [ui] ui/proc-macro/parent-source-spans.rs stdout ----
2020-01-09T06:54:59.3038665Z diff of stderr:
2020-01-09T06:54:59.3038715Z 
2020-01-09T06:54:59.3038766Z 132 ...
2020-01-09T06:54:59.3038836Z 133 LL |     one!("hello", "world");
2020-01-09T06:54:59.3039271Z -    | 
2020-01-09T06:54:59.3039271Z -    | 
2020-01-09T06:54:59.3039637Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2020-01-09T06:54:59.3039823Z -    |
2020-01-09T06:54:59.3040042Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2020-01-09T06:54:59.3040339Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2020-01-09T06:54:59.3040494Z 141 error[E0425]: cannot find value `ok` in this scope
2020-01-09T06:54:59.3040740Z 142   --> $DIR/parent-source-spans.rs:31:5
2020-01-09T06:54:59.3040782Z 
2020-01-09T06:54:59.3040834Z 146 ...
2020-01-09T06:54:59.3040834Z 146 ...
2020-01-09T06:54:59.3040904Z 147 LL |     two!("yay", "rust");
2020-01-09T06:54:59.3041355Z -    | 
2020-01-09T06:54:59.3041355Z -    | 
2020-01-09T06:54:59.3041577Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2020-01-09T06:54:59.3041765Z -    |
2020-01-09T06:54:59.3042017Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2020-01-09T06:54:59.3042315Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2020-01-09T06:54:59.3042475Z 155 error[E0425]: cannot find value `ok` in this scope
2020-01-09T06:54:59.3042889Z 156   --> $DIR/parent-source-spans.rs:31:5
2020-01-09T06:54:59.3042929Z 
2020-01-09T06:54:59.3042995Z 160 ...
2020-01-09T06:54:59.3042995Z 160 ...
2020-01-09T06:54:59.3043045Z 161 LL |     three!("hip", "hop");
2020-01-09T06:54:59.3043476Z -    | 
2020-01-09T06:54:59.3043476Z -    | 
2020-01-09T06:54:59.3043848Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2020-01-09T06:54:59.3044416Z -    |
2020-01-09T06:54:59.3044702Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2020-01-09T06:54:59.3045032Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2020-01-09T06:54:59.3045225Z 169 error: aborting due to 21 previous errors
2020-01-09T06:54:59.3045296Z 170 
2020-01-09T06:54:59.3045332Z 
2020-01-09T06:54:59.3045386Z 
2020-01-09T06:54:59.3045386Z 
2020-01-09T06:54:59.3045455Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3045838Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/parent-source-spans.stderr
2020-01-09T06:54:59.3046137Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3046683Z To only update this specific test, also pass `--test-args proc-macro/parent-source-spans.rs`
2020-01-09T06:54:59.3046840Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3046912Z status: exit code: 1
2020-01-09T06:54:59.3046912Z status: exit code: 1
2020-01-09T06:54:59.3048184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/parent-source-spans.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3048732Z ------------------------------------------
2020-01-09T06:54:59.3048790Z 
2020-01-09T06:54:59.3049016Z ------------------------------------------
2020-01-09T06:54:59.3049101Z stderr:
2020-01-09T06:54:59.3049101Z stderr:
2020-01-09T06:54:59.3049322Z ------------------------------------------
2020-01-09T06:54:59.3049412Z error: first final: "hello"
2020-01-09T06:54:59.3049690Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:18:12
2020-01-09T06:54:59.3049770Z    |
2020-01-09T06:54:59.3049847Z LL |     three!($a, $b);
2020-01-09T06:54:59.3061007Z ...
2020-01-09T06:54:59.3061007Z ...
2020-01-09T06:54:59.3061070Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3061829Z 
2020-01-09T06:54:59.3062066Z error: second final: "world"
2020-01-09T06:54:59.3062339Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:18:16
2020-01-09T06:54:59.3062431Z    |
2020-01-09T06:54:59.3062431Z    |
2020-01-09T06:54:59.3062490Z LL |     three!($a, $b);
2020-01-09T06:54:59.3062655Z ...
2020-01-09T06:54:59.3062655Z ...
2020-01-09T06:54:59.3062728Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3063046Z 
2020-01-09T06:54:59.3063103Z error: first parent: "hello"
2020-01-09T06:54:59.3095595Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:12:5
2020-01-09T06:54:59.3095701Z    |
2020-01-09T06:54:59.3095701Z    |
2020-01-09T06:54:59.3095782Z LL |     two!($a, $b);
2020-01-09T06:54:59.3099619Z ...
2020-01-09T06:54:59.3099619Z ...
2020-01-09T06:54:59.3099710Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3100333Z 
2020-01-09T06:54:59.3100399Z error: second parent: "world"
2020-01-09T06:54:59.3100642Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:12:5
2020-01-09T06:54:59.3100717Z    |
2020-01-09T06:54:59.3100717Z    |
2020-01-09T06:54:59.3100770Z LL |     two!($a, $b);
2020-01-09T06:54:59.3100923Z ...
2020-01-09T06:54:59.3100923Z ...
2020-01-09T06:54:59.3100984Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3101278Z 
2020-01-09T06:54:59.3101278Z 
2020-01-09T06:54:59.3101332Z error: first grandparent: "hello"
2020-01-09T06:54:59.3101659Z    |
2020-01-09T06:54:59.3101659Z    |
2020-01-09T06:54:59.3101713Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3102042Z 
2020-01-09T06:54:59.3102042Z 
2020-01-09T06:54:59.3102097Z error: second grandparent: "world"
2020-01-09T06:54:59.3102538Z    |
2020-01-09T06:54:59.3102538Z    |
2020-01-09T06:54:59.3102598Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3104291Z 
2020-01-09T06:54:59.3104374Z error: first source: "hello"
2020-01-09T06:54:59.3105089Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:38:5
2020-01-09T06:54:59.3105180Z    |
2020-01-09T06:54:59.3105180Z    |
2020-01-09T06:54:59.3105250Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3105369Z 
2020-01-09T06:54:59.3105429Z error: second source: "world"
2020-01-09T06:54:59.3105743Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:38:5
2020-01-09T06:54:59.3105822Z    |
2020-01-09T06:54:59.3105822Z    |
2020-01-09T06:54:59.3106026Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3106147Z 
2020-01-09T06:54:59.3106147Z 
2020-01-09T06:54:59.3106206Z error: first final: "yay"
2020-01-09T06:54:59.3106593Z    |
2020-01-09T06:54:59.3106593Z    |
2020-01-09T06:54:59.3106664Z LL |     three!($a, $b);
2020-01-09T06:54:59.3106807Z ...
2020-01-09T06:54:59.3106807Z ...
2020-01-09T06:54:59.3106882Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3107214Z 
2020-01-09T06:54:59.3107288Z error: second final: "rust"
2020-01-09T06:54:59.3107717Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:18:16
2020-01-09T06:54:59.3107805Z    |
2020-01-09T06:54:59.3107805Z    |
2020-01-09T06:54:59.3108036Z LL |     three!($a, $b);
2020-01-09T06:54:59.3108160Z ...
2020-01-09T06:54:59.3108160Z ...
2020-01-09T06:54:59.3108215Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3108680Z 
2020-01-09T06:54:59.3108680Z 
2020-01-09T06:54:59.3108739Z error: first parent: "yay"
2020-01-09T06:54:59.3109050Z    |
2020-01-09T06:54:59.3109050Z    |
2020-01-09T06:54:59.3109102Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3109203Z 
2020-01-09T06:54:59.3109255Z error: second parent: "rust"
2020-01-09T06:54:59.3109494Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:44:5
2020-01-09T06:54:59.3109584Z    |
2020-01-09T06:54:59.3109584Z    |
2020-01-09T06:54:59.3109637Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3109738Z 
2020-01-09T06:54:59.3109789Z error: first source: "yay"
2020-01-09T06:54:59.3110031Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:44:5
2020-01-09T06:54:59.3110104Z    |
2020-01-09T06:54:59.3110104Z    |
2020-01-09T06:54:59.3110156Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3110257Z 
2020-01-09T06:54:59.3110309Z error: second source: "rust"
2020-01-09T06:54:59.3110550Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:44:5
2020-01-09T06:54:59.3110622Z    |
2020-01-09T06:54:59.3110622Z    |
2020-01-09T06:54:59.3110676Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3110781Z 
2020-01-09T06:54:59.3110781Z 
2020-01-09T06:54:59.3110833Z error: first final: "hip"
2020-01-09T06:54:59.3111160Z    |
2020-01-09T06:54:59.3111160Z    |
2020-01-09T06:54:59.3111228Z LL |     three!("hip", "hop");
2020-01-09T06:54:59.3111877Z 
2020-01-09T06:54:59.3111930Z error: second final: "hop"
2020-01-09T06:54:59.3112268Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:50:19
2020-01-09T06:54:59.3112337Z    |
2020-01-09T06:54:59.3112337Z    |
2020-01-09T06:54:59.3112401Z LL |     three!("hip", "hop");
2020-01-09T06:54:59.3112504Z 
2020-01-09T06:54:59.3112557Z error: first source: "hip"
2020-01-09T06:54:59.3112802Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:50:12
2020-01-09T06:54:59.3112868Z    |
2020-01-09T06:54:59.3112868Z    |
2020-01-09T06:54:59.3112926Z LL |     three!("hip", "hop");
2020-01-09T06:54:59.3113027Z 
2020-01-09T06:54:59.3113079Z error: second source: "hop"
2020-01-09T06:54:59.3113321Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:50:19
2020-01-09T06:54:59.3113509Z    |
2020-01-09T06:54:59.3113509Z    |
2020-01-09T06:54:59.3113614Z LL |     three!("hip", "hop");
2020-01-09T06:54:59.3113722Z 
2020-01-09T06:54:59.3113781Z error[E0425]: cannot find value `ok` in this scope
2020-01-09T06:54:59.3114710Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:31:5
2020-01-09T06:54:59.3114796Z    |
2020-01-09T06:54:59.3114796Z    |
2020-01-09T06:54:59.3114872Z LL |     parent_source_spans!($($tokens)*);
2020-01-09T06:54:59.3115067Z ...
2020-01-09T06:54:59.3115067Z ...
2020-01-09T06:54:59.3115145Z LL |     one!("hello", "world");
2020-01-09T06:54:59.3115480Z 
2020-01-09T06:54:59.3115548Z error[E0425]: cannot find value `ok` in this scope
2020-01-09T06:54:59.3115830Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:31:5
2020-01-09T06:54:59.3115907Z    |
2020-01-09T06:54:59.3115907Z    |
2020-01-09T06:54:59.3115990Z LL |     parent_source_spans!($($tokens)*);
2020-01-09T06:54:59.3116182Z ...
2020-01-09T06:54:59.3116182Z ...
2020-01-09T06:54:59.3116242Z LL |     two!("yay", "rust");
2020-01-09T06:54:59.3116560Z 
2020-01-09T06:54:59.3116634Z error[E0425]: cannot find value `ok` in this scope
2020-01-09T06:54:59.3116909Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:31:5
2020-01-09T06:54:59.3116993Z    |
2020-01-09T06:54:59.3116993Z    |
2020-01-09T06:54:59.3117057Z LL |     parent_source_spans!($($tokens)*);
2020-01-09T06:54:59.3117238Z ...
2020-01-09T06:54:59.3117238Z ...
2020-01-09T06:54:59.3117306Z LL |     three!("hip", "hop");
2020-01-09T06:54:59.3117791Z 
2020-01-09T06:54:59.3118015Z error: aborting due to 21 previous errors
2020-01-09T06:54:59.3118077Z 
2020-01-09T06:54:59.3118325Z For more information about this error, try `rustc --explain E0425`.
---
2020-01-09T06:54:59.3119379Z 
2020-01-09T06:54:59.3119455Z 57 error: cannot find derive macro `Dlone` in this scope
2020-01-09T06:54:59.3120071Z 58   --> $DIR/resolve-error.rs:37:10
2020-01-09T06:54:59.3120154Z 59    |
2020-01-09T06:54:59.3120349Z - LL | #[derive(Dlone)]
2020-01-09T06:54:59.3120612Z -    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
2020-01-09T06:54:59.3120806Z -    | 
2020-01-09T06:54:59.3121018Z -   ::: $SRC_DIR/libcore/clone.rs:LL:COL
2020-01-09T06:54:59.3121361Z -    |
2020-01-09T06:54:59.3121575Z - LL | pub macro Clone($item:item) {
2020-01-09T06:54:59.3121834Z -    | --------------------------- similarly named derive macro `Clone` defined here
2020-01-09T06:54:59.3121917Z + LL |   #[derive(Dlone)]
2020-01-09T06:54:59.3121988Z +    |            ^^^^^ help: a derive macro with a similar name exists: `Clone`
2020-01-09T06:54:59.3122066Z 67 
2020-01-09T06:54:59.3122135Z 68 error: cannot find attribute `FooWithLongNan` in this scope
2020-01-09T06:54:59.3122386Z 
2020-01-09T06:54:59.3122422Z 
2020-01-09T06:54:59.3122482Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3122790Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
2020-01-09T06:54:59.3122790Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
2020-01-09T06:54:59.3123253Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3123531Z To only update this specific test, also pass `--test-args proc-macro/resolve-error.rs`
2020-01-09T06:54:59.3123813Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3124245Z status: exit code: 1
2020-01-09T06:54:59.3124245Z status: exit code: 1
2020-01-09T06:54:59.3125556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/resolve-error.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3126176Z ------------------------------------------
2020-01-09T06:54:59.3126231Z 
2020-01-09T06:54:59.3126503Z ------------------------------------------
2020-01-09T06:54:59.3126596Z stderr:
2020-01-09T06:54:59.3126596Z stderr:
2020-01-09T06:54:59.3126855Z ------------------------------------------
2020-01-09T06:54:59.3126944Z error: cannot find macro `bang_proc_macrp` in this scope
2020-01-09T06:54:59.3127333Z    |
2020-01-09T06:54:59.3127333Z    |
2020-01-09T06:54:59.3127406Z LL |     bang_proc_macrp!();
2020-01-09T06:54:59.3127496Z    |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `bang_proc_macro`
2020-01-09T06:54:59.3128060Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:15:1
2020-01-09T06:54:59.3128158Z    |
2020-01-09T06:54:59.3128158Z    |
2020-01-09T06:54:59.3128437Z LL | pub fn empty(_: TokenStream) -> TokenStream {
2020-01-09T06:54:59.3128772Z    | ------------------------------------------- similarly named macro `bang_proc_macro` defined here
2020-01-09T06:54:59.3128842Z 
2020-01-09T06:54:59.3128921Z error: cannot find macro `Dlona` in this scope
2020-01-09T06:54:59.3129704Z    |
2020-01-09T06:54:59.3129704Z    |
2020-01-09T06:54:59.3129775Z LL |     Dlona!();
2020-01-09T06:54:59.3130375Z 
2020-01-09T06:54:59.3130450Z error: cannot find macro `attr_proc_macra` in this scope
2020-01-09T06:54:59.3131334Z   --> /checkout/src/test/ui/proc-macro/resolve-error.rs:53:5
2020-01-09T06:54:59.3131404Z    |
2020-01-09T06:54:59.3131404Z    |
2020-01-09T06:54:59.3131644Z LL | / macro_rules! attr_proc_mac {
2020-01-09T06:54:59.3131777Z LL | | }
2020-01-09T06:54:59.3131777Z LL | | }
2020-01-09T06:54:59.3132172Z    | |_- similarly named macro `attr_proc_mac` defined here
2020-01-09T06:54:59.3132252Z ...
2020-01-09T06:54:59.3132306Z LL |       attr_proc_macra!();
2020-01-09T06:54:59.3132396Z    |       ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `attr_proc_mac`
2020-01-09T06:54:59.3132447Z 
2020-01-09T06:54:59.3132520Z error: cannot find macro `FooWithLongNama` in this scope
2020-01-09T06:54:59.3132857Z    |
2020-01-09T06:54:59.3132857Z    |
2020-01-09T06:54:59.3132910Z LL | / macro_rules! FooWithLongNam {
2020-01-09T06:54:59.3133040Z LL | | }
2020-01-09T06:54:59.3133040Z LL | | }
2020-01-09T06:54:59.3133276Z    | |_- similarly named macro `FooWithLongNam` defined here
2020-01-09T06:54:59.3133341Z ...
2020-01-09T06:54:59.3133401Z LL |       FooWithLongNama!();
2020-01-09T06:54:59.3133474Z    |       ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `FooWithLongNam`
2020-01-09T06:54:59.3133592Z error: cannot find derive macro `attr_proc_macra` in this scope
2020-01-09T06:54:59.3135242Z   --> /checkout/src/test/ui/proc-macro/resolve-error.rs:45:10
2020-01-09T06:54:59.3135378Z    |
2020-01-09T06:54:59.3135378Z    |
2020-01-09T06:54:59.3135453Z LL | #[derive(attr_proc_macra)]
2020-01-09T06:54:59.3135573Z 
2020-01-09T06:54:59.3135573Z 
2020-01-09T06:54:59.3135641Z error: cannot find derive macro `Dlona` in this scope
2020-01-09T06:54:59.3136322Z    |
2020-01-09T06:54:59.3136322Z    |
2020-01-09T06:54:59.3136392Z LL | #[derive(Dlona)]
2020-01-09T06:54:59.3136480Z    |          ^^^^^ help: a derive macro with a similar name exists: `Clona`
2020-01-09T06:54:59.3136561Z    | 
2020-01-09T06:54:59.3136871Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-clona.rs:11:1
2020-01-09T06:54:59.3136952Z    |
2020-01-09T06:54:59.3137219Z LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
2020-01-09T06:54:59.3137549Z    | ------------------------------------------------------- similarly named derive macro `Clona` defined here
2020-01-09T06:54:59.3137871Z error: cannot find derive macro `Dlone` in this scope
2020-01-09T06:54:59.3138122Z   --> /checkout/src/test/ui/proc-macro/resolve-error.rs:37:10
2020-01-09T06:54:59.3138189Z    |
2020-01-09T06:54:59.3138189Z    |
2020-01-09T06:54:59.3138255Z LL |   #[derive(Dlone)]
2020-01-09T06:54:59.3138337Z    |            ^^^^^ help: a derive macro with a similar name exists: `Clone`
2020-01-09T06:54:59.3138400Z 
2020-01-09T06:54:59.3138462Z error: cannot find attribute `FooWithLongNan` in this scope
2020-01-09T06:54:59.3138983Z    |
2020-01-09T06:54:59.3138983Z    |
2020-01-09T06:54:59.3139062Z LL | #[FooWithLongNan] //~ ERROR cannot find attribute `FooWithLongNan` in this scope
2020-01-09T06:54:59.3139185Z 
2020-01-09T06:54:59.3139246Z error: cannot find attribute `attr_proc_macra` in this scope
2020-01-09T06:54:59.3139499Z   --> /checkout/src/test/ui/proc-macro/resolve-error.rs:30:3
2020-01-09T06:54:59.3139566Z    |
2020-01-09T06:54:59.3139566Z    |
2020-01-09T06:54:59.3139643Z LL | #[attr_proc_macra] //~ ERROR cannot find attribute `attr_proc_macra` in this scope
2020-01-09T06:54:59.3139736Z    |   ^^^^^^^^^^^^^^^ help: an attribute macro with a similar name exists: `attr_proc_macro`
2020-01-09T06:54:59.3140084Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:20:1
2020-01-09T06:54:59.3140158Z    |
2020-01-09T06:54:59.3140158Z    |
2020-01-09T06:54:59.3140405Z LL | pub fn empty_attr(_: TokenStream, _: TokenStream) -> TokenStream {
2020-01-09T06:54:59.3140728Z    | ---------------------------------------------------------------- similarly named attribute macro `attr_proc_macro` defined here
2020-01-09T06:54:59.3140801Z 
2020-01-09T06:54:59.3141063Z error: cannot find derive macro `FooWithLongNan` in this scope
2020-01-09T06:54:59.3141551Z    |
2020-01-09T06:54:59.3141551Z    |
2020-01-09T06:54:59.3141614Z LL | #[derive(FooWithLongNan)]
2020-01-09T06:54:59.3141693Z    |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
2020-01-09T06:54:59.3142012Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-foo.rs:11:1
2020-01-09T06:54:59.3142089Z    |
2020-01-09T06:54:59.3142089Z    |
2020-01-09T06:54:59.3142313Z LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
2020-01-09T06:54:59.3142638Z    | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
2020-01-09T06:54:59.3142768Z error: aborting due to 10 previous errors
2020-01-09T06:54:59.3142809Z 
2020-01-09T06:54:59.3142854Z 
2020-01-09T06:54:59.3143061Z ------------------------------------------
2020-01-09T06:54:59.3143061Z ------------------------------------------
2020-01-09T06:54:59.3143104Z 
2020-01-09T06:54:59.3143144Z 
2020-01-09T06:54:59.3143357Z ---- [ui] ui/resolve/levenshtein.rs stdout ----
2020-01-09T06:54:59.3143434Z diff of stderr:
2020-01-09T06:54:59.3143471Z 
2020-01-09T06:54:59.3143544Z 16 error[E0412]: cannot find type `Opiton` in this scope
2020-01-09T06:54:59.3144421Z 17   --> $DIR/levenshtein.rs:15:10
2020-01-09T06:54:59.3144516Z 18    |
2020-01-09T06:54:59.3144831Z - LL | type B = Opiton<u8>; // Misspelled type name from the prelude.
2020-01-09T06:54:59.3145503Z -    | 
2020-01-09T06:54:59.3145503Z -    | 
2020-01-09T06:54:59.3145814Z -   ::: $SRC_DIR/libcore/option.rs:LL:COL
2020-01-09T06:54:59.3146278Z - LL | pub enum Option<T> {
2020-01-09T06:54:59.3146547Z -    | ------------------ similarly named enum `Option` defined here
2020-01-09T06:54:59.3146547Z -    | ------------------ similarly named enum `Option` defined here
2020-01-09T06:54:59.3146650Z + LL |   type B = Opiton<u8>; // Misspelled type name from the prelude.
2020-01-09T06:54:59.3146827Z 26 
2020-01-09T06:54:59.3146903Z 27 error[E0412]: cannot find type `Baz` in this scope
2020-01-09T06:54:59.3147339Z 28   --> $DIR/levenshtein.rs:19:14
2020-01-09T06:54:59.3147392Z 
2020-01-09T06:54:59.3147392Z 
2020-01-09T06:54:59.3147438Z 
2020-01-09T06:54:59.3147507Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3147992Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/levenshtein.stderr
2020-01-09T06:54:59.3148303Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3148597Z To only update this specific test, also pass `--test-args resolve/levenshtein.rs`
2020-01-09T06:54:59.3148715Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3148778Z status: exit code: 1
2020-01-09T06:54:59.3148778Z status: exit code: 1
2020-01-09T06:54:59.3149726Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/levenshtein.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3150223Z ------------------------------------------
2020-01-09T06:54:59.3150269Z 
2020-01-09T06:54:59.3150487Z ------------------------------------------
2020-01-09T06:54:59.3150552Z stderr:
2020-01-09T06:54:59.3150552Z stderr:
2020-01-09T06:54:59.3150761Z ------------------------------------------
2020-01-09T06:54:59.3150835Z error[E0412]: cannot find type `esize` in this scope
2020-01-09T06:54:59.3151076Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:8:11
2020-01-09T06:54:59.3151145Z    |
2020-01-09T06:54:59.3151215Z LL | fn foo(c: esize) {} // Misspelled primitive type name.
2020-01-09T06:54:59.3151296Z    |           ^^^^^ help: a builtin type with a similar name exists: `isize`
2020-01-09T06:54:59.3151416Z error[E0412]: cannot find type `Baz` in this scope
2020-01-09T06:54:59.3151661Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:13:10
2020-01-09T06:54:59.3151860Z    |
2020-01-09T06:54:59.3151927Z LL | enum Bar { }
2020-01-09T06:54:59.3151927Z LL | enum Bar { }
2020-01-09T06:54:59.3152184Z    | ------------ similarly named enum `Bar` defined here
2020-01-09T06:54:59.3152276Z LL | 
2020-01-09T06:54:59.3152354Z LL | type A = Baz; // Misspelled type name.
2020-01-09T06:54:59.3152475Z 
2020-01-09T06:54:59.3152475Z 
2020-01-09T06:54:59.3152550Z error[E0412]: cannot find type `Opiton` in this scope
2020-01-09T06:54:59.3152872Z    |
2020-01-09T06:54:59.3152872Z    |
2020-01-09T06:54:59.3152950Z LL |   type B = Opiton<u8>; // Misspelled type name from the prelude.
2020-01-09T06:54:59.3153082Z 
2020-01-09T06:54:59.3153152Z error[E0412]: cannot find type `Baz` in this scope
2020-01-09T06:54:59.3153398Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:19:14
2020-01-09T06:54:59.3153466Z    |
2020-01-09T06:54:59.3153466Z    |
2020-01-09T06:54:59.3153535Z LL |     type A = Baz; // No suggestion here, Bar is not visible
2020-01-09T06:54:59.3153807Z 
2020-01-09T06:54:59.3154240Z error[E0425]: cannot find value `MAXITEM` in this scope
2020-01-09T06:54:59.3154586Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:27:20
2020-01-09T06:54:59.3154664Z    |
2020-01-09T06:54:59.3154664Z    |
2020-01-09T06:54:59.3154731Z LL | const MAX_ITEM: usize = 10;
2020-01-09T06:54:59.3155020Z    | --------------------------- similarly named constant `MAX_ITEM` defined here
2020-01-09T06:54:59.3155112Z ...
2020-01-09T06:54:59.3155183Z LL |     let v = [0u32; MAXITEM]; // Misspelled constant name.
2020-01-09T06:54:59.3155284Z    |                    ^^^^^^^ help: a constant with a similar name exists: `MAX_ITEM`
2020-01-09T06:54:59.3155418Z error[E0425]: cannot find function `foobar` in this scope
2020-01-09T06:54:59.3155684Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:29:5
2020-01-09T06:54:59.3155764Z    |
2020-01-09T06:54:59.3155764Z    |
2020-01-09T06:54:59.3155824Z LL | fn foo_bar() {}
2020-01-09T06:54:59.3156110Z    | --------------- similarly named function `foo_bar` defined here
2020-01-09T06:54:59.3156271Z LL |     foobar(); // Misspelled function name.
2020-01-09T06:54:59.3156271Z LL |     foobar(); // Misspelled function name.
2020-01-09T06:54:59.3156354Z    |     ^^^^^^ help: a function with a similar name exists: `foo_bar`
2020-01-09T06:54:59.3156417Z 
2020-01-09T06:54:59.3156483Z error[E0412]: cannot find type `first` in module `m`
2020-01-09T06:54:59.3157019Z    |
2020-01-09T06:54:59.3157089Z LL |     pub struct First;
2020-01-09T06:54:59.3157089Z LL |     pub struct First;
2020-01-09T06:54:59.3157384Z    |     ----------------- similarly named struct `First` defined here
2020-01-09T06:54:59.3157471Z ...
2020-01-09T06:54:59.3157775Z LL |     let b: m::first = m::second; // Misspelled item in module.
2020-01-09T06:54:59.3158033Z    |               ^^^^^ help: a struct with a similar name exists (notice the capitalization): `First`
2020-01-09T06:54:59.3158169Z error[E0425]: cannot find value `second` in module `m`
2020-01-09T06:54:59.3158440Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:31:26
2020-01-09T06:54:59.3158509Z    |
2020-01-09T06:54:59.3158575Z LL |     pub struct Second;
2020-01-09T06:54:59.3158575Z LL |     pub struct Second;
2020-01-09T06:54:59.3159208Z    |     ------------------ similarly named unit struct `Second` defined here
2020-01-09T06:54:59.3159300Z ...
2020-01-09T06:54:59.3159530Z LL |     let b: m::first = m::second; // Misspelled item in module.
2020-01-09T06:54:59.3159643Z    |                          ^^^^^^ help: a unit struct with a similar name exists (notice the capitalization): `Second`
2020-01-09T06:54:59.3159774Z error: aborting due to 8 previous errors
2020-01-09T06:54:59.3159816Z 
2020-01-09T06:54:59.3159885Z Some errors have detailed explanations: E0412, E0425.
2020-01-09T06:54:59.3160146Z For more information about an error, try `rustc --explain E0412`.
---
2020-01-09T06:54:59.3161034Z 
2020-01-09T06:54:59.3161103Z 16 error: cannot find attribute `tests` in this scope
2020-01-09T06:54:59.3161329Z 17   --> $DIR/attribute-typos.rs:7:3
2020-01-09T06:54:59.3161405Z 18    |
2020-01-09T06:54:59.3161595Z - LL | #[tests]
2020-01-09T06:54:59.3162052Z -    | 
2020-01-09T06:54:59.3162052Z -    | 
2020-01-09T06:54:59.3162276Z -   ::: $SRC_DIR/libcore/macros/mod.rs:LL:COL
2020-01-09T06:54:59.3162465Z -    |
2020-01-09T06:54:59.3162681Z - LL |     pub macro test($item:item) {
2020-01-09T06:54:59.3163490Z -    |     -------------------------- similarly named attribute macro `test` defined here
2020-01-09T06:54:59.3163576Z + LL |   #[tests]
2020-01-09T06:54:59.3163830Z 26 
2020-01-09T06:54:59.3164424Z 27 error: cannot find attribute `deprcated` in this scope
2020-01-09T06:54:59.3164745Z 28   --> $DIR/attribute-typos.rs:4:3
2020-01-09T06:54:59.3164792Z 
2020-01-09T06:54:59.3164792Z 
2020-01-09T06:54:59.3164837Z 
2020-01-09T06:54:59.3164905Z The actual stderr differed from the expected stderr.
2020-01-09T06:54:59.3165269Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/attribute-typos.stderr
2020-01-09T06:54:59.3165562Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T06:54:59.3166084Z To only update this specific test, also pass `--test-args suggestions/attribute-typos.rs`
2020-01-09T06:54:59.3166230Z error: 1 errors occurred comparing output.
2020-01-09T06:54:59.3166299Z status: exit code: 1
2020-01-09T06:54:59.3166299Z status: exit code: 1
2020-01-09T06:54:59.3167399Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/attribute-typos.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/auxiliary" "-A" "unused"
2020-01-09T06:54:59.3168076Z ------------------------------------------
2020-01-09T06:54:59.3168122Z 
2020-01-09T06:54:59.3168331Z ------------------------------------------
2020-01-09T06:54:59.3168402Z stderr:
---
2020-01-09T06:54:59.3171785Z 
2020-01-09T06:54:59.3172017Z error: cannot find attribute `tests` in this scope
2020-01-09T06:54:59.3172260Z   --> /checkout/src/test/ui/suggestions/attribute-typos.rs:7:3
2020-01-09T06:54:59.3172531Z    |
2020-01-09T06:54:59.3172604Z LL |   #[tests] //~ ERROR cannot find attribute `tests` in this scope
2020-01-09T06:54:59.3172748Z 
2020-01-09T06:54:59.3172819Z error: cannot find attribute `deprcated` in this scope
2020-01-09T06:54:59.3173074Z   --> /checkout/src/test/ui/suggestions/attribute-typos.rs:4:3
2020-01-09T06:54:59.3173152Z    |
2020-01-09T06:54:59.3173152Z    |
2020-01-09T06:54:59.3173226Z LL | #[deprcated] //~ ERROR cannot find attribute `deprcated` in this scope
2020-01-09T06:54:59.3173508Z    |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `deprecated`
2020-01-09T06:54:59.3173629Z error: aborting due to 4 previous errors
2020-01-09T06:54:59.3173671Z 
2020-01-09T06:54:59.3174119Z For more information about this error, try `rustc --explain E0658`.
2020-01-09T06:54:59.3174357Z 
---
2020-01-09T06:54:59.3178412Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-09T06:54:59.3178528Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-09T06:54:59.3178589Z 
2020-01-09T06:54:59.3178622Z 
2020-01-09T06:54:59.3180722Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "ui" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-09T06:54:59.3181380Z 
2020-01-09T06:54:59.3181416Z 
2020-01-09T06:54:59.3181767Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-01-09T06:54:59.3181862Z Build completed unsuccessfully in 1:34:23
2020-01-09T06:54:59.3181862Z Build completed unsuccessfully in 1:34:23
2020-01-09T06:54:59.3181949Z == clock drift check ==
2020-01-09T06:54:59.3182020Z   local time: Thu Jan  9 06:54:59 UTC 2020
2020-01-09T06:55:00.8632674Z   network time: Thu, 09 Jan 2020 06:55:00 GMT
2020-01-09T06:55:00.8636178Z == end clock drift check ==
2020-01-09T06:55:01.4405807Z 
2020-01-09T06:55:01.4528564Z ##[error]Bash exited with code '1'.
2020-01-09T06:55:01.4572004Z ##[section]Starting: Checkout
2020-01-09T06:55:01.4573986Z ==============================================================================
2020-01-09T06:55:01.4574069Z Task         : Get sources
2020-01-09T06:55:01.4574561Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
