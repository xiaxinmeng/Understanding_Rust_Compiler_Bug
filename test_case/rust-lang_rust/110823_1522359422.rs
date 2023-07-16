plain
.............ii.iiii........ii........i.................................

failures:

---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
4         _0: GeneratorSavedTy {
5             ty: impl std::future::Future<Output = ()>,
6             source_info: SourceInfo {
-                 span: $DIR/async_await.rs:15:8: 15:14 (#8),
+                 span: $DIR/async_await.rs:15:9: 15:14 (#8),
8                 scope: scope[0],
10             ignore_for_traits: false,


12         _1: GeneratorSavedTy {
13             ty: impl std::future::Future<Output = ()>,
14             source_info: SourceInfo {
-                 span: $DIR/async_await.rs:16:8: 16:14 (#11),
+                 span: $DIR/async_await.rs:16:9: 16:14 (#11),
16                 scope: scope[0],
18             ignore_for_traits: false,


35     debug _task_context => _38;          // in scope 0 at $DIR/async_await.rs:+0:18: +3:2
36     let mut _0: std::task::Poll<()>;     // return place in scope 0 at $DIR/async_await.rs:+0:18: +3:2
37     let _3: ();                          // in scope 0 at $DIR/async_await.rs:+1:5: +1:14
-     let mut _4: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
+     let mut _4: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
39     let mut _5: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:5: +1:8
-     let mut _6: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
+     let mut _6: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
41     let mut _7: ();                      // in scope 0 at $DIR/async_await.rs:+0:18: +3:2
-     let _8: ();                          // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _9: std::task::Poll<()>;     // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _10: std::pin::Pin<&mut impl std::future::Future<Output = ()>>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _11: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _12: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
+     let _8: ();                          // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _9: std::task::Poll<()>;     // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _10: std::pin::Pin<&mut impl std::future::Future<Output = ()>>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _11: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _12: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
47     let mut _13: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:5: +1:14
48     let mut _14: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:5: +1:14
-     let mut _15: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _16: isize;                  // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
+     let mut _15: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _16: isize;                  // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
51     let mut _18: !;                      // in scope 0 at $DIR/async_await.rs:+1:5: +1:14
-     let mut _19: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _20: ();                     // in scope 0 at $DIR/async_await.rs:+1:8: +1:14
-     let mut _21: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
+     let mut _19: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _20: ();                     // in scope 0 at $DIR/async_await.rs:+1:9: +1:14
+     let mut _21: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
55     let mut _22: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:5: +2:8
-     let mut _23: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let _24: ();                         // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _25: std::task::Poll<()>;    // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _26: std::pin::Pin<&mut impl std::future::Future<Output = ()>>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _27: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _28: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
+     let mut _23: impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let _24: ();                         // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _25: std::task::Poll<()>;    // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _26: std::pin::Pin<&mut impl std::future::Future<Output = ()>>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _27: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _28: &mut impl std::future::Future<Output = ()>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
62     let mut _29: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:5: +2:14
63     let mut _30: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:5: +2:14
-     let mut _31: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _32: isize;                  // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
+     let mut _31: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _32: isize;                  // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
66     let mut _34: !;                      // in scope 0 at $DIR/async_await.rs:+2:5: +2:14
-     let mut _35: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
-     let mut _36: ();                     // in scope 0 at $DIR/async_await.rs:+2:8: +2:14
+     let mut _35: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
+     let mut _36: ();                     // in scope 0 at $DIR/async_await.rs:+2:9: +2:14
69     let mut _37: ();                     // in scope 0 at $DIR/async_await.rs:+0:18: +3:2
70     let mut _38: &mut std::task::Context<'_>; // in scope 0 at $DIR/async_await.rs:+0:18: +3:2
71     let mut _39: u32;                    // in scope 0 at $DIR/async_await.rs:+0:18: +3:2
72     scope 1 {
72     scope 1 {
-         debug __awaitee => (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>); // in scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         debug __awaitee => (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>); // in scope 1 at $DIR/async_await.rs:+1:9: +1:14
74         let _17: ();                     // in scope 1 at $DIR/async_await.rs:+1:5: +1:14
75         scope 2 {

79         }
80     }
81     scope 4 {
81     scope 4 {
-         debug __awaitee => (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>); // in scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         debug __awaitee => (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>); // in scope 4 at $DIR/async_await.rs:+2:9: +2:14
83         let _33: ();                     // in scope 4 at $DIR/async_await.rs:+2:5: +2:14
84         scope 5 {

96     bb1: {
96     bb1: {
97         _38 = move _2;                   // scope 0 at $DIR/async_await.rs:+0:18: +3:2
98         StorageLive(_3);                 // scope 0 at $DIR/async_await.rs:+1:5: +1:14
-         StorageLive(_4);                 // scope 0 at $DIR/async_await.rs:+1:8: +1:14
+         StorageLive(_4);                 // scope 0 at $DIR/async_await.rs:+1:9: +1:14
100         StorageLive(_5);                 // scope 0 at $DIR/async_await.rs:+1:5: +1:8
101         _5 = a() -> [return: bb2, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+1:5: +1:8
102                                          // mir::Constant
105     }
106 
107     bb2: {
107     bb2: {
-         _4 = <impl Future<Output = ()> as IntoFuture>::into_future(move _5) -> [return: bb3, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+1:8: +1:14
+         _4 = <impl Future<Output = ()> as IntoFuture>::into_future(move _5) -> [return: bb3, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+1:9: +1:14
109                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:15:8: 15:14
+                                          // + span: $DIR/async_await.rs:15:9: 15:14
111                                          // + literal: Const { ty: fn(impl Future<Output = ()>) -> <impl Future<Output = ()> as IntoFuture>::IntoFuture {<impl Future<Output = ()> as IntoFuture>::into_future}, val: Value(<ZST>) }
113 

114     bb3: {
114     bb3: {
115         StorageDead(_5);                 // scope 0 at $DIR/async_await.rs:+1:13: +1:14
-         nop;                             // scope 0 at $DIR/async_await.rs:+1:8: +1:14
-         (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>) = move _4; // scope 0 at $DIR/async_await.rs:+1:8: +1:14
-         goto -> bb4;                     // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         nop;                             // scope 0 at $DIR/async_await.rs:+1:9: +1:14
+         (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>) = move _4; // scope 0 at $DIR/async_await.rs:+1:9: +1:14
+         goto -> bb4;                     // scope 1 at $DIR/async_await.rs:+1:9: +1:14
120 
121     bb4: {


-         StorageLive(_8);                 // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         StorageLive(_9);                 // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         StorageLive(_10);                // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         StorageLive(_11);                // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         StorageLive(_12);                // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         _12 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>); // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         _11 = &mut (*_12);               // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         _10 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _11) -> [return: bb5, unwind unreachable]; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
+         StorageLive(_8);                 // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         StorageLive(_9);                 // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         StorageLive(_10);                // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         StorageLive(_11);                // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         StorageLive(_12);                // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         _12 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>); // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         _11 = &mut (*_12);               // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         _10 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _11) -> [return: bb5, unwind unreachable]; // scope 2 at $DIR/async_await.rs:+1:9: +1:14
130                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:15:8: 15:14
+                                          // + span: $DIR/async_await.rs:15:9: 15:14
132                                          // + literal: Const { ty: unsafe fn(&mut impl Future<Output = ()>) -> Pin<&mut impl Future<Output = ()>> {Pin::<&mut impl Future<Output = ()>>::new_unchecked}, val: Value(<ZST>) }
134 


136         StorageDead(_11);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
137         StorageLive(_13);                // scope 2 at $DIR/async_await.rs:+1:5: +1:14
138         StorageLive(_14);                // scope 2 at $DIR/async_await.rs:+1:5: +1:14
-         StorageLive(_15);                // scope 2 at $DIR/async_await.rs:+1:8: +1:14
-         _15 = _38;                       // scope 2 at $DIR/async_await.rs:+1:8: +1:14
+         StorageLive(_15);                // scope 2 at $DIR/async_await.rs:+1:9: +1:14
+         _15 = _38;                       // scope 2 at $DIR/async_await.rs:+1:9: +1:14
141         _14 = move _15;                  // scope 2 at $DIR/async_await.rs:+1:5: +1:14
142         goto -> bb6;                     // scope 2 at $DIR/async_await.rs:+1:5: +1:14

145     bb6: {
145     bb6: {
146         _13 = &mut (*_14);               // scope 2 at $DIR/async_await.rs:+1:5: +1:14
147         StorageDead(_15);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         _9 = <impl Future<Output = ()> as Future>::poll(move _10, move _13) -> [return: bb7, unwind unreachable]; // scope 2 at $DIR/async_await.rs:+1:8: +1:14
+         _9 = <impl Future<Output = ()> as Future>::poll(move _10, move _13) -> [return: bb7, unwind unreachable]; // scope 2 at $DIR/async_await.rs:+1:9: +1:14
149                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:15:8: 15:14
+                                          // + span: $DIR/async_await.rs:15:9: 15:14
151                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(Pin<&'a mut impl Future<Output = ()>>, &'b mut Context<'c>) -> Poll<<impl Future<Output = ()> as Future>::Output> {<impl Future<Output = ()> as Future>::poll}, val: Value(<ZST>) }
153 

154     bb7: {
154     bb7: {
155         StorageDead(_13);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
156         StorageDead(_10);                // scope 2 at $DIR/async_await.rs:+1:13: +1:14
-         _16 = discriminant(_9);          // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         switchInt(move _16) -> [0: bb10, 1: bb8, otherwise: bb9]; // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         _16 = discriminant(_9);          // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         switchInt(move _16) -> [0: bb10, 1: bb8, otherwise: bb9]; // scope 1 at $DIR/async_await.rs:+1:9: +1:14
160 
161     bb8: {


-         _8 = const ();                   // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         _8 = const ();                   // scope 1 at $DIR/async_await.rs:+1:9: +1:14
163         StorageDead(_14);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
164         StorageDead(_12);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
165         StorageDead(_9);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14

166         StorageDead(_8);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         StorageLive(_19);                // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         StorageLive(_20);                // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         _20 = ();                        // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         _0 = Poll::<()>::Pending;        // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2]))) = 3; // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         return;                          // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         StorageLive(_19);                // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         StorageLive(_20);                // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         _20 = ();                        // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         _0 = Poll::<()>::Pending;        // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2]))) = 3; // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         return;                          // scope 1 at $DIR/async_await.rs:+1:9: +1:14
174 
175     bb9: {


-         unreachable;                     // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         unreachable;                     // scope 1 at $DIR/async_await.rs:+1:9: +1:14
178 
179     bb10: {

190 
190 
191     bb11: {
192         StorageDead(_20);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         _38 = move _19;                  // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         _38 = move _19;                  // scope 1 at $DIR/async_await.rs:+1:9: +1:14
194         StorageDead(_19);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         _7 = const ();                   // scope 1 at $DIR/async_await.rs:+1:8: +1:14
-         goto -> bb4;                     // scope 1 at $DIR/async_await.rs:+1:8: +1:14
+         _7 = const ();                   // scope 1 at $DIR/async_await.rs:+1:9: +1:14
+         goto -> bb4;                     // scope 1 at $DIR/async_await.rs:+1:9: +1:14
198 
199     bb12: {

204     bb13: {
204     bb13: {
205         StorageDead(_4);                 // scope 0 at $DIR/async_await.rs:+1:14: +1:15
206         StorageDead(_3);                 // scope 0 at $DIR/async_await.rs:+1:14: +1:15
-         StorageLive(_21);                // scope 0 at $DIR/async_await.rs:+2:8: +2:14
+         StorageLive(_21);                // scope 0 at $DIR/async_await.rs:+2:9: +2:14
208         StorageLive(_22);                // scope 0 at $DIR/async_await.rs:+2:5: +2:8
209         _22 = a() -> [return: bb14, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+2:5: +2:8
210                                          // mir::Constant
213     }
214 
215     bb14: {
215     bb14: {
-         _21 = <impl Future<Output = ()> as IntoFuture>::into_future(move _22) -> [return: bb15, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+2:8: +2:14
+         _21 = <impl Future<Output = ()> as IntoFuture>::into_future(move _22) -> [return: bb15, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+2:9: +2:14
217                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:16:8: 16:14
+                                          // + span: $DIR/async_await.rs:16:9: 16:14
219                                          // + literal: Const { ty: fn(impl Future<Output = ()>) -> <impl Future<Output = ()> as IntoFuture>::IntoFuture {<impl Future<Output = ()> as IntoFuture>::into_future}, val: Value(<ZST>) }
221 

222     bb15: {
222     bb15: {
223         StorageDead(_22);                // scope 0 at $DIR/async_await.rs:+2:13: +2:14
-         nop;                             // scope 0 at $DIR/async_await.rs:+2:8: +2:14
-         (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>) = move _21; // scope 0 at $DIR/async_await.rs:+2:8: +2:14
-         goto -> bb16;                    // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         nop;                             // scope 0 at $DIR/async_await.rs:+2:9: +2:14
+         (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>) = move _21; // scope 0 at $DIR/async_await.rs:+2:9: +2:14
+         goto -> bb16;                    // scope 4 at $DIR/async_await.rs:+2:9: +2:14
228 
229     bb16: {


-         StorageLive(_24);                // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         StorageLive(_25);                // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         StorageLive(_26);                // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         StorageLive(_27);                // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         StorageLive(_28);                // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         _28 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>); // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         _27 = &mut (*_28);               // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         _26 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _27) -> [return: bb17, unwind unreachable]; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
+         StorageLive(_24);                // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         StorageLive(_25);                // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         StorageLive(_26);                // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         StorageLive(_27);                // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         StorageLive(_28);                // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         _28 = &mut (((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>); // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         _27 = &mut (*_28);               // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         _26 = Pin::<&mut impl Future<Output = ()>>::new_unchecked(move _27) -> [return: bb17, unwind unreachable]; // scope 5 at $DIR/async_await.rs:+2:9: +2:14
238                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:16:8: 16:14
+                                          // + span: $DIR/async_await.rs:16:9: 16:14
240                                          // + literal: Const { ty: unsafe fn(&mut impl Future<Output = ()>) -> Pin<&mut impl Future<Output = ()>> {Pin::<&mut impl Future<Output = ()>>::new_unchecked}, val: Value(<ZST>) }
242 


244         StorageDead(_27);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
245         StorageLive(_29);                // scope 5 at $DIR/async_await.rs:+2:5: +2:14
246         StorageLive(_30);                // scope 5 at $DIR/async_await.rs:+2:5: +2:14
-         StorageLive(_31);                // scope 5 at $DIR/async_await.rs:+2:8: +2:14
-         _31 = _38;                       // scope 5 at $DIR/async_await.rs:+2:8: +2:14
+         StorageLive(_31);                // scope 5 at $DIR/async_await.rs:+2:9: +2:14
+         _31 = _38;                       // scope 5 at $DIR/async_await.rs:+2:9: +2:14
249         _30 = move _31;                  // scope 5 at $DIR/async_await.rs:+2:5: +2:14
250         goto -> bb18;                    // scope 5 at $DIR/async_await.rs:+2:5: +2:14

253     bb18: {
253     bb18: {
254         _29 = &mut (*_30);               // scope 5 at $DIR/async_await.rs:+2:5: +2:14
255         StorageDead(_31);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         _25 = <impl Future<Output = ()> as Future>::poll(move _26, move _29) -> [return: bb19, unwind unreachable]; // scope 5 at $DIR/async_await.rs:+2:8: +2:14
+         _25 = <impl Future<Output = ()> as Future>::poll(move _26, move _29) -> [return: bb19, unwind unreachable]; // scope 5 at $DIR/async_await.rs:+2:9: +2:14
257                                          // mir::Constant
-                                          // + span: $DIR/async_await.rs:16:8: 16:14
+                                          // + span: $DIR/async_await.rs:16:9: 16:14
259                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(Pin<&'a mut impl Future<Output = ()>>, &'b mut Context<'c>) -> Poll<<impl Future<Output = ()> as Future>::Output> {<impl Future<Output = ()> as Future>::poll}, val: Value(<ZST>) }
261 

262     bb19: {
262     bb19: {
263         StorageDead(_29);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
264         StorageDead(_26);                // scope 5 at $DIR/async_await.rs:+2:13: +2:14
-         _32 = discriminant(_25);         // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         switchInt(move _32) -> [0: bb21, 1: bb20, otherwise: bb9]; // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         _32 = discriminant(_25);         // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         switchInt(move _32) -> [0: bb21, 1: bb20, otherwise: bb9]; // scope 4 at $DIR/async_await.rs:+2:9: +2:14
268 
269     bb20: {


-         _24 = const ();                  // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         _24 = const ();                  // scope 4 at $DIR/async_await.rs:+2:9: +2:14
271         StorageDead(_30);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
272         StorageDead(_28);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
273         StorageDead(_25);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14

274         StorageDead(_24);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         StorageLive(_35);                // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         StorageLive(_36);                // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         _36 = ();                        // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         _0 = Poll::<()>::Pending;        // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2]))) = 4; // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         return;                          // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         StorageLive(_35);                // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         StorageLive(_36);                // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         _36 = ();                        // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         _0 = Poll::<()>::Pending;        // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         discriminant((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2]))) = 4; // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         return;                          // scope 4 at $DIR/async_await.rs:+2:9: +2:14
282 
283     bb21: {

294 
294 
295     bb22: {
296         StorageDead(_36);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         _38 = move _35;                  // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         _38 = move _35;                  // scope 4 at $DIR/async_await.rs:+2:9: +2:14
298         StorageDead(_35);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         _7 = const ();                   // scope 4 at $DIR/async_await.rs:+2:8: +2:14
-         goto -> bb16;                    // scope 4 at $DIR/async_await.rs:+2:8: +2:14
+         _7 = const ();                   // scope 4 at $DIR/async_await.rs:+2:9: +2:14
+         goto -> bb16;                    // scope 4 at $DIR/async_await.rs:+2:9: +2:14
302 
303     bb23: {


thread '[mir-opt] tests/mir-opt/building/async_await.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/async_await.b-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/building/async_await.rs
