plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown

failures:

---- [mir-opt] checkout/tests/mir-opt/building/async_await.rs stdout ----
77     bb0: {
77     bb0: {
78         _39 = discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:13:18: 16:2]))); // scope 0 at $DIR/async_await.rs:+0:18: +3:2
-         switchInt(move _39) -> [0: bb1, 1: bb47, 2: bb46, 3: bb44, 4: bb45, otherwise: bb48]; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
+         switchInt(move _39) -> [0: bb1, 1: bb29, 3: bb27, 4: bb28, otherwise: bb30]; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
81 
82     bb1: {


84         StorageLive(_3);                 // scope 0 at $DIR/async_await.rs:+1:5: +1:14
85         StorageLive(_4);                 // scope 0 at $DIR/async_await.rs:+1:8: +1:14
86         StorageLive(_5);                 // scope 0 at $DIR/async_await.rs:+1:5: +1:8
-         _5 = a() -> [return: bb2, unwind: bb39]; // scope 0 at $DIR/async_await.rs:+1:5: +1:8
+         _5 = a() -> bb2;                 // scope 0 at $DIR/async_await.rs:+1:5: +1:8
88                                          // mir::Constant
89                                          // + span: $DIR/async_await.rs:14:5: 14:6
90                                          // + literal: Const { ty: fn() -> impl Future<Output = ()> {a}, val: Value(<ZST>) }
91     }
92 
93     bb2: {
93     bb2: {
-         _4 = <impl Future<Output = ()> as IntoFuture>::into_future(move _5) -> [return: bb3, unwind: bb38]; // scope 0 at $DIR/async_await.rs:+1:8: +1:14
+         _4 = <impl Future<Output = ()> as IntoFuture>::into_future(move _5) -> bb3; // scope 0 at $DIR/async_await.rs:+1:8: +1:14
95                                          // mir::Constant
96                                          // + span: $DIR/async_await.rs:14:8: 14:14
97                                          // + literal: Const { ty: fn(impl Future<Output = ()>) -> <impl Future<Output = ()> as IntoFuture>::IntoFuture {<impl Future<Output = ()> as IntoFuture>::into_future}, val: Value(<ZST>) }

112         StorageLive(_12);                // scope 2 at $DIR/async_await.rs:+1:8: +1:14
113         _12 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:13:18: 16:2])) as variant#3).0: impl std::future::Future<Output = ()>); // scope 2 at $DIR/async_await.rs:+1:8: +1:14
114         _11 = &mut (*_12);               // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         _10 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _11) -> [return: bb5, unwind: bb35]; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
+         _10 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _11) -> bb5; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
116                                          // mir::Constant
117                                          // + span: $DIR/async_await.rs:14:8: 14:14
118                                          // + literal: Const { ty: unsafe fn(&mut impl Future<Output = ()>) -> Pin<&mut impl Future<Output = ()>> {Pin::<&mut impl Future<Output = ()>>::new_unchecked}, val: Value(<ZST>) }
131     bb6: {
131     bb6: {
132         _13 = &mut (*_14);               // scope 2 at $DIR/async_await.rs:+1:5: +1:14
133         StorageDead(_15);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         _9 = <impl Future<Output = ()> as Future>::poll(move _10, move _13) -> [return: bb7, unwind: bb34]; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
+         _9 = <impl Future<Output = ()> as Future>::poll(move _10, move _13) -> bb7; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
135                                          // mir::Constant
136                                          // + span: $DIR/async_await.rs:14:8: 14:14
137                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(Pin<&'a mut impl Future<Output = ()>>, &'b mut Context<'c>) -> Poll<<impl Future<Output = ()> as Future>::Output> {<impl Future<Output = ()> as Future>::poll}, val: Value(<ZST>) }

193         StorageDead(_3);                 // scope 0 at $DIR/async_await.rs:+1:14: +1:15
194         StorageLive(_21);                // scope 0 at $DIR/async_await.rs:+2:8: +2:14
195         StorageLive(_22);                // scope 0 at $DIR/async_await.rs:+2:5: +2:8
-         _22 = a() -> [return: bb14, unwind: bb32]; // scope 0 at $DIR/async_await.rs:+2:5: +2:8
+         _22 = a() -> bb14;               // scope 0 at $DIR/async_await.rs:+2:5: +2:8
197                                          // mir::Constant
198                                          // + span: $DIR/async_await.rs:15:5: 15:6
199                                          // + literal: Const { ty: fn() -> impl Future<Output = ()> {a}, val: Value(<ZST>) }
200     }
201 
202     bb14: {
202     bb14: {
-         _21 = <impl Future<Output = ()> as IntoFuture>::into_future(move _22) -> [return: bb15, unwind: bb31]; // scope 0 at $DIR/async_await.rs:+2:8: +2:14
+         _21 = <impl Future<Output = ()> as IntoFuture>::into_future(move _22) -> bb15; // scope 0 at $DIR/async_await.rs:+2:8: +2:14
204                                          // mir::Constant
205                                          // + span: $DIR/async_await.rs:15:8: 15:14
206                                          // + literal: Const { ty: fn(impl Future<Output = ()>) -> <impl Future<Output = ()> as IntoFuture>::IntoFuture {<impl Future<Output = ()> as IntoFuture>::into_future}, val: Value(<ZST>) }

221         StorageLive(_28);                // scope 5 at $DIR/async_await.rs:+2:8: +2:14
222         _28 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:13:18: 16:2])) as variant#4).0: impl std::future::Future<Output = ()>); // scope 5 at $DIR/async_await.rs:+2:8: +2:14
223         _27 = &mut (*_28);               // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         _26 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _27) -> [return: bb17, unwind: bb28]; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
+         _26 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _27) -> bb17; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
225                                          // mir::Constant
226                                          // + span: $DIR/async_await.rs:15:8: 15:14
227                                          // + literal: Const { ty: unsafe fn(&mut impl Future<Output = ()>) -> Pin<&mut impl Future<Output = ()>> {Pin::<&mut impl Future<Output = ()>>::new_unchecked}, val: Value(<ZST>) }
240     bb18: {
240     bb18: {
241         _29 = &mut (*_30);               // scope 5 at $DIR/async_await.rs:+2:5: +2:14
242         StorageDead(_31);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         _25 = <impl Future<Output = ()> as Future>::poll(move _26, move _29) -> [return: bb19, unwind: bb27]; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
+         _25 = <impl Future<Output = ()> as Future>::poll(move _26, move _29) -> bb19; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
244                                          // mir::Constant
245                                          // + span: $DIR/async_await.rs:15:8: 15:14
246                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(Pin<&'a mut impl Future<Output = ()>>, &'b mut Context<'c>) -> Poll<<impl Future<Output = ()> as Future>::Output> {<impl Future<Output = ()> as Future>::poll}, val: Value(<ZST>) }

310         return;                          // scope 0 at $DIR/async_await.rs:+3:2: +3:2
312 
312 
-     bb27 (cleanup): {
-         StorageDead(_29);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         StorageDead(_26);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         StorageDead(_30);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         goto -> bb29;                    // scope 4 at no-location
- 
- 
-     bb28 (cleanup): {
-         StorageDead(_27);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         StorageDead(_26);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         goto -> bb29;                    // scope 5 at no-location
- 
- 
-     bb29 (cleanup): {
-         StorageDead(_28);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         StorageDead(_25);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         StorageDead(_24);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         goto -> bb30;                    // scope 0 at $DIR/async_await.rs:+2:13: +2:14
- 
- 
-     bb30 (cleanup): {
-         nop;                             // scope 0 at $DIR/async_await.rs:+2:13: +2:14
-         goto -> bb33;                    // scope 0 at $DIR/async_await.rs:+3:1: +3:2
- 
- 
-     bb31 (cleanup): {
-         goto -> bb32;                    // scope 0 at $DIR/async_await.rs:+2:13: +2:14
- 
- 
-     bb32 (cleanup): {
-         StorageDead(_22);                // scope 0 at $DIR/async_await.rs:+2:13: +2:14
-         goto -> bb33;                    // scope 0 at no-location
- 
- 
-     bb33 (cleanup): {
-         StorageDead(_21);                // scope 0 at $DIR/async_await.rs:+3:1: +3:2
-         goto -> bb41;                    // scope 0 at no-location
- 
- 
-     bb34 (cleanup): {
-         StorageDead(_13);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         StorageDead(_10);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         StorageDead(_14);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         goto -> bb36;                    // scope 1 at no-location
- 
- 
-     bb35 (cleanup): {
-         StorageDead(_11);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         StorageDead(_10);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         goto -> bb36;                    // scope 2 at no-location
- 
- 
-     bb36 (cleanup): {
-         StorageDead(_12);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         StorageDead(_9);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         StorageDead(_8);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         goto -> bb37;                    // scope 0 at $DIR/async_await.rs:+1:13: +1:14
- 
- 
-     bb37 (cleanup): {
-         nop;                             // scope 0 at $DIR/async_await.rs:+1:13: +1:14
-         goto -> bb40;                    // scope 0 at $DIR/async_await.rs:+1:14: +1:15
- 
- 
-     bb38 (cleanup): {
-         goto -> bb39;                    // scope 0 at $DIR/async_await.rs:+1:13: +1:14
- 
- 
-     bb39 (cleanup): {
-         StorageDead(_5);                 // scope 0 at $DIR/async_await.rs:+1:13: +1:14
-         goto -> bb40;                    // scope 0 at no-location
- 
- 
-     bb40 (cleanup): {
-         StorageDead(_4);                 // scope 0 at $DIR/async_await.rs:+1:14: +1:15
-         StorageDead(_3);                 // scope 0 at $DIR/async_await.rs:+1:14: +1:15
-         goto -> bb41;                    // scope 0 at no-location
- 
- 
-     bb41 (cleanup): {
-         goto -> bb42;                    // scope 0 at $DIR/async_await.rs:+3:1: +3:2
- 
- 
-     bb42 (cleanup): {
-         goto -> bb43;                    // scope 0 at $DIR/async_await.rs:+0:18: +3:2
- 
- 
-     bb43 (cleanup): {
-         discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:13:18: 16:2]))) = 2; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
-         resume;                          // scope 0 at $DIR/async_await.rs:+0:18: +3:2
- 
-     bb44: {
+     bb27: {
+     bb27: {
406         StorageLive(_3);                 // scope 0 at $DIR/async_await.rs:+0:18: +3:2
407         StorageLive(_4);                 // scope 0 at $DIR/async_await.rs:+0:18: +3:2
408         StorageLive(_19);                // scope 0 at $DIR/async_await.rs:+0:18: +3:2

411         goto -> bb11;                    // scope 0 at $DIR/async_await.rs:+0:18: +3:2
413 
-     bb45: {
+     bb28: {
+     bb28: {
415         StorageLive(_21);                // scope 0 at $DIR/async_await.rs:+0:18: +3:2
416         StorageLive(_35);                // scope 0 at $DIR/async_await.rs:+0:18: +3:2
417         StorageLive(_36);                // scope 0 at $DIR/async_await.rs:+0:18: +3:2

419         goto -> bb23;                    // scope 0 at $DIR/async_await.rs:+0:18: +3:2
421 
-     bb46: {
-     bb46: {
-         assert(const false, "`async fn` resumed after panicking") -> bb46; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
+     bb29: {
+         assert(const false, "`async fn` resumed after completion") -> bb29; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
425 
-     bb47: {
-     bb47: {
-         assert(const false, "`async fn` resumed after completion") -> bb47; // scope 0 at $DIR/async_await.rs:+0:18: +3:2
- 
-     bb48: {
+     bb30: {
+     bb30: {
431         unreachable;                     // scope 0 at $DIR/async_await.rs:+0:18: +3:2
433 }


thread '[mir-opt] checkout/tests/mir-opt/building/async_await.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/async_await.b-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3463:21


failures:
failures:
    [mir-opt] checkout/tests/mir-opt/building/async_await.rs
test result: FAILED. 184 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 9.13s

Build completed unsuccessfully in 0:13:14
