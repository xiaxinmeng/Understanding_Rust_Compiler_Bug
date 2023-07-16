plain
Successfully built 2503c91b6a88
Successfully tagged rust-ci:latest
Built container sha256:2503c91b6a88b100359dfb735604a62936a1af4bfad87bb8a2018d13d0900c8f
Uploading finished image to https://ci-caches.rust-lang.org/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174
upload failed: - to s3://rust-lang-ci-sccache2/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
1 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:52:9
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:45:9
3    |
- LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
+ LL | #![warn(x5400)]
6    |
7 note: the lint level is defined here

11    |                            ^^^^^^^^^^^^^
11    |                            ^^^^^^^^^^^^^
12 
13 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:53:10
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:46:10
15    |
- LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`
+ LL | #![allow(x5300)]
18 
19 warning: unknown lint: `x5200`

-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:54:11
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:54:11
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:47:11
21    |
- LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`
+ LL | #![forbid(x5200)]
24 
25 warning: unknown lint: `x5100`

-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:55:9
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:55:9
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:48:9
27    |
- LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`
+ LL | #![deny(x5100)]
30 
31 warning: unknown lint: `x5400`

-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:110:8
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:110:8
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:103:8
33    |
34 LL | #[warn(x5400)]

36 
37 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:25
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:25
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:106:25
39    |
40 LL |     mod inner { #![warn(x5400)] }

42 
43 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:116:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:116:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:109:12
45    |
46 LL |     #[warn(x5400)] fn f() { }

48 
49 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:119:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:119:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:112:12
51    |
52 LL |     #[warn(x5400)] struct S;

54 
55 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:122:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:122:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:115:12
57    |
58 LL |     #[warn(x5400)] type T = S;

60 
61 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:125:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:125:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:118:12
63    |
64 LL |     #[warn(x5400)] impl S { }

66 
67 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:129:9
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:129:9
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:122:9
69    |
70 LL | #[allow(x5300)]

72 
73 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:132:26
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:132:26
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:125:26
75    |
76 LL |     mod inner { #![allow(x5300)] }

78 
79 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:135:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:135:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:128:13
81    |
82 LL |     #[allow(x5300)] fn f() { }

84 
85 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:138:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:138:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:131:13
87    |
88 LL |     #[allow(x5300)] struct S;

90 
91 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:141:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:141:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:134:13
93    |
94 LL |     #[allow(x5300)] type T = S;

96 
97 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:144:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:144:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:137:13
99    |
100 LL |     #[allow(x5300)] impl S { }

102 
103 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:148:10
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:148:10
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:141:10
105    |
106 LL | #[forbid(x5200)]

108 
109 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:151:27
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:151:27
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:144:27
111    |
112 LL |     mod inner { #![forbid(x5200)] }

114 
115 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:154:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:154:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:147:14
117    |
118 LL |     #[forbid(x5200)] fn f() { }

120 
121 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:157:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:157:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:150:14
123    |
124 LL |     #[forbid(x5200)] struct S;

126 
127 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:160:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:160:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:153:14
129    |
130 LL |     #[forbid(x5200)] type T = S;

132 
133 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:163:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:163:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:156:14
135    |
136 LL |     #[forbid(x5200)] impl S { }

138 
139 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:167:8
---
145 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:170:25
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:163:25
147    |
148 LL |     mod inner { #![deny(x5100)] }

150 
151 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:173:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:173:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:166:12
153    |
154 LL |     #[deny(x5100)] fn f() { }

156 
157 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:176:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:176:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:169:12
159    |
160 LL |     #[deny(x5100)] struct S;

162 
163 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:179:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:179:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:172:12
165    |
166 LL |     #[deny(x5100)] type T = S;

168 
169 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:182:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:182:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:175:12
171    |
172 LL |     #[deny(x5100)] impl S { }

174 
174 
175 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:17
177    |
177    |
178 LL |     mod inner { #![macro_escape] }


181    = help: try an outer attribute: `#[macro_use]`
182 
183 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:1
185    |
186 LL | #[macro_escape]
187    | ^^^^^^^^^^^^^^^


