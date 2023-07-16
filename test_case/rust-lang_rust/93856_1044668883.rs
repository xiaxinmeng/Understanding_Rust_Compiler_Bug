plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 166 tests
...F.......F........................i..............................................i..............F. 100/166
.........................F........i...............................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
37         StorageLive(_4);                 // scope 2 at $DIR/basic_assignment.rs:18:9: 18:15
38         _4 = Option::<Box<u32>>::None;   // scope 2 at $DIR/basic_assignment.rs:18:36: 18:40
39         FakeRead(ForLet(None), _4);      // scope 2 at $DIR/basic_assignment.rs:18:9: 18:15
-         AscribeUserType(_4, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 2 at $DIR/basic_assignment.rs:18:17: 18:33
+         AscribeUserType(_4, UserTypeProjection { base: UserType(1), projs: [] }); // scope 2 at $DIR/basic_assignment.rs:18:17: 18:33
41         StorageLive(_5);                 // scope 3 at $DIR/basic_assignment.rs:19:9: 19:15
42         StorageLive(_6);                 // scope 4 at $DIR/basic_assignment.rs:23:14: 23:20
43         _6 = move _4;                    // scope 4 at $DIR/basic_assignment.rs:23:14: 23:20

thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/basic_assignment.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3375:25

---- [mir-opt] mir-opt/address-of.rs stdout ----
---- [mir-opt] mir-opt/address-of.rs stdout ----
139         StorageLive(_5);                 // scope 2 at $DIR/address-of.rs:7:5: 7:18
140         StorageLive(_6);                 // scope 2 at $DIR/address-of.rs:7:5: 7:18
141         _6 = &raw const (*_1);           // scope 2 at $DIR/address-of.rs:7:5: 7:6
-         AscribeUserType(_6, o, UserTypeProjection { base: UserType(0), projs: [] }); // scope 2 at $DIR/address-of.rs:7:5: 7:18
+         AscribeUserType(_6, UserTypeProjection { base: UserType(0), projs: [] }); // scope 2 at $DIR/address-of.rs:7:5: 7:18
143         _5 = _6;                         // scope 2 at $DIR/address-of.rs:7:5: 7:18
144         StorageDead(_6);                 // scope 2 at $DIR/address-of.rs:7:18: 7:19
145         StorageDead(_5);                 // scope 2 at $DIR/address-of.rs:7:18: 7:19

152         _10 = &raw const (*_1);          // scope 2 at $DIR/address-of.rs:9:5: 9:6
153         _9 = move _10 as *const dyn std::marker::Send (Pointer(Unsize)); // scope 2 at $DIR/address-of.rs:9:5: 9:6
154         StorageDead(_10);                // scope 2 at $DIR/address-of.rs:9:5: 9:6
-         AscribeUserType(_9, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 2 at $DIR/address-of.rs:9:5: 9:25
+         AscribeUserType(_9, UserTypeProjection { base: UserType(1), projs: [] }); // scope 2 at $DIR/address-of.rs:9:5: 9:25
156         _8 = _9;                         // scope 2 at $DIR/address-of.rs:9:5: 9:25
157         StorageDead(_9);                 // scope 2 at $DIR/address-of.rs:9:25: 9:26
158         StorageDead(_8);                 // scope 2 at $DIR/address-of.rs:9:25: 9:26

171         StorageLive(_15);                // scope 2 at $DIR/address-of.rs:13:9: 13:10
172         _15 = &raw const (*_1);          // scope 2 at $DIR/address-of.rs:13:23: 13:24
173         FakeRead(ForLet(None), _15);     // scope 2 at $DIR/address-of.rs:13:9: 13:10
-         AscribeUserType(_15, o, UserTypeProjection { base: UserType(3), projs: [] }); // scope 2 at $DIR/address-of.rs:13:12: 13:20
+         AscribeUserType(_15, UserTypeProjection { base: UserType(3), projs: [] }); // scope 2 at $DIR/address-of.rs:13:12: 13:20
175         StorageLive(_16);                // scope 3 at $DIR/address-of.rs:14:9: 14:10
176         _16 = &raw const (*_1);          // scope 3 at $DIR/address-of.rs:14:31: 14:32
177         FakeRead(ForLet(None), _16);     // scope 3 at $DIR/address-of.rs:14:9: 14:10

-         AscribeUserType(_16, o, UserTypeProjection { base: UserType(5), projs: [] }); // scope 3 at $DIR/address-of.rs:14:12: 14:28
+         AscribeUserType(_16, UserTypeProjection { base: UserType(5), projs: [] }); // scope 3 at $DIR/address-of.rs:14:12: 14:28
179         StorageLive(_17);                // scope 4 at $DIR/address-of.rs:15:9: 15:10
180         StorageLive(_18);                // scope 4 at $DIR/address-of.rs:15:30: 15:31
181         _18 = &raw const (*_1);          // scope 4 at $DIR/address-of.rs:15:30: 15:31

182         _17 = move _18 as *const dyn std::marker::Send (Pointer(Unsize)); // scope 4 at $DIR/address-of.rs:15:30: 15:31
183         StorageDead(_18);                // scope 4 at $DIR/address-of.rs:15:30: 15:31
184         FakeRead(ForLet(None), _17);     // scope 4 at $DIR/address-of.rs:15:9: 15:10
-         AscribeUserType(_17, o, UserTypeProjection { base: UserType(7), projs: [] }); // scope 4 at $DIR/address-of.rs:15:12: 15:27
+         AscribeUserType(_17, UserTypeProjection { base: UserType(7), projs: [] }); // scope 4 at $DIR/address-of.rs:15:12: 15:27
186         StorageLive(_19);                // scope 5 at $DIR/address-of.rs:16:9: 16:10
187         StorageLive(_20);                // scope 5 at $DIR/address-of.rs:16:27: 16:28
188         _20 = &raw const (*_1);          // scope 5 at $DIR/address-of.rs:16:27: 16:28

189         _19 = move _20 as *const [i32] (Pointer(Unsize)); // scope 5 at $DIR/address-of.rs:16:27: 16:28
190         StorageDead(_20);                // scope 5 at $DIR/address-of.rs:16:27: 16:28
191         FakeRead(ForLet(None), _19);     // scope 5 at $DIR/address-of.rs:16:9: 16:10
-         AscribeUserType(_19, o, UserTypeProjection { base: UserType(9), projs: [] }); // scope 5 at $DIR/address-of.rs:16:12: 16:24
+         AscribeUserType(_19, UserTypeProjection { base: UserType(9), projs: [] }); // scope 5 at $DIR/address-of.rs:16:12: 16:24
193         StorageLive(_21);                // scope 6 at $DIR/address-of.rs:18:5: 18:18
194         StorageLive(_22);                // scope 6 at $DIR/address-of.rs:18:5: 18:18
195         _22 = &raw const (*_3);          // scope 6 at $DIR/address-of.rs:18:5: 18:6

-         AscribeUserType(_22, o, UserTypeProjection { base: UserType(10), projs: [] }); // scope 6 at $DIR/address-of.rs:18:5: 18:18
+         AscribeUserType(_22, UserTypeProjection { base: UserType(10), projs: [] }); // scope 6 at $DIR/address-of.rs:18:5: 18:18
197         _21 = _22;                       // scope 6 at $DIR/address-of.rs:18:5: 18:18
198         StorageDead(_22);                // scope 6 at $DIR/address-of.rs:18:18: 18:19
199         StorageDead(_21);                // scope 6 at $DIR/address-of.rs:18:18: 18:19

206         _26 = &raw const (*_3);          // scope 6 at $DIR/address-of.rs:20:5: 20:6
207         _25 = move _26 as *const dyn std::marker::Send (Pointer(Unsize)); // scope 6 at $DIR/address-of.rs:20:5: 20:6
208         StorageDead(_26);                // scope 6 at $DIR/address-of.rs:20:5: 20:6
-         AscribeUserType(_25, o, UserTypeProjection { base: UserType(11), projs: [] }); // scope 6 at $DIR/address-of.rs:20:5: 20:25
+         AscribeUserType(_25, UserTypeProjection { base: UserType(11), projs: [] }); // scope 6 at $DIR/address-of.rs:20:5: 20:25
210         _24 = _25;                       // scope 6 at $DIR/address-of.rs:20:5: 20:25
211         StorageDead(_25);                // scope 6 at $DIR/address-of.rs:20:25: 20:26
212         StorageDead(_24);                // scope 6 at $DIR/address-of.rs:20:25: 20:26

219         StorageLive(_29);                // scope 6 at $DIR/address-of.rs:23:9: 23:10
220         _29 = &raw const (*_3);          // scope 6 at $DIR/address-of.rs:23:23: 23:24
221         FakeRead(ForLet(None), _29);     // scope 6 at $DIR/address-of.rs:23:9: 23:10
-         AscribeUserType(_29, o, UserTypeProjection { base: UserType(13), projs: [] }); // scope 6 at $DIR/address-of.rs:23:12: 23:20
+         AscribeUserType(_29, UserTypeProjection { base: UserType(13), projs: [] }); // scope 6 at $DIR/address-of.rs:23:12: 23:20
223         StorageLive(_30);                // scope 7 at $DIR/address-of.rs:24:9: 24:10
224         _30 = &raw const (*_3);          // scope 7 at $DIR/address-of.rs:24:31: 24:32
225         FakeRead(ForLet(None), _30);     // scope 7 at $DIR/address-of.rs:24:9: 24:10

-         AscribeUserType(_30, o, UserTypeProjection { base: UserType(15), projs: [] }); // scope 7 at $DIR/address-of.rs:24:12: 24:28
+         AscribeUserType(_30, UserTypeProjection { base: UserType(15), projs: [] }); // scope 7 at $DIR/address-of.rs:24:12: 24:28
227         StorageLive(_31);                // scope 8 at $DIR/address-of.rs:25:9: 25:10
228         StorageLive(_32);                // scope 8 at $DIR/address-of.rs:25:30: 25:31
229         _32 = &raw const (*_3);          // scope 8 at $DIR/address-of.rs:25:30: 25:31

230         _31 = move _32 as *const dyn std::marker::Send (Pointer(Unsize)); // scope 8 at $DIR/address-of.rs:25:30: 25:31
231         StorageDead(_32);                // scope 8 at $DIR/address-of.rs:25:30: 25:31
232         FakeRead(ForLet(None), _31);     // scope 8 at $DIR/address-of.rs:25:9: 25:10
-         AscribeUserType(_31, o, UserTypeProjection { base: UserType(17), projs: [] }); // scope 8 at $DIR/address-of.rs:25:12: 25:27
+         AscribeUserType(_31, UserTypeProjection { base: UserType(17), projs: [] }); // scope 8 at $DIR/address-of.rs:25:12: 25:27
234         StorageLive(_33);                // scope 9 at $DIR/address-of.rs:26:9: 26:10
235         StorageLive(_34);                // scope 9 at $DIR/address-of.rs:26:27: 26:28
236         _34 = &raw const (*_3);          // scope 9 at $DIR/address-of.rs:26:27: 26:28

237         _33 = move _34 as *const [i32] (Pointer(Unsize)); // scope 9 at $DIR/address-of.rs:26:27: 26:28
238         StorageDead(_34);                // scope 9 at $DIR/address-of.rs:26:27: 26:28
239         FakeRead(ForLet(None), _33);     // scope 9 at $DIR/address-of.rs:26:9: 26:10
-         AscribeUserType(_33, o, UserTypeProjection { base: UserType(19), projs: [] }); // scope 9 at $DIR/address-of.rs:26:12: 26:24
+         AscribeUserType(_33, UserTypeProjection { base: UserType(19), projs: [] }); // scope 9 at $DIR/address-of.rs:26:12: 26:24
241         StorageLive(_35);                // scope 10 at $DIR/address-of.rs:28:5: 28:16
242         StorageLive(_36);                // scope 10 at $DIR/address-of.rs:28:5: 28:16
243         _36 = &raw mut (*_3);            // scope 10 at $DIR/address-of.rs:28:5: 28:6

-         AscribeUserType(_36, o, UserTypeProjection { base: UserType(20), projs: [] }); // scope 10 at $DIR/address-of.rs:28:5: 28:16
+         AscribeUserType(_36, UserTypeProjection { base: UserType(20), projs: [] }); // scope 10 at $DIR/address-of.rs:28:5: 28:16
245         _35 = _36;                       // scope 10 at $DIR/address-of.rs:28:5: 28:16
246         StorageDead(_36);                // scope 10 at $DIR/address-of.rs:28:16: 28:17
247         StorageDead(_35);                // scope 10 at $DIR/address-of.rs:28:16: 28:17

254         _40 = &raw mut (*_3);            // scope 10 at $DIR/address-of.rs:30:5: 30:6
255         _39 = move _40 as *mut dyn std::marker::Send (Pointer(Unsize)); // scope 10 at $DIR/address-of.rs:30:5: 30:6
256         StorageDead(_40);                // scope 10 at $DIR/address-of.rs:30:5: 30:6
-         AscribeUserType(_39, o, UserTypeProjection { base: UserType(21), projs: [] }); // scope 10 at $DIR/address-of.rs:30:5: 30:23
+         AscribeUserType(_39, UserTypeProjection { base: UserType(21), projs: [] }); // scope 10 at $DIR/address-of.rs:30:5: 30:23
258         _38 = _39;                       // scope 10 at $DIR/address-of.rs:30:5: 30:23
259         StorageDead(_39);                // scope 10 at $DIR/address-of.rs:30:23: 30:24
260         StorageDead(_38);                // scope 10 at $DIR/address-of.rs:30:23: 30:24

267         StorageLive(_43);                // scope 10 at $DIR/address-of.rs:33:9: 33:10
268         _43 = &raw mut (*_3);            // scope 10 at $DIR/address-of.rs:33:21: 33:22
269         FakeRead(ForLet(None), _43);     // scope 10 at $DIR/address-of.rs:33:9: 33:10
-         AscribeUserType(_43, o, UserTypeProjection { base: UserType(23), projs: [] }); // scope 10 at $DIR/address-of.rs:33:12: 33:18
+         AscribeUserType(_43, UserTypeProjection { base: UserType(23), projs: [] }); // scope 10 at $DIR/address-of.rs:33:12: 33:18
271         StorageLive(_44);                // scope 11 at $DIR/address-of.rs:34:9: 34:10
272         _44 = &raw mut (*_3);            // scope 11 at $DIR/address-of.rs:34:29: 34:30
273         FakeRead(ForLet(None), _44);     // scope 11 at $DIR/address-of.rs:34:9: 34:10

-         AscribeUserType(_44, o, UserTypeProjection { base: UserType(25), projs: [] }); // scope 11 at $DIR/address-of.rs:34:12: 34:26
+         AscribeUserType(_44, UserTypeProjection { base: UserType(25), projs: [] }); // scope 11 at $DIR/address-of.rs:34:12: 34:26
275         StorageLive(_45);                // scope 12 at $DIR/address-of.rs:35:9: 35:10
276         StorageLive(_46);                // scope 12 at $DIR/address-of.rs:35:28: 35:29
277         _46 = &raw mut (*_3);            // scope 12 at $DIR/address-of.rs:35:28: 35:29

278         _45 = move _46 as *mut dyn std::marker::Send (Pointer(Unsize)); // scope 12 at $DIR/address-of.rs:35:28: 35:29
279         StorageDead(_46);                // scope 12 at $DIR/address-of.rs:35:28: 35:29
280         FakeRead(ForLet(None), _45);     // scope 12 at $DIR/address-of.rs:35:9: 35:10
-         AscribeUserType(_45, o, UserTypeProjection { base: UserType(27), projs: [] }); // scope 12 at $DIR/address-of.rs:35:12: 35:25
+         AscribeUserType(_45, UserTypeProjection { base: UserType(27), projs: [] }); // scope 12 at $DIR/address-of.rs:35:12: 35:25
282         StorageLive(_47);                // scope 13 at $DIR/address-of.rs:36:9: 36:10
283         StorageLive(_48);                // scope 13 at $DIR/address-of.rs:36:25: 36:26
284         _48 = &raw mut (*_3);            // scope 13 at $DIR/address-of.rs:36:25: 36:26

285         _47 = move _48 as *mut [i32] (Pointer(Unsize)); // scope 13 at $DIR/address-of.rs:36:25: 36:26
286         StorageDead(_48);                // scope 13 at $DIR/address-of.rs:36:25: 36:26
287         FakeRead(ForLet(None), _47);     // scope 13 at $DIR/address-of.rs:36:9: 36:10
-         AscribeUserType(_47, o, UserTypeProjection { base: UserType(29), projs: [] }); // scope 13 at $DIR/address-of.rs:36:12: 36:22
+         AscribeUserType(_47, UserTypeProjection { base: UserType(29), projs: [] }); // scope 13 at $DIR/address-of.rs:36:12: 36:22
289         _0 = const ();                   // scope 0 at $DIR/address-of.rs:3:26: 37:2
290         StorageDead(_47);                // scope 13 at $DIR/address-of.rs:37:1: 37:2
291         StorageDead(_45);                // scope 12 at $DIR/address-of.rs:37:1: 37:2

thread '[mir-opt] mir-opt/address-of.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/address_of.address_of_reborrow.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/issue-72181-1.rs stdout ----
30     bb1: {
30     bb1: {
31         StorageDead(_3);                 // scope 2 at $DIR/issue-72181-1.rs:17:43: 17:44
32         FakeRead(ForLet(None), _2);      // scope 0 at $DIR/issue-72181-1.rs:16:9: 16:10
-         AscribeUserType(_2, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/issue-72181-1.rs:16:12: 16:16
+         AscribeUserType(_2, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/issue-72181-1.rs:16:12: 16:16
34         StorageLive(_4);                 // scope 1 at $DIR/issue-72181-1.rs:20:5: 20:9
35         StorageLive(_5);                 // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8
36         _5 = move _2;                    // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8

thread '[mir-opt] mir-opt/issue-72181-1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181_1.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/receiver-ptr-mutability.rs stdout ----
37 
38     bb1: {
38     bb1: {
39         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/receiver-ptr-mutability.rs:14:9: 14:12
-         AscribeUserType(_1, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/receiver-ptr-mutability.rs:14:14: 14:23
+         AscribeUserType(_1, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/receiver-ptr-mutability.rs:14:14: 14:23
41         StorageLive(_2);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:12
42         StorageLive(_3);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:12
43         StorageLive(_4);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8

64         _6 = &_7;                        // scope 1 at $DIR/receiver-ptr-mutability.rs:18:34: 18:41
65         _5 = &(*_6);                     // scope 1 at $DIR/receiver-ptr-mutability.rs:18:34: 18:41
66         FakeRead(ForLet(None), _5);      // scope 1 at $DIR/receiver-ptr-mutability.rs:18:9: 18:16
-         AscribeUserType(_5, o, UserTypeProjection { base: UserType(3), projs: [] }); // scope 1 at $DIR/receiver-ptr-mutability.rs:18:18: 18:31
+         AscribeUserType(_5, UserTypeProjection { base: UserType(3), projs: [] }); // scope 1 at $DIR/receiver-ptr-mutability.rs:18:18: 18:31
68         StorageDead(_6);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:18:41: 18:42
69         StorageLive(_10);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
70         StorageLive(_11);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16

thread '[mir-opt] mir-opt/receiver-ptr-mutability.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/receiver_ptr_mutability.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3375:25

failures:
    [mir-opt] mir-opt/address-of.rs
    [mir-opt] mir-opt/basic_assignment.rs
