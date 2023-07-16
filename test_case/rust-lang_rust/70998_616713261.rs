plain
2020-04-20T16:47:07.8784716Z ========================== Starting Command Output ===========================
2020-04-20T16:47:07.8787133Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/84a12259-eb5e-4886-81c7-6ad2e95cad14.sh
2020-04-20T16:47:07.8787375Z 
2020-04-20T16:47:07.8791109Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T16:47:07.8808762Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-20T16:47:07.8811817Z Task         : Get sources
2020-04-20T16:47:07.8812093Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T16:47:07.8812359Z Version      : 1.0.0
2020-04-20T16:47:07.8812542Z Author       : Microsoft
---
2020-04-20T16:47:08.8857972Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T16:47:08.8863396Z ##[command]git config gc.auto 0
2020-04-20T16:47:08.8866991Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T16:47:08.8870307Z ##[command]git config --get-all http.proxy
2020-04-20T16:47:08.8876320Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70998/merge:refs/remotes/pull/70998/merge
---
2020-04-20T16:50:10.5547815Z  ---> 318032b5f0e2
2020-04-20T16:50:10.5548811Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-20T16:50:10.5558278Z  ---> Using cache
2020-04-20T16:50:10.5558973Z  ---> d44a858fd1ce
2020-04-20T16:50:10.5560115Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-20T16:50:10.5568137Z  ---> 58b910f50f5a
2020-04-20T16:50:10.5568585Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-20T16:50:10.5569147Z  ---> Using cache
2020-04-20T16:50:10.5569682Z  ---> ee7702aadba1
---
2020-04-20T16:50:10.6379234Z Looks like docker image is the same as before, not uploading
2020-04-20T16:50:18.5519025Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T16:50:18.5768271Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T16:50:18.5796700Z == clock drift check ==
2020-04-20T16:50:18.5807751Z   local time: Mon Apr 20 16:50:18 UTC 2020
2020-04-20T16:50:18.9035435Z   network time: Mon, 20 Apr 2020 16:50:18 GMT
2020-04-20T16:50:18.9066952Z Starting sccache server...
2020-04-20T16:50:18.9878460Z configure: processing command line
2020-04-20T16:50:18.9878865Z configure: 
2020-04-20T16:50:18.9880163Z configure: rust.dist-src        := False
---
2020-04-20T16:55:29.8883209Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T16:55:31.2704233Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T16:55:32.8709410Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T16:55:33.8981120Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T16:55:42.9561207Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T16:55:44.7269065Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T16:55:48.9847670Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T16:55:53.0026233Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T16:56:02.8873040Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T17:17:30.1025837Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T17:17:31.6842733Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T17:17:33.4405290Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T17:17:33.7910726Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T17:17:43.6913459Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T17:17:46.0060476Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T17:17:50.5579146Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T17:17:54.9408451Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T17:18:04.3924344Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T17:40:22.7006540Z .................................................................................................... 1700/9908
2020-04-20T17:40:26.7941737Z .................................................................................................... 1800/9908
2020-04-20T17:40:35.4424693Z ...................................................................................................i 1900/9908
2020-04-20T17:40:43.0823456Z .................................................................................................... 2000/9908
2020-04-20T17:40:49.2837211Z .........................................................................................iiiii...... 2100/9908
2020-04-20T17:41:09.5223588Z .................................................................................................... 2300/9908
2020-04-20T17:41:11.7736011Z .................................................................................................... 2400/9908
2020-04-20T17:41:14.1093898Z .................................................................................................... 2500/9908
2020-04-20T17:41:20.0222233Z ......F............................................................................................. 2600/9908
---
2020-04-20T17:44:06.2518065Z .................................................................i...............i.................. 5000/9908
2020-04-20T17:44:13.4656331Z .................................................................................................... 5100/9908
2020-04-20T17:44:20.2811089Z .................................................................................................... 5200/9908
2020-04-20T17:44:25.4135222Z ...........i........................................................................................ 5300/9908
2020-04-20T17:44:35.6648528Z .i.................................................................................................. 5400/9908
2020-04-20T17:44:40.8939012Z .ii.ii........i...i................................................................................. 5500/9908
2020-04-20T17:44:48.9888494Z ................................................i................................................... 5700/9908
2020-04-20T17:44:58.0239929Z ................................................................................ii.................. 5800/9908
2020-04-20T17:45:05.2202765Z ...................i................................................................................ 5900/9908
2020-04-20T17:45:10.5172798Z .................................................................................................... 6000/9908
2020-04-20T17:45:10.5172798Z .................................................................................................... 6000/9908
2020-04-20T17:45:20.9805236Z .................................................................................................... 6100/9908
2020-04-20T17:45:31.3440010Z .............ii...i..ii...........i................................................................. 6200/9908
2020-04-20T17:45:47.1793046Z .................................................................................................... 6400/9908
2020-04-20T17:45:50.9235532Z .................................................................................................... 6500/9908
2020-04-20T17:45:50.9235532Z .................................................................................................... 6500/9908
2020-04-20T17:46:01.7312475Z ...........................................i..ii.................................................... 6600/9908
2020-04-20T17:46:22.5126337Z .................................................................................................... 6800/9908
2020-04-20T17:46:24.6081358Z ............................................i....................................................... 6900/9908
2020-04-20T17:46:26.5936268Z .................................................................................................... 7000/9908
2020-04-20T17:46:28.6211212Z ....................................................................................i............... 7100/9908
---
2020-04-20T17:48:02.1324281Z .................................................................................................... 7800/9908
2020-04-20T17:48:06.8081031Z .................................................................................................... 7900/9908
2020-04-20T17:48:13.1225189Z .................................................................................................... 8000/9908
2020-04-20T17:48:18.5619598Z ..................................................i................................................. 8100/9908
2020-04-20T17:48:28.3086876Z ...................................................................................................i 8200/9908
2020-04-20T17:48:33.4576388Z iiiii.iiiii.i....................................................................................... 8300/9908
2020-04-20T17:48:46.2884906Z .................................................................................................... 8500/9908
2020-04-20T17:48:53.8781634Z .................................................................................................... 8600/9908
2020-04-20T17:49:06.9832380Z .................................................................................................... 8700/9908
2020-04-20T17:49:13.2903622Z .................................................................................................... 8800/9908
---
2020-04-20T17:50:58.3163856Z 
2020-04-20T17:50:58.3164349Z 4 LL | fn foo() -> dyn Trait { Struct }
2020-04-20T17:50:58.3165254Z 5    |             ^^^^^^^^^ doesn't have a size known at compile-time
2020-04-20T17:50:58.3165604Z 6    |
2020-04-20T17:50:58.3166477Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3167426Z - help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
2020-04-20T17:50:58.3168002Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3168903Z + LL | fn foo() -> T { Struct }
2020-04-20T17:50:58.3169216Z +    |             ^
2020-04-20T17:50:58.3169216Z +    |             ^
2020-04-20T17:50:58.3169690Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3170611Z 10 LL | fn foo() -> impl Trait { Struct }
2020-04-20T17:50:58.3170974Z 11    |             ^^^^^^^^^^
2020-04-20T17:50:58.3171358Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3171698Z +    |
2020-04-20T17:50:58.3171698Z +    |
2020-04-20T17:50:58.3172206Z + LL | fn foo() -> Box<dyn Trait> { Struct }
2020-04-20T17:50:58.3172841Z 12 
2020-04-20T17:50:58.3173169Z 13 error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3173726Z 14   --> $DIR/E0746.rs:11:13
2020-04-20T17:50:58.3173986Z 
2020-04-20T17:50:58.3173986Z 
2020-04-20T17:50:58.3174454Z 16 LL | fn bar() -> dyn Trait {
2020-04-20T17:50:58.3175298Z 17    |             ^^^^^^^^^ doesn't have a size known at compile-time
2020-04-20T17:50:58.3176400Z 18    |
2020-04-20T17:50:58.3179380Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3180536Z - help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
2020-04-20T17:50:58.3181141Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3182033Z + LL | fn bar() -> T {
2020-04-20T17:50:58.3187703Z +    |             ^
2020-04-20T17:50:58.3187703Z +    |             ^
2020-04-20T17:50:58.3197225Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3198676Z 22 LL | fn bar() -> impl Trait {
2020-04-20T17:50:58.3198883Z 23    |             ^^^^^^^^^^
2020-04-20T17:50:58.3199175Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3199413Z +    |
---
2020-04-20T17:50:58.3200554Z 26 
2020-04-20T17:50:58.3200654Z 
2020-04-20T17:50:58.3200745Z 
2020-04-20T17:50:58.3200932Z The actual stderr differed from the expected stderr.
2020-04-20T17:50:58.3201544Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0746/E0746.stderr
2020-04-20T17:50:58.3202772Z thread '[ui] ui/error-codes/E0746.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/error-codes/E0746.rs" with rustfix: Cannot replace slice of data that was already replaced', src/tools/compiletest/src/runtest.rs:2965:30
2020-04-20T17:50:58.3203600Z 
2020-04-20T17:50:58.3204034Z ---- [ui] ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs stdout ----
2020-04-20T17:50:58.3204289Z diff of stderr:
2020-04-20T17:50:58.3204425Z 
2020-04-20T17:50:58.3204425Z 
2020-04-20T17:50:58.3204760Z 48 LL | fn bap() -> Trait { Struct }
2020-04-20T17:50:58.3205635Z 50    |
2020-04-20T17:50:58.3205635Z 50    |
2020-04-20T17:50:58.3206271Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3207094Z - help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
2020-04-20T17:50:58.3207565Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3207849Z 53    |
2020-04-20T17:50:58.3208192Z + LL | fn bap() -> T { Struct }
2020-04-20T17:50:58.3208394Z +    |             ^
2020-04-20T17:50:58.3208738Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3209430Z 54 LL | fn bap() -> impl Trait { Struct }
2020-04-20T17:50:58.3209650Z 55    |             ^^^^^^^^^^
2020-04-20T17:50:58.3209926Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3210792Z +    |
2020-04-20T17:50:58.3210792Z +    |
2020-04-20T17:50:58.3211192Z + LL | fn bap() -> Box<dyn Trait> { Struct }
2020-04-20T17:50:58.3211605Z 56 
2020-04-20T17:50:58.3211823Z 57 error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3212345Z 58   --> $DIR/dyn-trait-return-should-be-impl-trait.rs:15:13
2020-04-20T17:50:58.3212548Z 
2020-04-20T17:50:58.3212548Z 
2020-04-20T17:50:58.3212909Z 60 LL | fn ban() -> dyn Trait { Struct }
2020-04-20T17:50:58.3213612Z 62    |
2020-04-20T17:50:58.3213612Z 62    |
2020-04-20T17:50:58.3214245Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3214988Z - help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
2020-04-20T17:50:58.3215435Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3215738Z 65    |
2020-04-20T17:50:58.3216070Z + LL | fn ban() -> T { Struct }
2020-04-20T17:50:58.3216265Z +    |             ^
2020-04-20T17:50:58.3216631Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3217305Z 66 LL | fn ban() -> impl Trait { Struct }
2020-04-20T17:50:58.3217537Z 67    |             ^^^^^^^^^^
2020-04-20T17:50:58.3217805Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3218036Z +    |
2020-04-20T17:50:58.3218036Z +    |
2020-04-20T17:50:58.3218409Z + LL | fn ban() -> Box<dyn Trait> { Struct }
2020-04-20T17:50:58.3218796Z 68 
2020-04-20T17:50:58.3219027Z 69 error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3219546Z 70   --> $DIR/dyn-trait-return-should-be-impl-trait.rs:17:13
2020-04-20T17:50:58.3219751Z 
2020-04-20T17:50:58.3219751Z 
2020-04-20T17:50:58.3220076Z 91 LL | fn bal() -> dyn Trait {
2020-04-20T17:50:58.3220787Z 93    |
2020-04-20T17:50:58.3220787Z 93    |
2020-04-20T17:50:58.3221480Z -    = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
2020-04-20T17:50:58.3222274Z -    = note: if all the returned values were of the same type you could use `impl Trait` as the return type
2020-04-20T17:50:58.3223037Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3223718Z -    = note: you can create a new `enum` with a variant for each returned type
2020-04-20T17:50:58.3224189Z - help: return a boxed trait object instead
2020-04-20T17:50:58.3224636Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3224936Z 99    |
2020-04-20T17:50:58.3225286Z - LL | fn bal() -> Box<dyn Trait> {
2020-04-20T17:50:58.3225633Z - LL |     if true {
2020-04-20T17:50:58.3226088Z - LL |         return Box::new(Struct);
2020-04-20T17:50:58.3226452Z - LL |     }
2020-04-20T17:50:58.3226783Z - LL |     Box::new(42)
2020-04-20T17:50:58.3227122Z + LL | fn bal() -> T {
2020-04-20T17:50:58.3227314Z +    |             ^
2020-04-20T17:50:58.3227656Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3227982Z 105    |
2020-04-20T17:50:58.3228329Z + LL | fn bal() -> impl Trait {
2020-04-20T17:50:58.3228793Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3229039Z +    |
2020-04-20T17:50:58.3229039Z +    |
2020-04-20T17:50:58.3229382Z + LL | fn bal() -> Box<dyn Trait> {
2020-04-20T17:50:58.3229773Z 106 
2020-04-20T17:50:58.3229982Z 107 error[E0308]: `if` and `else` have incompatible types
2020-04-20T17:50:58.3230491Z 108   --> $DIR/dyn-trait-return-should-be-impl-trait.rs:29:9
2020-04-20T17:50:58.3230707Z 
2020-04-20T17:50:58.3230707Z 
2020-04-20T17:50:58.3231032Z 122 LL | fn bax() -> dyn Trait {
2020-04-20T17:50:58.3231723Z 124    |
2020-04-20T17:50:58.3231723Z 124    |
2020-04-20T17:50:58.3232425Z -    = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
2020-04-20T17:50:58.3233207Z -    = note: if all the returned values were of the same type you could use `impl Trait` as the return type
2020-04-20T17:50:58.3233984Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3234653Z -    = note: you can create a new `enum` with a variant for each returned type
2020-04-20T17:50:58.3235126Z - help: return a boxed trait object instead
2020-04-20T17:50:58.3235491Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3235780Z 130    |
2020-04-20T17:50:58.3236121Z - LL | fn bax() -> Box<dyn Trait> {
2020-04-20T17:50:58.3236481Z - LL |     if true {
2020-04-20T17:50:58.3236842Z - LL |         Box::new(Struct)
2020-04-20T17:50:58.3237190Z - LL |     } else {
2020-04-20T17:50:58.3240351Z - LL |         Box::new(42)
2020-04-20T17:50:58.3240720Z + LL | fn bax() -> T {
2020-04-20T17:50:58.3240894Z +    |             ^
2020-04-20T17:50:58.3244758Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3245141Z 136    |
2020-04-20T17:50:58.3245777Z + LL | fn bax() -> impl Trait {
2020-04-20T17:50:58.3246285Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3246517Z +    |
2020-04-20T17:50:58.3246517Z +    |
2020-04-20T17:50:58.3246886Z + LL | fn bax() -> Box<dyn Trait> {
2020-04-20T17:50:58.3247279Z 137 
2020-04-20T17:50:58.3247453Z 138 error[E0308]: mismatched types
2020-04-20T17:50:58.3247956Z 139   --> $DIR/dyn-trait-return-should-be-impl-trait.rs:34:16
2020-04-20T17:50:58.3248162Z 
2020-04-20T17:50:58.3248162Z 
2020-04-20T17:50:58.3248488Z 269 LL | fn bat() -> dyn Trait {
2020-04-20T17:50:58.3249200Z 271    |
2020-04-20T17:50:58.3249200Z 271    |
2020-04-20T17:50:58.3249822Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3250589Z - help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
2020-04-20T17:50:58.3251173Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3251461Z 274    |
2020-04-20T17:50:58.3251802Z + LL | fn bat() -> T {
2020-04-20T17:50:58.3252041Z +    |             ^
2020-04-20T17:50:58.3252669Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3254114Z 275 LL | fn bat() -> impl Trait {
2020-04-20T17:50:58.3254321Z 276    |             ^^^^^^^^^^
2020-04-20T17:50:58.3254591Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3254839Z +    |
2020-04-20T17:50:58.3254839Z +    |
2020-04-20T17:50:58.3255178Z + LL | fn bat() -> Box<dyn Trait> {
2020-04-20T17:50:58.3255572Z 277 
2020-04-20T17:50:58.3255794Z 278 error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3256325Z 279   --> $DIR/dyn-trait-return-should-be-impl-trait.rs:66:13
2020-04-20T17:50:58.3256544Z 
2020-04-20T17:50:58.3256544Z 
2020-04-20T17:50:58.3257062Z 281 LL | fn bay() -> dyn Trait {
2020-04-20T17:50:58.3257785Z 283    |
2020-04-20T17:50:58.3257785Z 283    |
2020-04-20T17:50:58.3258407Z -    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-20T17:50:58.3259160Z - help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
2020-04-20T17:50:58.3259620Z + help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3260223Z + LL | fn bay() -> T {
2020-04-20T17:50:58.3260413Z +    |             ^
2020-04-20T17:50:58.3260413Z +    |             ^
2020-04-20T17:50:58.3260755Z + help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3261437Z 287 LL | fn bay() -> impl Trait {
2020-04-20T17:50:58.3261643Z 288    |             ^^^^^^^^^^
2020-04-20T17:50:58.3261915Z + help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3262164Z +    |
2020-04-20T17:50:58.3262164Z +    |
2020-04-20T17:50:58.3262505Z + LL | fn bay() -> Box<dyn Trait> {
2020-04-20T17:50:58.3262882Z 289 
2020-04-20T17:50:58.3263085Z 290 error: aborting due to 20 previous errors
2020-04-20T17:50:58.3263271Z 291 
2020-04-20T17:50:58.3263368Z 
2020-04-20T17:50:58.3263368Z 
2020-04-20T17:50:58.3263458Z 
2020-04-20T17:50:58.3263662Z The actual stderr differed from the expected stderr.
2020-04-20T17:50:58.3264388Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait/dyn-trait-return-should-be-impl-trait.stderr
2020-04-20T17:50:58.3265034Z To update references, rerun the tests and pass the `--bless` flag
2020-04-20T17:50:58.3265685Z To only update this specific test, also pass `--test-args impl-trait/dyn-trait-return-should-be-impl-trait.rs`
2020-04-20T17:50:58.3266113Z error: 1 errors occurred comparing output.
2020-04-20T17:50:58.3266344Z status: exit code: 1
2020-04-20T17:50:58.3266344Z status: exit code: 1
2020-04-20T17:50:58.3268259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait/auxiliary"
2020-04-20T17:50:58.3269881Z ------------------------------------------
2020-04-20T17:50:58.3270039Z 
2020-04-20T17:50:58.3270407Z ------------------------------------------
2020-04-20T17:50:58.3270663Z stderr:
2020-04-20T17:50:58.3270663Z stderr:
2020-04-20T17:50:58.3271036Z ------------------------------------------
2020-04-20T17:50:58.3271273Z error[E0308]: mismatched types
2020-04-20T17:50:58.3271799Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:7:35
2020-04-20T17:50:58.3272066Z    |
2020-04-20T17:50:58.3272460Z LL | fn fuz() -> (usize, Trait) { (42, Struct) }
2020-04-20T17:50:58.3272788Z    |                                   ^^^^^^ expected trait object `dyn Trait`, found struct `Struct`
2020-04-20T17:50:58.3273049Z    |
2020-04-20T17:50:58.3273457Z    = note: expected trait object `(dyn Trait + 'static)`
2020-04-20T17:50:58.3273867Z 
2020-04-20T17:50:58.3273867Z 
2020-04-20T17:50:58.3274385Z error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
2020-04-20T17:50:58.3275268Z    |
2020-04-20T17:50:58.3275268Z    |
2020-04-20T17:50:58.3275660Z LL | fn fuz() -> (usize, Trait) { (42, Struct) }
2020-04-20T17:50:58.3276238Z    |             ^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
2020-04-20T17:50:58.3276968Z    |             doesn't have a size known at compile-time
2020-04-20T17:50:58.3277171Z    |
2020-04-20T17:50:58.3277171Z    |
2020-04-20T17:50:58.3277774Z    = help: within `(usize, (dyn Trait + 'static))`, the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
2020-04-20T17:50:58.3278604Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-20T17:50:58.3279279Z    = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
2020-04-20T17:50:58.3279858Z 
2020-04-20T17:50:58.3280017Z error[E0308]: mismatched types
2020-04-20T17:50:58.3283608Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:10:39
2020-04-20T17:50:58.3284051Z    |
2020-04-20T17:50:58.3284051Z    |
2020-04-20T17:50:58.3284478Z LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
2020-04-20T17:50:58.3284820Z    |                                       ^^^^^^ expected trait object `dyn Trait`, found struct `Struct`
2020-04-20T17:50:58.3285223Z    |
2020-04-20T17:50:58.3285653Z    = note: expected trait object `(dyn Trait + 'static)`
2020-04-20T17:50:58.3286063Z 
2020-04-20T17:50:58.3286063Z 
2020-04-20T17:50:58.3294378Z error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
2020-04-20T17:50:58.3295350Z    |
2020-04-20T17:50:58.3295350Z    |
2020-04-20T17:50:58.3296263Z LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
2020-04-20T17:50:58.3296897Z    |             ^^^^^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
2020-04-20T17:50:58.3297640Z    |             doesn't have a size known at compile-time
2020-04-20T17:50:58.3297843Z    |
2020-04-20T17:50:58.3297843Z    |
2020-04-20T17:50:58.3298451Z    = help: within `(usize, (dyn Trait + 'static))`, the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
2020-04-20T17:50:58.3299286Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-04-20T17:50:58.3299961Z    = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
2020-04-20T17:50:58.3300959Z 
2020-04-20T17:50:58.3301593Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3302241Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:13:13
2020-04-20T17:50:58.3302616Z    |
2020-04-20T17:50:58.3302616Z    |
2020-04-20T17:50:58.3302970Z LL | fn bap() -> Trait { Struct }
2020-04-20T17:50:58.3303652Z    |
2020-04-20T17:50:58.3303934Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3304210Z    |
2020-04-20T17:50:58.3304550Z LL | fn bap() -> T { Struct }
2020-04-20T17:50:58.3304550Z LL | fn bap() -> T { Struct }
2020-04-20T17:50:58.3304730Z    |             ^
2020-04-20T17:50:58.3305068Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3305398Z    |
2020-04-20T17:50:58.3305747Z LL | fn bap() -> impl Trait { Struct }
2020-04-20T17:50:58.3305960Z    |             ^^^^^^^^^^
2020-04-20T17:50:58.3306237Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3306460Z    |
2020-04-20T17:50:58.3306815Z LL | fn bap() -> Box<dyn Trait> { Struct }
2020-04-20T17:50:58.3307191Z 
2020-04-20T17:50:58.3307396Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3307966Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:15:13
2020-04-20T17:50:58.3308244Z    |
2020-04-20T17:50:58.3308244Z    |
2020-04-20T17:50:58.3308588Z LL | fn ban() -> dyn Trait { Struct }
2020-04-20T17:50:58.3309281Z    |
2020-04-20T17:50:58.3309561Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3309843Z    |
2020-04-20T17:50:58.3310182Z LL | fn ban() -> T { Struct }
2020-04-20T17:50:58.3310182Z LL | fn ban() -> T { Struct }
2020-04-20T17:50:58.3310370Z    |             ^
2020-04-20T17:50:58.3310706Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3311035Z    |
2020-04-20T17:50:58.3311381Z LL | fn ban() -> impl Trait { Struct }
2020-04-20T17:50:58.3311588Z    |             ^^^^^^^^^^
2020-04-20T17:50:58.3311865Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3312088Z    |
2020-04-20T17:50:58.3312442Z LL | fn ban() -> Box<dyn Trait> { Struct }
2020-04-20T17:50:58.3312812Z 
2020-04-20T17:50:58.3313018Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3313583Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:17:13
2020-04-20T17:50:58.3313863Z    |
2020-04-20T17:50:58.3313863Z    |
2020-04-20T17:50:58.3314269Z LL | fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0746
2020-04-20T17:50:58.3315005Z    |
2020-04-20T17:50:58.3315284Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3315565Z    |
2020-04-20T17:50:58.3315565Z    |
2020-04-20T17:50:58.3315966Z LL | fn bak() -> T { unimplemented!() } //~ ERROR E0746
2020-04-20T17:50:58.3316525Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3316858Z    |
2020-04-20T17:50:58.3316858Z    |
2020-04-20T17:50:58.3317267Z LL | fn bak() -> impl Trait { unimplemented!() } //~ ERROR E0746
2020-04-20T17:50:58.3317789Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3318013Z    |
2020-04-20T17:50:58.3318013Z    |
2020-04-20T17:50:58.3318431Z LL | fn bak() -> Box<dyn Trait> { unimplemented!() } //~ ERROR E0746
2020-04-20T17:50:58.3318903Z 
2020-04-20T17:50:58.3319106Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3319684Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:19:13
2020-04-20T17:50:58.3320010Z    |
2020-04-20T17:50:58.3320010Z    |
2020-04-20T17:50:58.3320384Z LL | fn bal() -> dyn Trait { //~ ERROR E0746
2020-04-20T17:50:58.3321088Z    |
2020-04-20T17:50:58.3321369Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3321644Z    |
2020-04-20T17:50:58.3321644Z    |
2020-04-20T17:50:58.3322566Z LL | fn bal() -> T { //~ ERROR E0746
2020-04-20T17:50:58.3323100Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3323426Z    |
2020-04-20T17:50:58.3323426Z    |
2020-04-20T17:50:58.3323806Z LL | fn bal() -> impl Trait { //~ ERROR E0746
2020-04-20T17:50:58.3324308Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3324532Z    |
2020-04-20T17:50:58.3324532Z    |
2020-04-20T17:50:58.3324911Z LL | fn bal() -> Box<dyn Trait> { //~ ERROR E0746
2020-04-20T17:50:58.3325298Z 
2020-04-20T17:50:58.3325496Z error[E0308]: `if` and `else` have incompatible types
2020-04-20T17:50:58.3326059Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:29:9
2020-04-20T17:50:58.3326337Z    |
---
2020-04-20T17:50:58.3328594Z 
2020-04-20T17:50:58.3328814Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3329387Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:25:13
2020-04-20T17:50:58.3329654Z    |
2020-04-20T17:50:58.3330031Z LL | fn bax() -> dyn Trait { //~ ERROR E0746
2020-04-20T17:50:58.3330724Z    |
2020-04-20T17:50:58.3331017Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3331292Z    |
2020-04-20T17:50:58.3331292Z    |
2020-04-20T17:50:58.3331634Z LL | fn bax() -> T { //~ ERROR E0746
2020-04-20T17:50:58.3332173Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3332489Z    |
2020-04-20T17:50:58.3332489Z    |
2020-04-20T17:50:58.3332865Z LL | fn bax() -> impl Trait { //~ ERROR E0746
2020-04-20T17:50:58.3333349Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3333593Z    |
2020-04-20T17:50:58.3333593Z    |
2020-04-20T17:50:58.3333967Z LL | fn bax() -> Box<dyn Trait> { //~ ERROR E0746
2020-04-20T17:50:58.3334334Z 
2020-04-20T17:50:58.3334508Z error[E0308]: mismatched types
2020-04-20T17:50:58.3335030Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:34:16
2020-04-20T17:50:58.3335290Z    |
2020-04-20T17:50:58.3335290Z    |
2020-04-20T17:50:58.3335634Z LL | fn bam() -> Box<dyn Trait> {
2020-04-20T17:50:58.3336202Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3336749Z LL |         return Struct; //~ ERROR mismatched types
2020-04-20T17:50:58.3337058Z    |                ^^^^^^
2020-04-20T17:50:58.3337235Z    |                |
2020-04-20T17:50:58.3337527Z    |                expected struct `std::boxed::Box`, found struct `Struct`
2020-04-20T17:50:58.3337527Z    |                expected struct `std::boxed::Box`, found struct `Struct`
2020-04-20T17:50:58.3337990Z    |                help: store this in the heap by calling `Box::new`: `Box::new(Struct)`
2020-04-20T17:50:58.3338284Z    |
2020-04-20T17:50:58.3338745Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3339026Z               found struct `Struct`
2020-04-20T17:50:58.3339935Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3340620Z error[E0308]: mismatched types
2020-04-20T17:50:58.3341145Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:36:5
2020-04-20T17:50:58.3341429Z    |
2020-04-20T17:50:58.3341429Z    |
2020-04-20T17:50:58.3341763Z LL | fn bam() -> Box<dyn Trait> {
2020-04-20T17:50:58.3342332Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3342817Z LL |     42 //~ ERROR mismatched types
2020-04-20T17:50:58.3343001Z    |     ^^
2020-04-20T17:50:58.3343157Z    |     |
2020-04-20T17:50:58.3343383Z    |     expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3343383Z    |     expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3343742Z    |     help: store this in the heap by calling `Box::new`: `Box::new(42)`
2020-04-20T17:50:58.3344016Z    |
2020-04-20T17:50:58.3344449Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3344737Z                 found type `{integer}`
2020-04-20T17:50:58.3345661Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3346356Z error[E0308]: mismatched types
2020-04-20T17:50:58.3347182Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:40:16
2020-04-20T17:50:58.3347930Z    |
2020-04-20T17:50:58.3347930Z    |
2020-04-20T17:50:58.3348287Z LL | fn baq() -> Box<dyn Trait> {
2020-04-20T17:50:58.3348862Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3353941Z LL |         return 0; //~ ERROR mismatched types
2020-04-20T17:50:58.3354158Z    |                ^
2020-04-20T17:50:58.3354354Z    |                |
2020-04-20T17:50:58.3355333Z    |                expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3355333Z    |                expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3355734Z    |                help: store this in the heap by calling `Box::new`: `Box::new(0)`
2020-04-20T17:50:58.3356985Z    |
2020-04-20T17:50:58.3358061Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3358371Z                 found type `{integer}`
2020-04-20T17:50:58.3359322Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3359985Z error[E0308]: mismatched types
2020-04-20T17:50:58.3360532Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:42:5
2020-04-20T17:50:58.3360792Z    |
2020-04-20T17:50:58.3360792Z    |
2020-04-20T17:50:58.3361126Z LL | fn baq() -> Box<dyn Trait> {
2020-04-20T17:50:58.3361707Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3362456Z LL |     42 //~ ERROR mismatched types
2020-04-20T17:50:58.3362654Z    |     ^^
2020-04-20T17:50:58.3362795Z    |     |
2020-04-20T17:50:58.3363020Z    |     expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3363020Z    |     expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3363535Z    |     help: store this in the heap by calling `Box::new`: `Box::new(42)`
2020-04-20T17:50:58.3363808Z    |
2020-04-20T17:50:58.3364274Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3364621Z                 found type `{integer}`
2020-04-20T17:50:58.3365558Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3366210Z error[E0308]: mismatched types
2020-04-20T17:50:58.3366749Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:46:9
2020-04-20T17:50:58.3367013Z    |
2020-04-20T17:50:58.3367344Z LL | fn baz() -> Box<dyn Trait> {
2020-04-20T17:50:58.3367344Z LL | fn baz() -> Box<dyn Trait> {
2020-04-20T17:50:58.3367922Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3368453Z LL |         Struct //~ ERROR mismatched types
2020-04-20T17:50:58.3368673Z    |         ^^^^^^
2020-04-20T17:50:58.3368831Z    |         |
2020-04-20T17:50:58.3369087Z    |         expected struct `std::boxed::Box`, found struct `Struct`
2020-04-20T17:50:58.3369087Z    |         expected struct `std::boxed::Box`, found struct `Struct`
2020-04-20T17:50:58.3369493Z    |         help: store this in the heap by calling `Box::new`: `Box::new(Struct)`
2020-04-20T17:50:58.3369766Z    |
2020-04-20T17:50:58.3370198Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3370492Z               found struct `Struct`
2020-04-20T17:50:58.3371391Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3372052Z error[E0308]: mismatched types
2020-04-20T17:50:58.3372569Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:48:9
2020-04-20T17:50:58.3372836Z    |
2020-04-20T17:50:58.3373181Z LL | fn baz() -> Box<dyn Trait> {
2020-04-20T17:50:58.3373181Z LL | fn baz() -> Box<dyn Trait> {
2020-04-20T17:50:58.3373749Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3374224Z LL |         42 //~ ERROR mismatched types
2020-04-20T17:50:58.3374431Z    |         ^^
2020-04-20T17:50:58.3374584Z    |         |
2020-04-20T17:50:58.3374823Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3374823Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3375207Z    |         help: store this in the heap by calling `Box::new`: `Box::new(42)`
2020-04-20T17:50:58.3375471Z    |
2020-04-20T17:50:58.3375903Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3376204Z                 found type `{integer}`
2020-04-20T17:50:58.3377110Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3377787Z error[E0308]: mismatched types
2020-04-20T17:50:58.3378313Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:53:9
2020-04-20T17:50:58.3378576Z    |
2020-04-20T17:50:58.3378921Z LL | fn baw() -> Box<dyn Trait> {
2020-04-20T17:50:58.3378921Z LL | fn baw() -> Box<dyn Trait> {
2020-04-20T17:50:58.3379485Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3380011Z LL |         0 //~ ERROR mismatched types
2020-04-20T17:50:58.3380204Z    |         ^
2020-04-20T17:50:58.3380354Z    |         |
2020-04-20T17:50:58.3380607Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3380607Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3380975Z    |         help: store this in the heap by calling `Box::new`: `Box::new(0)`
2020-04-20T17:50:58.3381244Z    |
2020-04-20T17:50:58.3381759Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3382043Z                 found type `{integer}`
2020-04-20T17:50:58.3383005Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3383672Z error[E0308]: mismatched types
2020-04-20T17:50:58.3384199Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:55:9
2020-04-20T17:50:58.3384457Z    |
2020-04-20T17:50:58.3384801Z LL | fn baw() -> Box<dyn Trait> {
2020-04-20T17:50:58.3384801Z LL | fn baw() -> Box<dyn Trait> {
2020-04-20T17:50:58.3385367Z    |             -------------- expected `std::boxed::Box<(dyn Trait + 'static)>` because of return type
2020-04-20T17:50:58.3385849Z LL |         42 //~ ERROR mismatched types
2020-04-20T17:50:58.3386041Z    |         ^^
2020-04-20T17:50:58.3386197Z    |         |
2020-04-20T17:50:58.3386451Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3386451Z    |         expected struct `std::boxed::Box`, found integer
2020-04-20T17:50:58.3386822Z    |         help: store this in the heap by calling `Box::new`: `Box::new(42)`
2020-04-20T17:50:58.3387085Z    |
2020-04-20T17:50:58.3387536Z    = note: expected struct `std::boxed::Box<(dyn Trait + 'static)>`
2020-04-20T17:50:58.3387820Z                 found type `{integer}`
2020-04-20T17:50:58.3388729Z    = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
2020-04-20T17:50:58.3389434Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3389997Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:60:13
2020-04-20T17:50:58.3390277Z    |
2020-04-20T17:50:58.3390277Z    |
2020-04-20T17:50:58.3390635Z LL | fn bat() -> dyn Trait { //~ ERROR E0746
2020-04-20T17:50:58.3391348Z    |
2020-04-20T17:50:58.3391628Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3391910Z    |
2020-04-20T17:50:58.3391910Z    |
2020-04-20T17:50:58.3392265Z LL | fn bat() -> T { //~ ERROR E0746
2020-04-20T17:50:58.3392799Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3393125Z    |
2020-04-20T17:50:58.3393125Z    |
2020-04-20T17:50:58.3393488Z LL | fn bat() -> impl Trait { //~ ERROR E0746
2020-04-20T17:50:58.3393983Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3394208Z    |
2020-04-20T17:50:58.3394208Z    |
2020-04-20T17:50:58.3394582Z LL | fn bat() -> Box<dyn Trait> { //~ ERROR E0746
2020-04-20T17:50:58.3394968Z 
2020-04-20T17:50:58.3395173Z error[E0746]: return type cannot have an unboxed trait object
2020-04-20T17:50:58.3395744Z   --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:66:13
2020-04-20T17:50:58.3396019Z    |
2020-04-20T17:50:58.3396019Z    |
2020-04-20T17:50:58.3396385Z LL | fn bay() -> dyn Trait { //~ ERROR E0746
2020-04-20T17:50:58.3397090Z    |
2020-04-20T17:50:58.3397369Z help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
2020-04-20T17:50:58.3397647Z    |
2020-04-20T17:50:58.3397647Z    |
2020-04-20T17:50:58.3398001Z LL | fn bay() -> T { //~ ERROR E0746
2020-04-20T17:50:58.3398528Z help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
2020-04-20T17:50:58.3398854Z    |
2020-04-20T17:50:58.3398854Z    |
2020-04-20T17:50:58.3399216Z LL | fn bay() -> impl Trait { //~ ERROR E0746
2020-04-20T17:50:58.3399768Z help: use a boxed trait object if all return paths implement trait `Trait`
2020-04-20T17:50:58.3400007Z    |
2020-04-20T17:50:58.3400007Z    |
2020-04-20T17:50:58.3400389Z LL | fn bay() -> Box<dyn Trait> { //~ ERROR E0746
2020-04-20T17:50:58.3400821Z 
2020-04-20T17:50:58.3400995Z error: aborting due to 20 previous errors
2020-04-20T17:50:58.3401148Z 
2020-04-20T17:50:58.3401361Z Some errors have detailed explanations: E0277, E0308, E0746.
---
2020-04-20T17:50:58.3404944Z 
2020-04-20T17:50:58.3405417Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-20T17:50:58.3405646Z 
2020-04-20T17:50:58.3405735Z 
2020-04-20T17:50:58.3409097Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-20T17:50:58.3411356Z 
2020-04-20T17:50:58.3411454Z 
2020-04-20T17:50:58.3411953Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-20T17:50:58.3412281Z Build completed unsuccessfully in 0:59:03
2020-04-20T17:50:58.3412281Z Build completed unsuccessfully in 0:59:03
2020-04-20T17:50:58.3412592Z == clock drift check ==
2020-04-20T17:50:58.3412821Z   local time: Mon Apr 20 17:50:58 UTC 2020
2020-04-20T17:50:58.6580977Z   network time: Mon, 20 Apr 2020 17:50:58 GMT
2020-04-20T17:50:59.3783447Z 
2020-04-20T17:50:59.3783447Z 
2020-04-20T17:50:59.3861892Z ##[error]Bash exited with code '1'.
2020-04-20T17:50:59.3911060Z ##[section]Finishing: Run build
2020-04-20T17:50:59.3988222Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-20T17:50:59.3993207Z Task         : Get sources
2020-04-20T17:50:59.3993538Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T17:50:59.3993820Z Version      : 1.0.0
2020-04-20T17:50:59.3994022Z Author       : Microsoft
2020-04-20T17:50:59.3994022Z Author       : Microsoft
2020-04-20T17:50:59.3994363Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T17:50:59.3994718Z ==============================================================================
2020-04-20T17:50:59.7429594Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T17:50:59.7478766Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-20T17:50:59.7586475Z Cleaning up task key
2020-04-20T17:50:59.7587826Z Start cleaning up orphan processes.
2020-04-20T17:50:59.7782237Z Terminate orphan process: pid (3625) (python)
2020-04-20T17:50:59.7947381Z ##[section]Finishing: Finalize Job