188 
- error: cannot find attribute `plugin_registrar` in this scope
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:45:4
-    |
- LL | #![plugin_registrar]
- 
- error: cannot determine resolution for the attribute macro `test`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:225:3
-    |
-    |
- LL | #[test]
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- error: cannot determine resolution for the attribute macro `test`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:226:27
-    |
-    |
- LL | mod test { mod inner { #![test] }
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
- error: cannot determine resolution for the attribute macro `bench`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:240:3
-    |
- LL | #[bench]
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
- error: cannot determine resolution for the attribute macro `bench`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:242:20
-    |
- LL |     mod inner { #![bench] }
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
- error: cannot determine resolution for the attribute macro `bench`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:244:7
-    |
- LL |     #[bench]
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
- error: cannot determine resolution for the attribute macro `bench`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:247:7
-    |
- LL |     #[bench]
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
- error: cannot determine resolution for the attribute macro `bench`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:250:7
-    |
- LL |     #[bench]
-    |
-    |
-    = note: import resolution is stuck, try simplifying macro imports
- 
251 warning: use of deprecated attribute `crate_id`: no longer used.
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:90:1
253    |
253    |
254 LL | #![crate_id = "10"]

257    = note: `#[warn(deprecated)]` on by default
258 
258 
259 warning: use of deprecated attribute `no_start`: no longer used.
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:99:1
261    |
261    |
262 LL | #![no_start]

264 
265 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:303:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:303:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:296:1
267    |
268 LL |   #[no_mangle]

270 ...
270 ...
271 LL | / mod no_mangle {
- LL | |     //~^ NOTE not a function or static
+ LL | |
273 LL | |     mod inner { #![no_mangle] }
- LL | |     //~^ WARN attribute should be applied to a function or static [unused_attributes]
+ LL | |
275 ...  |
- LL | |     //~| NOTE not a function or static
+ LL | |
277 LL | | }
278    | |_- not a function or static

285    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
286 
287 warning: attribute should be applied to a function
287 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:470:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:463:1
289    |
290 LL |   #[cold]

292 ...
292 ...
293 LL | / mod cold {
- LL | |     //~^ NOTE not a function
295 LL | |
+ LL | |
296 LL | |     mod inner { #![cold] }
297 ...  |
- LL | |     //~| NOTE not a function
+ LL | |
299 LL | | }
300    | |_- not a function

302    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
303 
304 warning: attribute should be applied to a foreign function or static
304 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:499:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:492:1
306    |
307 LL |   #[link_name = "1900"]
308    |   ^^^^^^^^^^^^^^^^^^^^^

309 ...
310 LL | / mod link_name {
- LL | |     //~^ NOTE not a foreign function or static
312 LL | |
+ LL | |
313 LL | |     #[link_name = "1900"]
314 ...  |
- LL | |     //~| NOTE not a foreign function or static
+ LL | |
316 LL | | }
317    | |_- not a foreign function or static

319    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
320 
321 warning: attribute should be applied to a function or static
321 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:538:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:531:1
323    |
324 LL |   #[link_section = "1800"]
325    |   ^^^^^^^^^^^^^^^^^^^^^^^^

326 ...
327 LL | / mod link_section {
- LL | |     //~^ NOTE not a function or static
329 LL | |
+ LL | |
330 LL | |     mod inner { #![link_section="1800"] }
331 ...  |
- LL | |     //~| NOTE not a function or static
+ LL | |
333 LL | | }
334    | |_- not a function or static

336    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
337 
338 warning: attribute should be applied to a function
338 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:68:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:61:1
340    |
- LL | #![cold] //~ WARN attribute should be applied to a function
+ LL | #![cold]
343    |
344    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

345 
345 
346 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:72:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:65:1
348    |
349 LL | #![link_name = "1900"]

352    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
353 
354 warning: attribute should be applied to a function or static
354 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:75:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:68:1
356    |
357 LL | #![link_section = "1800"]

360    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
361 
362 warning: attribute should be applied to a function or static
362 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:308:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:301:17
364    |
365 LL |     mod inner { #![no_mangle] }
366    |     ------------^^^^^^^^^^^^^-- not a function or static
368    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
369 
370 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:315:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:315:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:308:5
372    |
373 LL |     #[no_mangle] struct S;

376    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
377 
378 warning: attribute should be applied to a function or static
378 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:320:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:313:5
380    |
381 LL |     #[no_mangle] type T = S;

384    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
385 
386 warning: attribute should be applied to a function or static
386 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:325:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:318:5
388    |
389 LL |     #[no_mangle] impl S { }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

392    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
393 
393 
394 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:469:17
396    |
397 LL |     mod inner { #![cold] }
398    |     ------------^^^^^^^^-- not a function
400    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
401 
402 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:483:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:483:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:5
404    |
405 LL |     #[cold] struct S;

408    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
409 
410 warning: attribute should be applied to a function
410 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:488:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:481:5
412    |
413 LL |     #[cold] type T = S;

416    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
417 
418 warning: attribute should be applied to a function
418 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:493:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:486:5
420    |
421 LL |     #[cold] impl S { }

424    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
425 
426 warning: attribute should be applied to a foreign function or static
---
430    |     ^^^^^^^^^^^^^^^^^^^^^

434    |
435    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
436 help: try `#[link(name = "1900")]` instead
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:505:5
438    |
439 LL |     #[link_name = "1900"]
440    |     ^^^^^^^^^^^^^^^^^^^^^


441 
442 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:512:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:505:17
444    |
445 LL |     mod inner { #![link_name="1900"] }
446    |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
448    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
449 
450 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:517:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:517:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:510:5
452    |
453 LL |     #[link_name = "1900"] fn f() { }

456    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
457 
458 warning: attribute should be applied to a foreign function or static
458 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:522:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:515:5
460    |
461 LL |     #[link_name = "1900"] struct S;

464    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
465 
466 warning: attribute should be applied to a foreign function or static
466 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:527:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:520:5
468    |
469 LL |     #[link_name = "1900"] type T = S;

472    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
473 
474 warning: attribute should be applied to a foreign function or static
474 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:532:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:525:5
476    |
477 LL |     #[link_name = "1900"] impl S { }

480    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
481 
482 warning: attribute should be applied to a function or static
482 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:544:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:537:17
484    |
485 LL |     mod inner { #![link_section="1800"] }
486    |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
488    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
489 
490 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:551:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:551:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:544:5
492    |
493 LL |     #[link_section = "1800"] struct S;

496    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
497 
498 warning: attribute should be applied to a function or static
498 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:556:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:549:5
500    |
501 LL |     #[link_section = "1800"] type T = S;

504    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
505 
506 warning: attribute should be applied to a function or static
506 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:561:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:554:5
508    |
509 LL |     #[link_section = "1800"] impl S { }

511    |
512    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
513 
513 
- error: aborting due to 9 previous errors; 57 warnings emitted
+ warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
+    |
+    |
+ LL | #![feature(rust1)]
+    |
+    = note: `#[warn(stable_features)]` on by default
515 
- For more information about this error, try `rustc --explain E0557`.
- For more information about this error, try `rustc --explain E0557`.
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:52:1
+    |
+ LL | #![should_panic]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:53:1
+    |
+    |
+ LL | #![ignore]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:59:1
+    |
+    |
+ LL | #![proc_macro_derive()]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:183:5
+    |
+    |
+ LL |     #[macro_use] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:186:5
+    |
+    |
+ LL |     #[macro_use] struct S;
+    |     ^^^^^^^^^^^^
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:189:5
+    |
+ LL |     #[macro_use] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:192:5
+    |
+    |
+ LL |     #[macro_use] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:196:1
+    |
+    |
+ LL | #[macro_export]
+    | ^^^^^^^^^^^^^^^
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:199:17
+    |
+ LL |     mod inner { #![macro_export] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:202:5
+    |
+    |
+ LL |     #[macro_export] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:205:5
+    |
+    |
+ LL |     #[macro_export] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:208:5
+    |
+    |
+ LL |     #[macro_export] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:211:5
+    |
+    |
+ LL |     #[macro_export] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:264:5
+    |
+    |
+ LL |     #[path = "3800"] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:267:5
+    |
+    |
+ LL |     #[path = "3800"]  struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:270:5
+    |
+    |
+ LL |     #[path = "3800"] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:273:5
+    |
+    |
+ LL |     #[path = "3800"] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:277:1
+    |
+    |
+ LL | #[automatically_derived]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:280:17
+    |
+    |
+ LL |     mod inner { #![automatically_derived] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:283:5
+    |
+    |
+ LL |     #[automatically_derived] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:286:5
+    |
+    |
+ LL |     #[automatically_derived] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:289:5
+    |
+    |
+ LL |     #[automatically_derived] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:292:5
+    |
+    |
+ LL |     #[automatically_derived] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:324:1
+    |
+    |
+ LL | #[should_panic]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:327:17
+    |
+    |
+ LL |     mod inner { #![should_panic] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:330:5
+    |
+    |
+ LL |     #[should_panic] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:333:5
+    |
+    |
+ LL |     #[should_panic] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:336:5
+    |
+    |
+ LL |     #[should_panic] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:339:5
+    |
+    |
+ LL |     #[should_panic] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:343:1
+    |
+    |
+ LL | #[ignore]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:346:17
+    |
+    |
+ LL |     mod inner { #![ignore] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:349:5
+    |
+    |
+ LL |     #[ignore] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:352:5
+    |
+    |
+ LL |     #[ignore] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:355:5
+    |
+    |
+ LL |     #[ignore] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:358:5
+    |
+    |
+ LL |     #[ignore] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:362:1
+    |
+    |
+ LL | #[no_implicit_prelude]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:17
+    |
+    |
+ LL |     mod inner { #![no_implicit_prelude] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:368:5
+    |
+    |
+ LL |     #[no_implicit_prelude] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:371:5
+    |
+    |
+ LL |     #[no_implicit_prelude] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:374:5
+    |
+    |
+ LL |     #[no_implicit_prelude] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:377:5
+    |
+    |
+ LL |     #[no_implicit_prelude] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:381:1
+    |
+    |
+ LL | #[reexport_test_harness_main = "2900"]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:384:17
+    |
+    |
+ LL |     mod inner { #![reexport_test_harness_main="2900"] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:387:5
+    |
+    |
+ LL |     #[reexport_test_harness_main = "2900"] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:390:5
+    |
+    |
+ LL |     #[reexport_test_harness_main = "2900"] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:393:5
+    |
+    |
+ LL |     #[reexport_test_harness_main = "2900"] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:396:5
+    |
+    |
+ LL |     #[reexport_test_harness_main = "2900"] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:5
+    |
+    |
+ LL |     #[macro_escape] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:5
+    |
+    |
+ LL |     #[macro_escape] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:414:5
+    |
+    |
+ LL |     #[macro_escape] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:417:5
+    |
+    |
+ LL |     #[macro_escape] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:1
+    |
+    |
+ LL | #[no_std]
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL | #[no_std]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:17
+    |
+    |
+ LL |     mod inner { #![no_std] }
+ 
+ 
+ warning: crate-level attribute should be in the root module
+    |
+    |
+ LL |     mod inner { #![no_std] }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:429:5
+    |
+    |
+ LL |     #[no_std] fn f() { }
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL |     #[no_std] fn f() { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
+    |
+    |
+ LL |     #[no_std] struct S;
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL |     #[no_std] struct S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:5
+    |
+    |
+ LL |     #[no_std] type T = S;
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL |     #[no_std] type T = S;
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:441:5
+    |
+    |
+ LL |     #[no_std] impl S { }
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL |     #[no_std] impl S { }
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:622:1
+    |
+    |
+ LL | #[crate_name = "0900"]
+ 
+ 
+ warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
+    |
+    |
+ LL | #[crate_name = "0900"]
+ 
+ warning: unused attribute
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:626:17
+    |
+    |
+ LL |     mod inner { #![crate_name="0900"] }
+ 
+ 
---
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-builtin-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:45:9
   |
LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:28
   |
   |
LL | #![warn(unused_attributes, unknown_lints)]
   |                            ^^^^^^^^^^^^^

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:46:10
   |
LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:47:11
   |
   |
LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:48:9
   |
   |
LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:103:8
   |
   |
LL | #[warn(x5400)]

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:106:25
   |
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:109:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:112:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:115:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:118:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:122:9
   |
   |
LL | #[allow(x5300)]

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:125:26
   |
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:128:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:131:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:134:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:137:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:141:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:144:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:147:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:150:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:153:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:156:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:160:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:163:25
   |
LL |     mod inner { #![deny(x5100)] }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:166:12
   |
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:169:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:172:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:175:12
   |
   |
LL |     #[deny(x5100)] impl S { }


warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
   |
LL |     mod inner { #![macro_escape] }
   |
   |
   = help: try an outer attribute: `#[macro_use]`

warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
LL | #[macro_escape]
   | ^^^^^^^^^^^^^^^


warning: use of deprecated attribute `crate_id`: no longer used.
   |
   |
LL | #![crate_id = "10"]
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated attribute `no_start`: no longer used.
   |
   |
LL | #![no_start]

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:296:1
   |
   |
LL |   #[no_mangle]
...
...
LL | / mod no_mangle {
LL | |     //~^ NOTE not a function or static
LL | |     mod inner { #![no_mangle] }
LL | |     //~^ WARN attribute should be applied to a function or static [unused_attributes]
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:9
   |
LL | #![warn(unused_attributes, unknown_lints)]
LL | #![warn(unused_attributes, unknown_lints)]
   |         ^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:463:1
   |
LL |   #[cold]
...
...
LL | / mod cold {
LL | |     //~^ NOTE not a function
LL | |
LL | |     mod inner { #![cold] }
...  |
LL | |     //~| NOTE not a function
LL | | }
   | |_- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:492:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:492:1
   |
LL |   #[link_name = "1900"]
   |   ^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_name {
LL | |     //~^ NOTE not a foreign function or static
LL | |
LL | |     #[link_name = "1900"]
...  |
LL | |     //~| NOTE not a foreign function or static
LL | | }
   | |_- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:531:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:531:1
   |
LL |   #[link_section = "1800"]
   |   ^^^^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_section {
LL | |     //~^ NOTE not a function or static
LL | |
LL | |     mod inner { #![link_section="1800"] }
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:61:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:61:1
   |
LL | #![cold] //~ WARN attribute should be applied to a function
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:65:1
   |
LL | #![link_name = "1900"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:68:1
   |
LL | #![link_section = "1800"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:301:17
   |
LL |     mod inner { #![no_mangle] }
   |     ------------^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:308:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:308:5
   |
LL |     #[no_mangle] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:313:5
   |
LL |     #[no_mangle] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:318:5
   |
LL |     #[no_mangle] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:469:17
   |
LL |     mod inner { #![cold] }
   |     ------------^^^^^^^^-- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:5
   |
LL |     #[cold] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:481:5
   |
LL |     #[cold] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:486:5
   |
LL |     #[cold] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
---
LL |     extern "C" { }
   |     -------------- not a foreign function or static
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
help: try `#[link(name = "1900")]` instead
   |
LL |     #[link_name = "1900"]
   |     ^^^^^^^^^^^^^^^^^^^^^


warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:505:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:510:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:510:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:515:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:520:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:525:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:537:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:549:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:554:5
   |
LL |     #[link_section = "1800"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:88:12
   |
LL | #![feature(rust1)]
   |
   = note: `#[warn(stable_features)]` on by default

warning: unused attribute
warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:52:1
   |
LL | #![should_panic] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:53:1
   |
   |
LL | #![ignore] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:59:1
   |
   |
LL | #![proc_macro_derive()] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:183:5
   |
   |
LL |     #[macro_use] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:186:5
   |
   |
LL |     #[macro_use] struct S;
   |     ^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:189:5
   |
LL |     #[macro_use] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:192:5
   |
   |
LL |     #[macro_use] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:196:1
   |
   |
LL | #[macro_export]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:199:17
   |
LL |     mod inner { #![macro_export] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:202:5
   |
   |
LL |     #[macro_export] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:205:5
   |
   |
LL |     #[macro_export] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:208:5
   |
   |
LL |     #[macro_export] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:211:5
   |
   |
LL |     #[macro_export] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:264:5
   |
   |
LL |     #[path = "3800"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:267:5
   |
   |
LL |     #[path = "3800"]  struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:270:5
   |
   |
LL |     #[path = "3800"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:273:5
   |
   |
LL |     #[path = "3800"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:277:1
   |
   |
LL | #[automatically_derived]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:280:17
   |
   |
LL |     mod inner { #![automatically_derived] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:283:5
   |
   |
LL |     #[automatically_derived] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:286:5
   |
   |
LL |     #[automatically_derived] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:289:5
   |
   |
LL |     #[automatically_derived] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:292:5
   |
   |
LL |     #[automatically_derived] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:324:1
   |
   |
LL | #[should_panic]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:327:17
   |
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:330:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:333:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:336:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:339:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:343:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:346:17
   |
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:349:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:352:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:355:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:358:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:362:1
   |
   |
LL | #[no_implicit_prelude]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:365:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:368:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:371:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:374:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:377:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:381:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:384:17
   |
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:387:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:390:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:393:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:396:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:408:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:411:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:414:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:417:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:421:1
   |
   |
LL | #[no_std]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[no_std]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:425:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:425:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:429:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:433:5
   |
   |
LL |     #[no_std] struct S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:437:5
   |
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:441:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:622:1
   |
   |
LL | #[crate_name = "0900"]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[crate_name = "0900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:626:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:626:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:630:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
---
test result: FAILED. 11906 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 126.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:13
