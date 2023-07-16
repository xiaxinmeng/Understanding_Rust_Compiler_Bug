plain
.....
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/inline/generator-memcpy.rs stdout ----
28       let mut _26: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 0 at $DIR/generator-memcpy.rs:47:22: 47:24
29       let _27: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 0 at $DIR/generator-memcpy.rs:47:22: 47:24
30       let _28: &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 0 at $DIR/generator-memcpy.rs:47:22: 47:24
- +     let mut _40: usize;                  // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
32       scope 1 {
33           debug f => _1;                   // in scope 1 at $DIR/generator-memcpy.rs:39:9: 39:14
34           let _3: std::pin::Pin<&mut std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>; // in scope 1 at $DIR/generator-memcpy.rs:43:13: 43:14

43                       debug _crashed_poll => _7; // in scope 5 at $DIR/generator-memcpy.rs:45:13: 45:26
45                   scope 6 {
45                   scope 6 {
- +                     scope 8 (inlined ptr::mut_ptr::<impl *mut Context>::as_mut) { // at $DIR/generator-memcpy.rs:45:45: 45:66
- +                         debug self => _13; // in scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                         let mut _31: bool; // in scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                         let mut _32: *mut std::task::Context; // in scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                         let mut _33: &mut std::task::Context; // in scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                         let mut _34: &mut std::task::Context; // in scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+ +                     scope 8 (inlined #[track_caller] Option::<&mut Context>::unwrap) { // at $DIR/generator-memcpy.rs:45:45: 45:75
+ +                         debug self => _12; // in scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+ +                         let mut _31: isize; // in scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+ +                         let mut _32: !;  // in scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
52 +                         scope 9 {
+ +                             debug val => _11; // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
53 +                         }
- +                         scope 10 (inlined ptr::mut_ptr::<impl *mut Context>::is_null) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             debug self => _32; // in scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             let mut _35: *mut u8; // in scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             let mut _36: *mut std::task::Context; // in scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             let mut _37: *mut u8; // in scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             scope 11 (inlined null_mut::<u8>) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _38: *mut (); // in scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                 let mut _39: (); // in scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                 scope 12 (inlined invalid_mut::<()>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                     debug addr => _40; // in scope 12 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                     let mut _41: usize; // in scope 12 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                     scope 13 {
- +                                     }
- +                                 }
- +                                 scope 14 (inlined std::ptr::from_raw_parts_mut::<u8>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                     debug data_address => _38; // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     debug metadata => _39; // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     let mut _42: std::ptr::metadata::PtrRepr<u8>; // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     let mut _43: std::ptr::metadata::PtrComponents<u8>; // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     let mut _44: *const (); // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     let mut _45: *mut (); // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     let mut _46: (); // in scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +                                     scope 15 {
- +                                     }
- +                                 }
- +                             }
- +                             scope 16 (inlined ptr::mut_ptr::<impl *mut u8>::guaranteed_eq) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 debug self => _35; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 debug other => _37; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _47: *const u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _48: *const u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _49: *mut u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _50: *const u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _51: *const u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                 let mut _52: *mut u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                             }
- +                         }
91 +                     }
- +                     scope 17 (inlined #[track_caller] Option::<&mut Context>::unwrap) { // at $DIR/generator-memcpy.rs:45:45: 45:75
- +                         debug self => _12; // in scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         let mut _53: isize; // in scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         let mut _54: !;  // in scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         scope 18 {
- +                             debug val => _11; // in scope 18 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         }
- +                     }
101               }
102           }


106 +                 let mut _30: &mut std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 7 at $SRC_DIR/core/src/pin.rs:LL:COL
108           }
108           }
- +         scope 19 (inlined ArgumentV1::new_pointer::<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>) { // at $DIR/generator-memcpy.rs:47:22: 47:24
- +             debug x => _26;              // in scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +             let mut _55: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +             let mut _56: for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +             scope 20 (inlined ArgumentV1::new::<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>) { // at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 debug x => _55;          // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 debug f => _56;          // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 let mut _57: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 let mut _58: for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 let mut _59: &core::fmt::Opaque; // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 let mut _60: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 20 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +                 scope 21 {
+ +         scope 10 (inlined ArgumentV1::new_pointer::<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>) { // at $DIR/generator-memcpy.rs:47:22: 47:24
+ +             debug x => _26;              // in scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +             let mut _33: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +             let mut _34: for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +             scope 11 (inlined ArgumentV1::new::<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>) { // at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 debug x => _33;          // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 debug f => _34;          // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 let mut _35: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 let mut _36: for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 let mut _37: &core::fmt::Opaque; // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 let mut _38: &&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>; // in scope 11 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +                 scope 12 {
122 +             }
123 +         }


