plain
2020-04-21T12:12:10.9390348Z ========================== Starting Command Output ===========================
2020-04-21T12:12:10.9393338Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a52fa420-7dc0-43c2-bead-1da18cc6a49b.sh
2020-04-21T12:12:10.9393604Z 
2020-04-21T12:12:10.9399614Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T12:12:10.9420673Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-04-21T12:12:10.9424670Z Task         : Get sources
2020-04-21T12:12:10.9424972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T12:12:10.9425262Z Version      : 1.0.0
2020-04-21T12:12:10.9425461Z Author       : Microsoft
---
2020-04-21T12:12:11.9328716Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T12:12:11.9334672Z ##[command]git config gc.auto 0
2020-04-21T12:12:11.9338729Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T12:12:11.9342412Z ##[command]git config --get-all http.proxy
2020-04-21T12:12:11.9349624Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70175/merge:refs/remotes/pull/70175/merge
---
2020-04-21T12:14:23.2436752Z  ---> 318032b5f0e2
2020-04-21T12:14:23.2437641Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-21T12:14:23.2438321Z  ---> Using cache
2020-04-21T12:14:23.2438698Z  ---> d44a858fd1ce
2020-04-21T12:14:23.2439779Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-21T12:14:23.2440950Z  ---> 58b910f50f5a
2020-04-21T12:14:23.2441206Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-21T12:14:23.2447631Z  ---> Using cache
2020-04-21T12:14:23.2450449Z  ---> ee7702aadba1
---
2020-04-21T12:14:23.2891574Z Looks like docker image is the same as before, not uploading
2020-04-21T12:14:30.1126512Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T12:14:30.1424538Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T12:14:30.1460048Z == clock drift check ==
2020-04-21T12:14:30.1470602Z   local time: Tue Apr 21 12:14:30 UTC 2020
2020-04-21T12:14:30.2131608Z   network time: Tue, 21 Apr 2020 12:14:30 GMT
2020-04-21T12:14:30.2165719Z Starting sccache server...
2020-04-21T12:14:30.3093482Z configure: processing command line
2020-04-21T12:14:30.3093954Z configure: 
2020-04-21T12:14:30.3094987Z configure: rust.dist-src        := False
---
2020-04-21T12:19:42.8562288Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T12:19:44.4864254Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T12:19:46.0709520Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T12:19:48.0221040Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T12:19:56.6414994Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T12:19:59.3728292Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T12:20:03.9429069Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T12:20:08.1575554Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T12:20:17.4029271Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T12:42:44.2128168Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T12:42:45.8462837Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T12:42:47.7935426Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T12:42:48.4225624Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T12:42:58.7929207Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T12:43:01.4424103Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T12:43:06.4303132Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T12:43:10.9053172Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T12:43:20.8142232Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T13:06:02.7662853Z .................................................................................................... 1600/9912
2020-04-21T13:06:08.8355378Z .................................................................................................... 1700/9912
2020-04-21T13:06:12.9791550Z .................................................................................................... 1800/9912
2020-04-21T13:06:21.2428780Z .................................................................................................... 1900/9912
2020-04-21T13:06:28.9350204Z ..i................................................................................................. 2000/9912
2020-04-21T13:06:34.9435747Z ............................................................................................iiiii... 2100/9912
2020-04-21T13:06:54.5241549Z .................................................................................................... 2300/9912
2020-04-21T13:06:57.1058537Z .................................................................................................... 2400/9912
2020-04-21T13:06:59.5521082Z .................................................................................................... 2500/9912
2020-04-21T13:07:05.4478777Z .................................................................................................... 2600/9912
---
2020-04-21T13:10:00.1607831Z .................................................................................................... 5100/9912
2020-04-21T13:10:06.9543816Z .................................................................................................... 5200/9912
2020-04-21T13:10:11.6858704Z ...............i.................................................................................... 5300/9912
2020-04-21T13:10:20.9349943Z .....i.............................................................................................. 5400/9912
2020-04-21T13:10:25.8126665Z .....ii.ii........i...i............................................................................. 5500/9912
2020-04-21T13:10:33.0415534Z ....................................................i............................................... 5700/9912
2020-04-21T13:10:41.5309201Z ....................................................................................ii.............. 5800/9912
2020-04-21T13:10:47.9434021Z .......................i............................................................................ 5900/9912
2020-04-21T13:10:53.2189310Z .................................................................................................... 6000/9912
2020-04-21T13:10:53.2189310Z .................................................................................................... 6000/9912
2020-04-21T13:11:03.3159931Z .................................................................................................... 6100/9912
2020-04-21T13:11:12.9640502Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-21T13:11:28.1715917Z .................................................................................................... 6400/9912
2020-04-21T13:11:34.6611376Z .................................................................................................... 6500/9912
2020-04-21T13:11:34.6611376Z .................................................................................................... 6500/9912
2020-04-21T13:11:48.2926732Z ...............................................i..ii................................................ 6600/9912
2020-04-21T13:12:10.6646071Z .................................................................................................... 6800/9912
2020-04-21T13:12:12.5724792Z ................................................i................................................... 6900/9912
2020-04-21T13:12:14.5487161Z .................................................................................................... 7000/9912
2020-04-21T13:12:16.4512025Z ........................................................................................i........... 7100/9912
---
2020-04-21T13:13:49.6140290Z .................................................................................................... 7900/9912
2020-04-21T13:13:55.6623716Z .................................................................................................... 8000/9912
2020-04-21T13:14:01.0195551Z ......................................................i............................................. 8100/9912
2020-04-21T13:14:10.2778451Z .................................................................................................... 8200/9912
2020-04-21T13:14:15.9367769Z ....iiiiiiiiiii.i................................................................................... 8300/9912
2020-04-21T13:14:28.7091599Z .................................................................................................... 8500/9912
2020-04-21T13:14:36.4103206Z .................................................................................................... 8600/9912
2020-04-21T13:14:49.3500790Z .................................................................................................... 8700/9912
2020-04-21T13:14:55.8226572Z .................................................................................................... 8800/9912
---
2020-04-21T13:16:59.7270239Z running 90 tests
2020-04-21T13:17:07.6044426Z ........................................F.................................................
2020-04-21T13:17:07.6046328Z failures:
2020-04-21T13:17:07.6046442Z 
2020-04-21T13:17:07.6046928Z ---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
2020-04-21T13:17:07.6047257Z 1 // MIR for `main::{{closure}}#0` 0 generator_resume
2020-04-21T13:17:07.6047935Z 2 // generator_layout = GeneratorLayout { field_tys: [HasDrop], variant_fields: [[], [], [], [_0]], storage_conflicts: BitMatrix { num_rows: 1, num_columns: 1, words: [1], marker: PhantomData } }
2020-04-21T13:17:07.6048650Z 3 
2020-04-21T13:17:07.6049394Z - fn main::{{closure}}#0(_1: std::pin::Pin<&mut [generator@$DIR/generator-tiny.rs:18:16: 24:6 {u8, HasDrop, ()}]>, _2: u8) -> std::ops::GeneratorState<(), ()> {
2020-04-21T13:17:07.6051359Z -     debug _x => _10;                     // in scope 0 at $DIR/generator-tiny.rs:18:17: 18:19
2020-04-21T13:17:07.6052378Z -     let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6053107Z -     let _3: HasDrop;                     // in scope 0 at $DIR/generator-tiny.rs:19:13: 19:15
2020-04-21T13:17:07.6053884Z -     let mut _4: !;                       // in scope 0 at $DIR/generator-tiny.rs:20:9: 23:10
2020-04-21T13:17:07.6054554Z -     let mut _5: ();                      // in scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6055207Z -     let _6: u8;                          // in scope 0 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6055860Z -     let mut _7: ();                      // in scope 0 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6056530Z -     let _8: ();                          // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:21
2020-04-21T13:17:07.6057180Z -     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:18:25: 18:25
2020-04-21T13:17:07.6057965Z -     let _10: u8;                         // in scope 0 at $DIR/generator-tiny.rs:18:17: 18:19
2020-04-21T13:17:07.6058655Z -     let mut _11: isize;                  // in scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6059493Z + fn main::{{closure}}#0(_1: std::pin::Pin<&mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}]>, _2: u8) -> std::ops::GeneratorState<(), ()> {
2020-04-21T13:17:07.6060281Z +     debug _x => _10;                     // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
2020-04-21T13:17:07.6060984Z +     let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6061677Z +     let _3: HasDrop;                     // in scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
2020-04-21T13:17:07.6062344Z +     let mut _4: !;                       // in scope 0 at $DIR/generator-tiny.rs:21:9: 24:10
2020-04-21T13:17:07.6063003Z +     let mut _5: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6063658Z +     let _6: u8;                          // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6064331Z +     let mut _7: ();                      // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6064982Z +     let _8: ();                          // in scope 0 at $DIR/generator-tiny.rs:23:13: 23:21
2020-04-21T13:17:07.6065648Z +     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
2020-04-21T13:17:07.6066304Z +     let _10: u8;                         // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
2020-04-21T13:17:07.6067196Z +     let mut _11: isize;                  // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6067627Z 16     scope 1 {
2020-04-21T13:17:07.6068415Z -         debug _d => (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:18:16: 24:6 {u8, HasDrop, ()}])) as variant#3).0: HasDrop); // in scope 1 at $DIR/generator-tiny.rs:19:13: 19:15
2020-04-21T13:17:07.6069513Z +         debug _d => (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}])) as variant#3).0: HasDrop); // in scope 1 at $DIR/generator-tiny.rs:20:13: 20:15
2020-04-21T13:17:07.6070104Z 19 
2020-04-21T13:17:07.6070245Z 20     bb0: {
2020-04-21T13:17:07.6070381Z 
2020-04-21T13:17:07.6070381Z 
2020-04-21T13:17:07.6071149Z -         _11 = discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:18:16: 24:6 {u8, HasDrop, ()}]))); // bb0[0]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6072153Z -         switchInt(move _11) -> [0u32: bb1, 3u32: bb5, otherwise: bb6]; // bb0[1]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6073303Z +         _11 = discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}]))); // bb0[0]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6074392Z +         switchInt(move _11) -> [0u32: bb1, 3u32: bb5, otherwise: bb6]; // bb0[1]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6074923Z 24 
2020-04-21T13:17:07.6075060Z 25     bb1: {
2020-04-21T13:17:07.6075174Z 
2020-04-21T13:17:07.6075174Z 
2020-04-21T13:17:07.6075729Z -         _10 = move _2;                   // bb1[0]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6076590Z -         nop;                             // bb1[1]: scope 0 at $DIR/generator-tiny.rs:19:13: 19:15
2020-04-21T13:17:07.6077524Z -         (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:18:16: 24:6 {u8, HasDrop, ()}])) as variant#3).0: HasDrop) = HasDrop; // bb1[2]: scope 0 at $DIR/generator-tiny.rs:19:18: 19:25
2020-04-21T13:17:07.6078384Z -         StorageLive(_4);                 // bb1[3]: scope 1 at $DIR/generator-tiny.rs:20:9: 23:10
2020-04-21T13:17:07.6079074Z -         goto -> bb2;                     // bb1[4]: scope 1 at $DIR/generator-tiny.rs:20:9: 23:10
2020-04-21T13:17:07.6079782Z +         _10 = move _2;                   // bb1[0]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6080572Z +         nop;                             // bb1[1]: scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
2020-04-21T13:17:07.6081538Z +         (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}])) as variant#3).0: HasDrop) = HasDrop; // bb1[2]: scope 0 at $DIR/generator-tiny.rs:20:18: 20:25
2020-04-21T13:17:07.6082399Z +         StorageLive(_4);                 // bb1[3]: scope 1 at $DIR/generator-tiny.rs:21:9: 24:10
2020-04-21T13:17:07.6083099Z +         goto -> bb2;                     // bb1[4]: scope 1 at $DIR/generator-tiny.rs:21:9: 24:10
2020-04-21T13:17:07.6083528Z 32 
2020-04-21T13:17:07.6083659Z 33     bb2: {
2020-04-21T13:17:07.6083769Z 
2020-04-21T13:17:07.6083769Z 
2020-04-21T13:17:07.6084288Z -         StorageLive(_6);                 // bb2[0]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6085024Z -         StorageLive(_7);                 // bb2[1]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6085718Z -         _7 = ();                         // bb2[2]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6086508Z -         _0 = std::ops::GeneratorState::<(), ()>::Yielded(move _7); // bb2[3]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6087475Z -         discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:18:16: 24:6 {u8, HasDrop, ()}]))) = 3; // bb2[4]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6088302Z -         return;                          // bb2[5]: scope 1 at $DIR/generator-tiny.rs:21:13: 21:18
2020-04-21T13:17:07.6089021Z +         StorageLive(_6);                 // bb2[0]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6089721Z +         StorageLive(_7);                 // bb2[1]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6090423Z +         _7 = ();                         // bb2[2]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6091250Z +         _0 = std::ops::GeneratorState::<(), ()>::Yielded(move _7); // bb2[3]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6092199Z +         discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6 {u8, HasDrop, ()}]))) = 3; // bb2[4]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6093036Z +         return;                          // bb2[5]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
2020-04-21T13:17:07.6093458Z 41 
2020-04-21T13:17:07.6093586Z 42     bb3: {
2020-04-21T13:17:07.6093711Z 
2020-04-21T13:17:07.6093711Z 
2020-04-21T13:17:07.6094233Z -         StorageDead(_7);                 // bb3[0]: scope 1 at $DIR/generator-tiny.rs:21:17: 21:18
2020-04-21T13:17:07.6094941Z -         StorageDead(_6);                 // bb3[1]: scope 1 at $DIR/generator-tiny.rs:21:18: 21:19
2020-04-21T13:17:07.6095653Z -         StorageLive(_8);                 // bb3[2]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:21
2020-04-21T13:17:07.6096465Z -         _8 = const callee() -> bb4;      // bb3[3]: scope 1 at $DIR/generator-tiny.rs:22:13: 22:21
2020-04-21T13:17:07.6097295Z +         StorageDead(_7);                 // bb3[0]: scope 1 at $DIR/generator-tiny.rs:22:17: 22:18
2020-04-21T13:17:07.6098035Z +         StorageDead(_6);                 // bb3[1]: scope 1 at $DIR/generator-tiny.rs:22:18: 22:19
2020-04-21T13:17:07.6098761Z +         StorageLive(_8);                 // bb3[2]: scope 1 at $DIR/generator-tiny.rs:23:13: 23:21
2020-04-21T13:17:07.6104908Z +         _8 = const callee() -> bb4;      // bb3[3]: scope 1 at $DIR/generator-tiny.rs:23:13: 23:21
2020-04-21T13:17:07.6105352Z 47                                          // ty::Const
2020-04-21T13:17:07.6105656Z 48                                          // + ty: fn() {callee}
2020-04-21T13:17:07.6106135Z 49                                          // + val: Value(Scalar(<ZST>))
2020-04-21T13:17:07.6106760Z 50                                          // mir::Constant
2020-04-21T13:17:07.6106760Z 50                                          // mir::Constant
2020-04-21T13:17:07.6107529Z -                                          // + span: $DIR/generator-tiny.rs:22:13: 22:19
2020-04-21T13:17:07.6108257Z +                                          // + span: $DIR/generator-tiny.rs:23:13: 23:19
2020-04-21T13:17:07.6108735Z 52                                          // + literal: Const { ty: fn() {callee}, val: Value(Scalar(<ZST>)) }
2020-04-21T13:17:07.6109317Z 54 
2020-04-21T13:17:07.6109406Z 
2020-04-21T13:17:07.6109536Z 55     bb4: {
2020-04-21T13:17:07.6109536Z 55     bb4: {
2020-04-21T13:17:07.6110355Z -         StorageDead(_8);                 // bb4[0]: scope 1 at $DIR/generator-tiny.rs:22:21: 22:22
2020-04-21T13:17:07.6111115Z -         _5 = const ();                   // bb4[1]: scope 1 at $DIR/generator-tiny.rs:20:14: 23:10
2020-04-21T13:17:07.6111842Z +         StorageDead(_8);                 // bb4[0]: scope 1 at $DIR/generator-tiny.rs:23:21: 23:22
2020-04-21T13:17:07.6112712Z +         _5 = const ();                   // bb4[1]: scope 1 at $DIR/generator-tiny.rs:21:14: 24:10
2020-04-21T13:17:07.6113099Z 58                                          // ty::Const
2020-04-21T13:17:07.6113362Z 59                                          // + ty: ()
2020-04-21T13:17:07.6113678Z 60                                          // + val: Value(Scalar(<ZST>))
2020-04-21T13:17:07.6114076Z 61                                          // mir::Constant
2020-04-21T13:17:07.6114076Z 61                                          // mir::Constant
2020-04-21T13:17:07.6114777Z -                                          // + span: $DIR/generator-tiny.rs:20:14: 23:10
2020-04-21T13:17:07.6115634Z +                                          // + span: $DIR/generator-tiny.rs:21:14: 24:10
2020-04-21T13:17:07.6116112Z 63                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-21T13:17:07.6116861Z -         goto -> bb2;                     // bb4[2]: scope 1 at $DIR/generator-tiny.rs:20:9: 23:10
2020-04-21T13:17:07.6121255Z +         goto -> bb2;                     // bb4[2]: scope 1 at $DIR/generator-tiny.rs:21:9: 24:10
2020-04-21T13:17:07.6121751Z 66 
2020-04-21T13:17:07.6121883Z 67     bb5: {
2020-04-21T13:17:07.6121998Z 
2020-04-21T13:17:07.6121998Z 
2020-04-21T13:17:07.6128994Z -         StorageLive(_4);                 // bb5[0]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6130129Z -         StorageLive(_6);                 // bb5[1]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6130878Z -         StorageLive(_7);                 // bb5[2]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6131608Z -         _6 = move _2;                    // bb5[3]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6132355Z -         goto -> bb3;                     // bb5[4]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6133075Z +         StorageLive(_4);                 // bb5[0]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6134030Z +         StorageLive(_6);                 // bb5[1]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6134770Z +         StorageLive(_7);                 // bb5[2]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6135606Z +         _6 = move _2;                    // bb5[3]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6136324Z +         goto -> bb3;                     // bb5[4]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6136877Z 74 
2020-04-21T13:17:07.6137030Z 75     bb6: {
2020-04-21T13:17:07.6137148Z 
2020-04-21T13:17:07.6137148Z 
2020-04-21T13:17:07.6137690Z -         unreachable;                     // bb6[0]: scope 0 at $DIR/generator-tiny.rs:18:16: 24:6
2020-04-21T13:17:07.6138417Z +         unreachable;                     // bb6[0]: scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
2020-04-21T13:17:07.6138868Z 78 }
2020-04-21T13:17:07.6138989Z 79 
2020-04-21T13:17:07.6139099Z 
2020-04-21T13:17:07.6139099Z 
2020-04-21T13:17:07.6140122Z thread '[mir-opt] mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator-tiny/rustc.main-{{closure}}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-21T13:17:07.6141055Z 
2020-04-21T13:17:07.6141150Z 
2020-04-21T13:17:07.6141272Z failures:
2020-04-21T13:17:07.6141663Z     [mir-opt] mir-opt/generator-tiny.rs
2020-04-21T13:17:07.6141663Z     [mir-opt] mir-opt/generator-tiny.rs
2020-04-21T13:17:07.6141828Z 
2020-04-21T13:17:07.6142300Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T13:17:07.6142529Z 
2020-04-21T13:17:07.6142618Z 
2020-04-21T13:17:07.6142724Z 
2020-04-21T13:17:07.6146850Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-21T13:17:07.6149210Z 
2020-04-21T13:17:07.6149299Z 
2020-04-21T13:17:07.6149952Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-21T13:17:07.6151181Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-21T13:17:07.6151181Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-21T13:17:07.6151534Z Build completed unsuccessfully in 1:01:04
2020-04-21T13:17:07.6151760Z == clock drift check ==
2020-04-21T13:17:07.6152002Z   local time: Tue Apr 21 13:17:07 UTC 2020
2020-04-21T13:17:07.6890830Z   network time: Tue, 21 Apr 2020 13:17:07 GMT
2020-04-21T13:17:09.8806294Z 
2020-04-21T13:17:09.8806294Z 
2020-04-21T13:17:09.8899639Z ##[error]Bash exited with code '1'.
2020-04-21T13:17:09.8917798Z ##[section]Finishing: Run build
2020-04-21T13:17:09.8974065Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-04-21T13:17:09.8980163Z Task         : Get sources
2020-04-21T13:17:09.8980541Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T13:17:09.8980864Z Version      : 1.0.0
2020-04-21T13:17:09.8981093Z Author       : Microsoft
2020-04-21T13:17:09.8981093Z Author       : Microsoft
2020-04-21T13:17:09.8981493Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T13:17:09.8981904Z ==============================================================================
2020-04-21T13:17:10.2287211Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T13:17:10.2336083Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-04-21T13:17:10.2426946Z Cleaning up task key
2020-04-21T13:17:10.2428167Z Start cleaning up orphan processes.
2020-04-21T13:17:10.2656132Z Terminate orphan process: pid (3624) (python)
2020-04-21T13:17:10.2851460Z ##[section]Finishing: Finalize Job
