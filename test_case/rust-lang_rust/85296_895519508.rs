plain
31 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:103:8
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:104:8
33    |
34 LL | #[warn(x5400)]

36 
37 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:106:25
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:106:25
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:107:25
39    |
40 LL |     mod inner { #![warn(x5400)] }

42 
43 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:109:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:109:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:110:12
45    |
46 LL |     #[warn(x5400)] fn f() { }

48 
49 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:112:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:112:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:12
51    |
52 LL |     #[warn(x5400)] struct S;

54 
55 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:115:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:115:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:116:12
57    |
58 LL |     #[warn(x5400)] type T = S;

60 
61 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:118:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:118:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:119:12
63    |
64 LL |     #[warn(x5400)] impl S { }

66 
67 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:122:9
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:122:9
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:123:9
69    |
70 LL | #[allow(x5300)]

72 
73 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:125:26
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:125:26
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:126:26
75    |
76 LL |     mod inner { #![allow(x5300)] }

78 
79 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:128:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:128:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:129:13
81    |
82 LL |     #[allow(x5300)] fn f() { }

84 
85 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:131:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:131:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:132:13
87    |
88 LL |     #[allow(x5300)] struct S;

90 
91 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:134:13
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:134:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:135:13
93    |
94 LL |     #[allow(x5300)] type T = S;

96 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
97 warning: unknown lint: `x5300`
97 warning: unknown lint: `x5300`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:137:13
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:138:13
99    |
100 LL |     #[allow(x5300)] impl S { }

102 
103 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:141:10
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:141:10
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:142:10
105    |
106 LL | #[forbid(x5200)]

108 
109 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:144:27
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:144:27
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:145:27
111    |
112 LL |     mod inner { #![forbid(x5200)] }

114 
115 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:147:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:147:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:148:14
117    |
118 LL |     #[forbid(x5200)] fn f() { }

120 
121 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:150:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:150:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:151:14
123    |
124 LL |     #[forbid(x5200)] struct S;

126 
127 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:153:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:153:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:154:14
129    |
130 LL |     #[forbid(x5200)] type T = S;

132 
133 warning: unknown lint: `x5200`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:156:14
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:156:14
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:157:14
135    |
136 LL |     #[forbid(x5200)] impl S { }

138 
139 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:160:8
---
145 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:163:25
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:164:25
147    |
148 LL |     mod inner { #![deny(x5100)] }

150 
151 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:166:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:166:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:167:12
153    |
154 LL |     #[deny(x5100)] fn f() { }

156 
157 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:169:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:169:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:170:12
159    |
160 LL |     #[deny(x5100)] struct S;

162 
163 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:172:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:172:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:173:12
165    |
166 LL |     #[deny(x5100)] type T = S;

168 
169 warning: unknown lint: `x5100`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:175:12
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:175:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:176:12
171    |
172 LL |     #[deny(x5100)] impl S { }

174 
174 
175 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:404:17
177    |
177    |
178 LL |     mod inner { #![macro_escape] }


181    = help: try an outer attribute: `#[macro_use]`
182 
183 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:401:1
185    |
186 LL | #[macro_escape]
187    | ^^^^^^^^^^^^^^^


195    = note: `#[warn(deprecated)]` on by default
196 
197 warning: use of deprecated attribute `no_start`: no longer used.
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:92:1
199    |
199    |
200 LL | #![no_start]

202 
203 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:296:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:296:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:297:1
205    |
206 LL |   #[no_mangle]

223    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
224 
225 warning: attribute should be applied to a function
225 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:463:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:464:1
227    |
228 LL |   #[cold]

240    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
241 
242 warning: attribute should be applied to a foreign function or static
---
300 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:301:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:302:17
302    |
303 LL |     mod inner { #![no_mangle] }
304    |     ------------^^^^^^^^^^^^^-- not a function or static
306    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
307 
308 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:308:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:308:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:309:5
310    |
311 LL |     #[no_mangle] struct S;

314    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
315 
316 warning: attribute should be applied to a function or static
316 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:313:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:314:5
318    |
319 LL |     #[no_mangle] type T = S;

322    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
323 
324 warning: attribute should be applied to a function or static
324 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:318:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:319:5
326    |
327 LL |     #[no_mangle] impl S { }

330    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
331 
332 warning: attribute should be applied to a function
332 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:469:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:470:17
334    |
335 LL |     mod inner { #![cold] }
336    |     ------------^^^^^^^^-- not a function
338    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
339 
340 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:477:5
342    |
343 LL |     #[cold] struct S;

346    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
347 
348 warning: attribute should be applied to a function
348 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:481:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:482:5
350    |
351 LL |     #[cold] type T = S;

354    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
355 
356 warning: attribute should be applied to a function
356 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:486:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:487:5
358    |
359 LL |     #[cold] impl S { }

362    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
363 
364 warning: attribute should be applied to a foreign function or static
---
368    |     ^^^^^^^^^^^^^^^^^^^^^

372    |
373    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
374 help: try `#[link(name = "1900")]` instead
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:498:5
376    |
377 LL |     #[link_name = "1900"]
378    |     ^^^^^^^^^^^^^^^^^^^^^


379 
380 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:505:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:506:17
382    |
383 LL |     mod inner { #![link_name="1900"] }
384    |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
386    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
387 
388 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:510:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:510:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:511:5
390    |
391 LL |     #[link_name = "1900"] fn f() { }

394    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
395 
396 warning: attribute should be applied to a foreign function or static
396 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:515:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:516:5
398    |
399 LL |     #[link_name = "1900"] struct S;

402    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
403 
404 warning: attribute should be applied to a foreign function or static
404 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:520:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:521:5
406    |
407 LL |     #[link_name = "1900"] type T = S;

410    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
411 
412 warning: attribute should be applied to a foreign function or static
412 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:525:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:526:5
414    |
415 LL |     #[link_name = "1900"] impl S { }

418    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
419 
420 warning: attribute should be applied to a function or static
420 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:537:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:538:17
422    |
423 LL |     mod inner { #![link_section="1800"] }
424    |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
426    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
427 
428 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:544:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:544:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:545:5
430    |
431 LL |     #[link_section = "1800"] struct S;

434    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
435 
436 warning: attribute should be applied to a function or static
436 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:549:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:550:5
438    |
439 LL |     #[link_section = "1800"] type T = S;

442    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
443 
444 warning: attribute should be applied to a function or static
444 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:554:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:555:5
446    |
447 LL |     #[link_section = "1800"] impl S { }

450    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
451 
452 warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
452 warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:88:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:89:12
454    |
455 LL | #![feature(rust1)]

476    | ^^^^^^^^^^^^^^^^^^^^^^^
477 
478 warning: unused attribute
478 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:183:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:184:5
480    |
481 LL |     #[macro_use] fn f() { }

483 
484 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:186:5
---
490 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:189:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:190:5
492    |
493 LL |     #[macro_use] type T = S;

495 
496 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:192:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:192:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:193:5
498    |
499 LL |     #[macro_use] impl S { }

501 
502 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:196:1
---
508 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:199:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:200:17
510    |
511 LL |     mod inner { #![macro_export] }

513 
514 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:202:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:202:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:203:5
516    |
517 LL |     #[macro_export] fn f() { }

519 
520 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:205:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:205:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:206:5
522    |
523 LL |     #[macro_export] struct S;

525 
526 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:208:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:208:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:209:5
528    |
529 LL |     #[macro_export] type T = S;

531 
532 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:211:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:211:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:212:5
534    |
535 LL |     #[macro_export] impl S { }

537 
538 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:264:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:264:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:265:5
540    |
541 LL |     #[path = "3800"] fn f() { }

543 
544 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:267:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:267:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:268:5
546    |
547 LL |     #[path = "3800"]  struct S;

549 
550 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:270:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:270:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:271:5
552    |
553 LL |     #[path = "3800"] type T = S;

555 
556 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:273:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:273:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:274:5
558    |
559 LL |     #[path = "3800"] impl S { }

561 
562 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:277:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:277:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:278:1
564    |
565 LL | #[automatically_derived]

567 
568 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:280:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:280:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:281:17
570    |
571 LL |     mod inner { #![automatically_derived] }

573 
574 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:283:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:283:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:284:5
576    |
577 LL |     #[automatically_derived] fn f() { }

579 
580 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:286:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:286:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:287:5
582    |
583 LL |     #[automatically_derived] struct S;

585 
586 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:289:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:289:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:290:5
588    |
589 LL |     #[automatically_derived] type T = S;

591 
592 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:292:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:292:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:293:5
594    |
595 LL |     #[automatically_derived] impl S { }

597 
598 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:324:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:324:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:325:1
600    |
601 LL | #[should_panic]

603 
604 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:327:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:327:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:328:17
606    |
607 LL |     mod inner { #![should_panic] }

609 
610 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:330:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:330:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:331:5
612    |
613 LL |     #[should_panic] fn f() { }

615 
616 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:333:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:333:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:334:5
618    |
619 LL |     #[should_panic] struct S;

621 
622 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:336:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:336:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:337:5
624    |
625 LL |     #[should_panic] type T = S;

627 
628 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:339:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:339:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:340:5
630    |
631 LL |     #[should_panic] impl S { }

633 
634 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:343:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:343:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:344:1
636    |
637 LL | #[ignore]

639 
640 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:346:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:346:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:347:17
642    |
643 LL |     mod inner { #![ignore] }

645 
646 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:349:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:349:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:350:5
648    |
649 LL |     #[ignore] fn f() { }

651 
652 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:352:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:352:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:353:5
654    |
655 LL |     #[ignore] struct S;

657 
658 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:355:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:355:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:356:5
660    |
661 LL |     #[ignore] type T = S;

663 
664 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:358:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:358:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:359:5
666    |
667 LL |     #[ignore] impl S { }

669 
670 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:362:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:362:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:363:1
672    |
673 LL | #[no_implicit_prelude]

675 
676 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:366:17
678    |
679 LL |     mod inner { #![no_implicit_prelude] }

681 
682 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:368:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:368:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:369:5
684    |
685 LL |     #[no_implicit_prelude] fn f() { }

687 
688 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:371:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:371:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:372:5
690    |
691 LL |     #[no_implicit_prelude] struct S;

693 
694 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:374:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:374:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:375:5
696    |
697 LL |     #[no_implicit_prelude] type T = S;

699 
700 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:377:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:377:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:378:5
702    |
703 LL |     #[no_implicit_prelude] impl S { }

705 
706 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:381:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:381:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:382:1
708    |
709 LL | #[reexport_test_harness_main = "2900"]

711 
712 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:384:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:384:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:385:17
714    |
715 LL |     mod inner { #![reexport_test_harness_main="2900"] }

717 
718 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:387:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:387:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:388:5
720    |
721 LL |     #[reexport_test_harness_main = "2900"] fn f() { }

723 
724 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:390:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:390:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:391:5
726    |
727 LL |     #[reexport_test_harness_main = "2900"] struct S;

729 
730 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:393:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:393:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:394:5
732    |
733 LL |     #[reexport_test_harness_main = "2900"] type T = S;

735 
736 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:396:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:396:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:397:5
738    |
739 LL |     #[reexport_test_harness_main = "2900"] impl S { }

741 
742 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:409:5
744    |
745 LL |     #[macro_escape] fn f() { }

747 
748 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:412:5
750    |
751 LL |     #[macro_escape] struct S;

753 
754 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:414:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:414:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:415:5
756    |
757 LL |     #[macro_escape] type T = S;

759 
760 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:417:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:417:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:418:5
762    |
763 LL |     #[macro_escape] impl S { }

765 
766 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:422:1
768    |
769 LL | #[no_std]

771 
771 
772 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:1
774    |
774    |
775 LL | #[no_std]

777 
778 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
780    |
781 LL |     mod inner { #![no_std] }

783 
784 warning: crate-level attribute should be in the root module
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
786    |
787 LL |     mod inner { #![no_std] }

789 
790 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:429:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:429:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
792    |
793 LL |     #[no_std] fn f() { }

795 
795 
796 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:429:5
798    |
798    |
799 LL |     #[no_std] fn f() { }

801 
802 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:5
804    |
805 LL |     #[no_std] struct S;

807 
807 
808 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
810    |
810    |
811 LL |     #[no_std] struct S;

813 
814 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:5
816    |
817 LL |     #[no_std] type T = S;

819 
819 
820 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:5
822    |
822    |
823 LL |     #[no_std] type T = S;

825 
826 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:441:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:441:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:442:5
828    |
829 LL |     #[no_std] impl S { }

831 
831 
832 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:441:5
834    |
834    |
835 LL |     #[no_std] impl S { }

837 
838 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:622:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:622:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:623:1
840    |
841 LL | #[crate_name = "0900"]

843 
843 
844 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:622:1
846    |
846    |
847 LL | #[crate_name = "0900"]

849 
850 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:626:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:626:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
852    |
853 LL |     mod inner { #![crate_name="0900"] }

855 
856 warning: crate-level attribute should be in the root module
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:626:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:626:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
858    |
859 LL |     mod inner { #![crate_name="0900"] }

861 
862 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:630:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:630:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:5
864    |
865 LL |     #[crate_name = "0900"] fn f() { }

867 
867 
868 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:630:5
870    |
870    |
871 LL |     #[crate_name = "0900"] fn f() { }

873 
874 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:634:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:634:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:5
876    |
877 LL |     #[crate_name = "0900"] struct S;

879 
879 
880 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:634:5
882    |
882    |
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:506:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:511:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:511:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:516:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:521:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:526:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:538:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:545:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:545:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:550:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:555:5
   |
LL |     #[link_section = "1800"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:89:12
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:184:5
   |
   |
LL |     #[macro_use] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:187:5
   |
   |
LL |     #[macro_use] struct S;
   |     ^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:190:5
   |
LL |     #[macro_use] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:193:5
   |
   |
LL |     #[macro_use] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:197:1
   |
   |
LL | #[macro_export]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:200:17
   |
LL |     mod inner { #![macro_export] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:203:5
   |
   |
LL |     #[macro_export] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:206:5
   |
   |
LL |     #[macro_export] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:209:5
   |
   |
LL |     #[macro_export] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:212:5
   |
   |
LL |     #[macro_export] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:265:5
   |
   |
LL |     #[path = "3800"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:268:5
   |
   |
LL |     #[path = "3800"]  struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:271:5
   |
   |
LL |     #[path = "3800"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:274:5
   |
   |
LL |     #[path = "3800"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:278:1
   |
   |
LL | #[automatically_derived]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:281:17
   |
   |
LL |     mod inner { #![automatically_derived] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:284:5
   |
   |
LL |     #[automatically_derived] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:287:5
   |
   |
LL |     #[automatically_derived] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:290:5
   |
   |
LL |     #[automatically_derived] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:293:5
   |
   |
LL |     #[automatically_derived] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:325:1
   |
   |
LL | #[should_panic]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:328:17
   |
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:331:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:334:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:337:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:340:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:344:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:347:17
   |
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:350:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:353:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:356:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:359:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:363:1
   |
   |
LL | #[no_implicit_prelude]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:366:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:369:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:372:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:375:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:378:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:382:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:385:17
   |
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:388:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:391:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:394:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:397:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:409:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:412:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:415:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:418:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:422:1
   |
   |
LL | #[no_std]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[no_std]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:426:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:426:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:430:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:434:5
   |
   |
LL |     #[no_std] struct S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:438:5
   |
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:442:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:623:1
   |
   |
LL | #[crate_name = "0900"]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[crate_name = "0900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:627:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:627:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:631:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
---
test result: FAILED. 12022 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 122.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:28