165           StorageLive(_13);                // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:57
166           _13 = _5;                        // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:57
167 -         _12 = ptr::mut_ptr::<impl *mut Context>::as_mut(move _13) -> [return: bb3, unwind: bb10]; // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:66
- +         StorageLive(_31);                // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_32);                // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _32 = _13;                       // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_35);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_36);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _36 = _32;                       // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _35 = move _36 as *mut u8 (Misc); // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_36);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_37);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_38);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageLive(_40);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         _40 = const 0_usize;             // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageLive(_41);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         _41 = _40;                       // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         _38 = transmute::<usize, *mut ()>(move _41) -> [return: bb11, unwind: bb6]; // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+ +         _12 = ptr::mut_ptr::<impl *mut Context>::as_mut(move _13) -> [return: bb2, unwind: bb7]; // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:66
183                                            // mir::Constant
- -                                          // + span: $DIR/generator-memcpy.rs:45:58: 45:64
- -                                          // + literal: Const { ty: unsafe fn(*mut Context) -> Option<&mut Context> {ptr::mut_ptr::<impl *mut Context>::as_mut}, val: Value(<ZST>) }
- +                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(usize) -> *mut () {transmute::<usize, *mut ()>}, val: Value(<ZST>) }
+                                            // + span: $DIR/generator-memcpy.rs:45:58: 45:64
+                                            // + literal: Const { ty: unsafe fn(*mut Context) -> Option<&mut Context> {ptr::mut_ptr::<impl *mut Context>::as_mut}, val: Value(<ZST>) }
189   
190 -     bb3: {


- -         StorageDead(_13);                // scope 6 at $DIR/generator-memcpy.rs:45:65: 45:66
+ +     bb2: {
+           StorageDead(_13);                // scope 6 at $DIR/generator-memcpy.rs:45:65: 45:66
192 -         _11 = Option::<&mut Context>::unwrap(move _12) -> [return: bb4, unwind: bb10]; // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
193 -                                          // mir::Constant
194 -                                          // + span: $DIR/generator-memcpy.rs:45:67: 45:73

195 -                                          // + literal: Const { ty: fn(Option<&mut Context>) -> &mut Context {Option::<&mut Context>::unwrap}, val: Value(<ZST>) }
- -     }
- - 
+ +         StorageLive(_31);                // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
+ +         _31 = discriminant(_12);         // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         switchInt(move _31) -> [0_isize: bb9, 1_isize: bb11, otherwise: bb10]; // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+   
198 -     bb4: {
198 -     bb4: {
199 -         _10 = &mut (*_11);               // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
200 -         StorageDead(_12);                // scope 6 at $DIR/generator-memcpy.rs:45:74: 45:75
206 -     }
207 - 
208 -     bb5: {
- +     bb2: {
- +     bb2: {
+ +     bb3: {
210           StorageDead(_9);                 // scope 4 at $DIR/generator-memcpy.rs:45:77: 45:78
211           StorageDead(_8);                 // scope 4 at $DIR/generator-memcpy.rs:45:77: 45:78
212           StorageDead(_11);                // scope 4 at $DIR/generator-memcpy.rs:45:78: 45:79

242           _27 = &_28;                      // scope 1 at $DIR/generator-memcpy.rs:47:22: 47:24
243           _26 = &(*_27);                   // scope 1 at $DIR/generator-memcpy.rs:47:22: 47:24
244 -         _25 = ArgumentV1::new_pointer::<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>(move _26) -> [return: bb6, unwind: bb10]; // scope 1 at $DIR/generator-memcpy.rs:47:22: 47:24
- +         StorageLive(_55);                // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _55 = _26;                       // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageLive(_56);                // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _56 = <&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Pointer>::fmt as for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> (Pointer(ReifyFnPointer)); // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_33);                // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _33 = _26;                       // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_34);                // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _34 = <&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Pointer>::fmt as for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> (Pointer(ReifyFnPointer)); // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
249                                            // mir::Constant
250 -                                          // + span: $DIR/generator-memcpy.rs:47:22: 47:24
251 -                                          // + user_ty: UserType(3)

263 -         _16 = Arguments::new_v1(move _17, move _21) -> [return: bb7, unwind: bb10]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
264 +                                          // + span: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
265 +                                          // + literal: Const { ty: for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut Formatter<'t0>) -> Result<(), std::fmt::Error> {<&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Pointer>::fmt}, val: Value(<ZST>) }
- +         StorageLive(_57);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageLive(_58);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _58 = _56;                       // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _57 = transmute::<for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut Formatter<'t0>) -> Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> Result<(), std::fmt::Error>>(move _58) -> [return: bb16, unwind: bb6]; // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_35);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_36);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _36 = _34;                       // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _35 = transmute::<for<'r, 's, 't0> fn(&'r &std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &'s mut Formatter<'t0>) -> Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> Result<(), std::fmt::Error>>(move _36) -> [return: bb12, unwind: bb7]; // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
270                                            // mir::Constant
271 -                                          // + span: $SRC_DIR/std/src/macros.rs:LL:COL
272 -                                          // + user_ty: UserType(2)
276       }
277   
278 -     bb7: {
- +     bb3: {
- +     bb3: {
+ +     bb4: {
280           StorageDead(_21);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
281           StorageDead(_17);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
282 -         _15 = _print(move _16) -> [return: bb8, unwind: bb10]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL

- +         _15 = _print(move _16) -> [return: bb4, unwind: bb6]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
+ +         _15 = _print(move _16) -> [return: bb5, unwind: bb7]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
284                                            // mir::Constant
285                                            // + span: $SRC_DIR/std/src/macros.rs:LL:COL
286                                            // + literal: Const { ty: for<'r> fn(Arguments<'r>) {_print}, val: Value(<ZST>) }
287       }
288   
289 -     bb8: {
- +     bb4: {
- +     bb4: {
+ +     bb5: {
291           StorageDead(_16);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
292           StorageDead(_28);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
293           StorageDead(_27);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL

299           StorageDead(_14);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
300           _0 = const ();                   // scope 0 at $DIR/generator-memcpy.rs:38:15: 48:2
301 -         drop(_1) -> [return: bb9, unwind: bb11]; // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
- +         drop(_1) -> [return: bb5, unwind: bb7]; // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
+ +         drop(_1) -> [return: bb6, unwind: bb8]; // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
304   
305 -     bb9: {

- +     bb5: {
- +     bb5: {
+ +     bb6: {
307           StorageDead(_1);                 // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
308           return;                          // scope 0 at $DIR/generator-memcpy.rs:48:2: 48:2

310   
310   
311 -     bb10 (cleanup): {
312 -         drop(_1) -> bb11;                // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
- +     bb6 (cleanup): {
- +         drop(_1) -> bb7;                 // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
+ +     bb7 (cleanup): {
+ +         drop(_1) -> bb8;                 // scope 0 at $DIR/generator-memcpy.rs:48:1: 48:2
316   
316   
317 -     bb11 (cleanup): {

- +     bb7 (cleanup): {
+ +     bb8 (cleanup): {
319           resume;                          // scope 0 at $DIR/generator-memcpy.rs:38:1: 48:2
321 + 

- +     bb8: {
- +     bb8: {
- +         Deinit(_12);                     // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         discriminant(_12) = 0;           // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         goto -> bb10;                    // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +     }
328 +     bb9: {
328 +     bb9: {
- +         StorageLive(_33);                // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_34);                // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _34 = &mut (*_13);               // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _33 = &mut (*_34);               // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         Deinit(_12);                     // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         ((_12 as Some).0: &mut std::task::Context) = move _33; // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         discriminant(_12) = 1;           // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_33);                // scope 9 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_34);                // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         goto -> bb10;                    // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +     }
- +     bb10: {
- +     bb10: {
- +         StorageDead(_31);                // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_13);                // scope 6 at $DIR/generator-memcpy.rs:45:65: 45:66
- +         StorageLive(_53);                // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
- +         _53 = discriminant(_12);         // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +         switchInt(move _53) -> [0_isize: bb13, 1_isize: bb15, otherwise: bb14]; // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +     }
- +     bb11: {
- +     bb11: {
- +         StorageDead(_41);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageDead(_40);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageLive(_39);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageLive(_42);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageLive(_43);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageLive(_44);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageLive(_45);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         _45 = _38;                       // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         _44 = move _45 as *const () (Pointer(MutToConstPointer)); // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_45);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageLive(_46);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         _46 = _39;                       // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         Deinit(_43);                     // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         (_43.0: *const ()) = move _44;   // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         (_43.1: ()) = move _46;          // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_46);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_44);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         Deinit(_42);                     // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         (_42.2: std::ptr::metadata::PtrComponents<u8>) = move _43; // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_43);                // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         _37 = (_42.1: *mut u8);          // scope 15 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_42);                // scope 14 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
- +         StorageDead(_39);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageDead(_38);                // scope 11 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
- +         StorageLive(_47);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_48);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_49);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _49 = _35;                       // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _48 = move _49 as *const u8 (Pointer(MutToConstPointer)); // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_49);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _47 = _48;                       // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_50);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_51);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageLive(_52);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _52 = _37;                       // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _51 = move _52 as *const u8 (Pointer(MutToConstPointer)); // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_52);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _50 = _51;                       // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         _31 = ptr_guaranteed_eq::<u8>(move _47, move _50) -> [return: bb12, unwind: bb6]; // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+ +         StorageLive(_32);                // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         _32 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value") -> bb7; // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
389 +                                          // mir::Constant
- +                                          // + span: $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +                                          // + literal: Const { ty: extern "rust-intrinsic" fn(*const u8, *const u8) -> bool {ptr_guaranteed_eq::<u8>}, val: Value(<ZST>) }
- +     }
- +     bb12: {
- +     bb12: {
- +         StorageDead(_50);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_47);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_51);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_48);                // scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_37);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_35);                // scope 10 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         StorageDead(_32);                // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +         switchInt(move _31) -> [false: bb9, otherwise: bb8]; // scope 8 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
- +     }
- +     bb13: {
- +     bb13: {
- +         StorageLive(_54);                // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +         _54 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value") -> bb6; // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +                                          // mir::Constant
409 +                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
410 +                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }
411 +                                          // mir::Constant

413 +                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
415 + 
- +     bb14: {
- +     bb14: {
- +         unreachable;                     // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
+ +     bb10: {
+ +         unreachable;                     // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
419 + 
- +     bb15: {
- +     bb15: {
- +         _11 = move ((_12 as Some).0: &mut std::task::Context); // scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
- +         StorageDead(_53);                // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
+ +     bb11: {
+ +         _11 = move ((_12 as Some).0: &mut std::task::Context); // scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         StorageDead(_31);                // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
423 +         _10 = &mut (*_11);               // scope 6 at $DIR/generator-memcpy.rs:45:45: 45:75
424 +         StorageDead(_12);                // scope 6 at $DIR/generator-memcpy.rs:45:74: 45:75
425 +         _9 = &mut (*_10);                // scope 4 at $DIR/generator-memcpy.rs:45:45: 45:75

- +         _7 = <std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Future>::poll(move _8, move _9) -> [return: bb2, unwind: bb6]; // scope 4 at $DIR/generator-memcpy.rs:45:29: 45:78
+ +         _7 = <std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Future>::poll(move _8, move _9) -> [return: bb3, unwind: bb7]; // scope 4 at $DIR/generator-memcpy.rs:45:29: 45:78
427 +                                          // mir::Constant
428 +                                          // + span: $DIR/generator-memcpy.rs:45:31: 45:35
429 +                                          // + literal: Const { ty: for<'r, 's, 't0> fn(Pin<&'r mut std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>>, &'s mut Context<'t0>) -> Poll<<std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Future>::Output> {<std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]> as Future>::poll}, val: Value(<ZST>) }
430 +     }
431 + 
- +     bb16: {
- +     bb16: {
- +         StorageDead(_58);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageLive(_59);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageLive(_60);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _60 = _55;                       // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         _59 = transmute::<&&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &core::fmt::Opaque>(move _60) -> [return: bb17, unwind: bb6]; // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +     bb12: {
+ +         StorageDead(_36);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_37);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageLive(_38);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _38 = _33;                       // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _37 = transmute::<&&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &core::fmt::Opaque>(move _38) -> [return: bb13, unwind: bb7]; // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
438 +                                          // mir::Constant
439 +                                          // + span: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
440 +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>) -> &core::fmt::Opaque {transmute::<&&std::future::from_generator::GenFuture<[static generator@$DIR/generator-memcpy.rs:8:23: 22:2]>, &core::fmt::Opaque>}, val: Value(<ZST>) }
441 +     }
442 + 
- +     bb17: {
- +     bb17: {
- +         StorageDead(_60);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         Deinit(_25);                     // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         (_25.0: &core::fmt::Opaque) = move _59; // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         (_25.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _57; // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageDead(_59);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageDead(_57);                // scope 21 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageDead(_56);                // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
- +         StorageDead(_55);                // scope 19 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +     bb13: {
+ +         StorageDead(_38);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         Deinit(_25);                     // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         (_25.0: &core::fmt::Opaque) = move _37; // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         (_25.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _35; // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageDead(_37);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageDead(_35);                // scope 12 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageDead(_34);                // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         StorageDead(_33);                // scope 10 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
452 +         StorageDead(_26);                // scope 1 at $DIR/generator-memcpy.rs:47:23: 47:24
453 +         _24 = [move _25];                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
454 +         StorageDead(_25);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL

456 +         _22 = &(*_23);                   // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
457 +         _21 = move _22 as &[std::fmt::ArgumentV1] (Pointer(Unsize)); // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
458 +         StorageDead(_22);                // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
- +         _16 = Arguments::new_v1(move _17, move _21) -> [return: bb3, unwind: bb6]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
+ +         _16 = Arguments::new_v1(move _17, move _21) -> [return: bb4, unwind: bb7]; // scope 1 at $SRC_DIR/std/src/macros.rs:LL:COL
460 +                                          // mir::Constant
461 +                                          // + span: $SRC_DIR/std/src/macros.rs:LL:COL
462 +                                          // + user_ty: UserType(2)

thread '[mir-opt] src/test/mir-opt/inline/generator-memcpy.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/generator_memcpy.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3512:25


failures:
    [mir-opt] src/test/mir-opt/inline/generator-memcpy.rs
