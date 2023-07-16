plain
diff of stderr:

173    |            ^^^^^
174 
175 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:405:17
177    |
177    |
178 LL |     mod inner { #![macro_escape] }


181    = help: try an outer attribute: `#[macro_use]`
182 
183 warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:402:1
185    |
186 LL | #[macro_escape]
187    | ^^^^^^^^^^^^^^^


200 LL | #![no_start]
202 
- warning: attribute should be applied to a function or static
- warning: attribute should be applied to a function or static
+ warning: attribute should be applied to a free function, impl method or static
205    |
205    |
206 LL |   #[no_mangle]
223    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
224 
225 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:464:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:464:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:1
227    |
228 LL |   #[cold]

240    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
241 
242 warning: attribute should be applied to a foreign function or static
---

297    |
298    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
299 
- warning: attribute should be applied to a function or static
+ warning: attribute should be applied to a free function, impl method or static
302    |
302    |
303 LL |     mod inner { #![no_mangle] }
305    |
306    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
307 
- warning: attribute should be applied to a function or static
- warning: attribute should be applied to a function or static
+ warning: attribute should be applied to a free function, impl method or static
310    |
310    |
311 LL |     #[no_mangle] struct S;
313    |
314    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
315 
- warning: attribute should be applied to a function or static
- warning: attribute should be applied to a function or static
+ warning: attribute should be applied to a free function, impl method or static
318    |
318    |
319 LL |     #[no_mangle] type T = S;
321    |
322    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
323 
- warning: attribute should be applied to a function or static
- warning: attribute should be applied to a function or static
+ warning: attribute should be applied to a free function, impl method or static
326    |
326    |
327 LL |     #[no_mangle] impl S { }
330    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
331 
331 
332 warning: attribute should be applied to a free function, impl method or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:360:9
334    |
334    |
335 LL |         #[no_mangle] fn foo();
336    |         ^^^^^^^^^^^^ --------- not a free function, impl method or static
338    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
339 
339 
340 warning: attribute should be applied to a free function, impl method or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:9
342    |
342    |
343 LL |         #[no_mangle] fn bar() {}
344    |         ^^^^^^^^^^^^ ----------- not a free function, impl method or static
346    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
347 
348 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:470:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:470:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:482:17
350    |
351 LL |     mod inner { #![cold] }
352    |     ------------^^^^^^^^-- not a function
354    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
355 
356 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:477:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:477:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:489:5
358    |
359 LL |     #[cold] struct S;

362    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
363 
364 warning: attribute should be applied to a function
364 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:482:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:494:5
366    |
367 LL |     #[cold] type T = S;

370    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
371 
372 warning: attribute should be applied to a function
372 warning: attribute should be applied to a function
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:487:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:499:5
374    |
375 LL |     #[cold] impl S { }

378    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
379 
380 warning: attribute should be applied to a foreign function or static
---
384    |     ^^^^^^^^^^^^^^^^^^^^^

388    |
389    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
390 help: try `#[link(name = "1900")]` instead
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:499:5
392    |
393 LL |     #[link_name = "1900"]
394    |     ^^^^^^^^^^^^^^^^^^^^^


395 
396 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:506:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:518:17
398    |
399 LL |     mod inner { #![link_name="1900"] }
400    |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
402    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
403 
404 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:511:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:511:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:523:5
406    |
407 LL |     #[link_name = "1900"] fn f() { }

410    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
411 
412 warning: attribute should be applied to a foreign function or static
412 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:516:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:528:5
414    |
415 LL |     #[link_name = "1900"] struct S;

418    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
419 
420 warning: attribute should be applied to a foreign function or static
420 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:521:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:533:5
422    |
423 LL |     #[link_name = "1900"] type T = S;

426    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
427 
428 warning: attribute should be applied to a foreign function or static
428 warning: attribute should be applied to a foreign function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:526:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:538:5
430    |
431 LL |     #[link_name = "1900"] impl S { }

434    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
435 
436 warning: attribute should be applied to a function or static
436 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:538:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:550:17
438    |
439 LL |     mod inner { #![link_section="1800"] }
440    |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
442    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
443 
444 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:545:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:545:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:557:5
446    |
447 LL |     #[link_section = "1800"] struct S;

450    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
451 
452 warning: attribute should be applied to a function or static
452 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:550:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:562:5
454    |
455 LL |     #[link_section = "1800"] type T = S;

458    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
459 
460 warning: attribute should be applied to a function or static
460 warning: attribute should be applied to a function or static
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:555:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:567:5
462    |
463 LL |     #[link_section = "1800"] impl S { }

612    |     ^^^^^^^^^^^^^^^^^^^^^^^^
613 
614 warning: unused attribute
614 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:325:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:337:1
616    |
617 LL | #[should_panic]

619 
620 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:328:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:328:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:340:17
622    |
623 LL |     mod inner { #![should_panic] }

625 
626 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:331:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:331:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:343:5
628    |
629 LL |     #[should_panic] fn f() { }

631 
632 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:334:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:334:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:346:5
634    |
635 LL |     #[should_panic] struct S;

637 
638 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:337:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:337:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:349:5
640    |
641 LL |     #[should_panic] type T = S;

643 
644 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:340:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:340:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:352:5
646    |
647 LL |     #[should_panic] impl S { }

649 
650 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:344:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:344:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:356:1
652    |
653 LL | #[ignore]

655 
656 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:347:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:347:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:359:17
658    |
659 LL |     mod inner { #![ignore] }

661 
662 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:350:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:350:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:362:5
664    |
665 LL |     #[ignore] fn f() { }

667 
668 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:353:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:353:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:5
670    |
671 LL |     #[ignore] struct S;

673 
674 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:356:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:356:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:368:5
676    |
677 LL |     #[ignore] type T = S;

679 
680 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:359:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:359:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:371:5
682    |
683 LL |     #[ignore] impl S { }

685 
686 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:363:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:363:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:375:1
688    |
689 LL | #[no_implicit_prelude]

691 
692 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:366:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:366:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:378:17
694    |
695 LL |     mod inner { #![no_implicit_prelude] }

697 
698 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:369:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:369:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:381:5
700    |
701 LL |     #[no_implicit_prelude] fn f() { }

703 
704 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:372:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:372:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:384:5
706    |
707 LL |     #[no_implicit_prelude] struct S;

709 
710 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:375:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:375:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:387:5
712    |
713 LL |     #[no_implicit_prelude] type T = S;

715 
716 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:378:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:378:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:390:5
718    |
719 LL |     #[no_implicit_prelude] impl S { }

721 
722 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:382:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:382:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:394:1
724    |
725 LL | #[reexport_test_harness_main = "2900"]

727 
728 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:385:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:385:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:397:17
730    |
731 LL |     mod inner { #![reexport_test_harness_main="2900"] }

733 
734 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:388:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:388:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:400:5
736    |
737 LL |     #[reexport_test_harness_main = "2900"] fn f() { }

739 
740 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:391:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:391:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:403:5
742    |
743 LL |     #[reexport_test_harness_main = "2900"] struct S;

745 
746 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:394:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:394:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:406:5
748    |
749 LL |     #[reexport_test_harness_main = "2900"] type T = S;

751 
752 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:397:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:397:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:409:5
754    |
755 LL |     #[reexport_test_harness_main = "2900"] impl S { }

757 
758 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:409:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:409:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:5
760    |
761 LL |     #[macro_escape] fn f() { }

763 
764 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:412:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:412:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:424:5
766    |
767 LL |     #[macro_escape] struct S;

769 
770 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:415:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:415:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:427:5
772    |
773 LL |     #[macro_escape] type T = S;

775 
776 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:418:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:418:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
778    |
779 LL |     #[macro_escape] impl S { }

781 
782 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:422:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:422:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:1
784    |
785 LL | #[no_std]

787 
787 
788 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:422:1
790    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
791 LL | #[no_std]

793 
794 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:17
796    |
797 LL |     mod inner { #![no_std] }

799 
800 warning: crate-level attribute should be in the root module
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:426:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:17
802    |
803 LL |     mod inner { #![no_std] }

805 
806 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:442:5
808    |
809 LL |     #[no_std] fn f() { }

811 
811 
812 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
814    |
814    |
815 LL |     #[no_std] fn f() { }

817 
818 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:446:5
820    |
821 LL |     #[no_std] struct S;

823 
823 
824 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:5
826    |
826    |
827 LL |     #[no_std] struct S;

829 
830 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:450:5
832    |
833 LL |     #[no_std] type T = S;

835 
835 
836 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:5
838    |
838    |
839 LL |     #[no_std] type T = S;

841 
842 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:442:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:442:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:454:5
844    |
845 LL |     #[no_std] impl S { }

847 
847 
848 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:442:5
850    |
850    |
851 LL |     #[no_std] impl S { }

853 
854 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:623:1
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:623:1
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:1
856    |
857 LL | #[crate_name = "0900"]

859 
859 
860 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:623:1
862    |
862    |
863 LL | #[crate_name = "0900"]

865 
866 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:17
868    |
869 LL |     mod inner { #![crate_name="0900"] }

871 
872 warning: crate-level attribute should be in the root module
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:627:17
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:17
874    |
875 LL |     mod inner { #![crate_name="0900"] }

877 
878 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:643:5
880    |
881 LL |     #[crate_name = "0900"] fn f() { }

883 
883 
884 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:5
886    |
886    |
887 LL |     #[crate_name = "0900"] fn f() { }

889 
890 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:647:5
892    |
893 LL |     #[crate_name = "0900"] struct S;

895 
895 
896 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:5
898    |
898    |
899 LL |     #[crate_name = "0900"] struct S;

901 
902 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:651:5
904    |
905 LL |     #[crate_name = "0900"] type T = S;

907 
907 
908 warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:5
910    |
910    |
911 LL |     #[crate_name = "0900"] type T = S;

913 
914 warning: unused attribute
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:643:5
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:643:5
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:655:5
916    |
917 LL |     #[crate_name = "0900"] impl S { }

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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:104:8
   |
   |
LL | #[warn(x5400)]

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:107:25
   |
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:110:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:113:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:116:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:119:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:123:9
   |
   |
LL | #[allow(x5300)]

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:126:26
   |
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:129:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:132:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:135:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:138:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:142:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:145:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:148:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:151:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:154:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:157:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:161:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:164:25
   |
LL |     mod inner { #![deny(x5100)] }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:167:12
   |
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:170:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:173:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:176:12
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


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |   #[no_mangle]
...
...
LL | / mod no_mangle {
LL | |     //~^ NOTE not a free function, impl method or static
LL | |     mod inner { #![no_mangle] }
LL | |     //~^ WARN attribute should be applied to a free function, impl method or static [unused_attributes]
LL | |     }
LL | | }
LL | | }
   | |_- not a free function, impl method or static
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:9
   |
LL | #![warn(unused_attributes, unknown_lints)]
LL | #![warn(unused_attributes, unknown_lints)]
   |         ^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:505:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:505:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:1
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


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |     mod inner { #![no_mangle] }
   |     ------------^^^^^^^^^^^^^-- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[no_mangle] struct S;
   |     ^^^^^^^^^^^^ --------- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[no_mangle] type T = S;
   |     ^^^^^^^^^^^^ ----------- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |     #[no_mangle] impl S { }
   |     ^^^^^^^^^^^^ ---------- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |         #[no_mangle] fn foo();
   |         ^^^^^^^^^^^^ --------- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!


warning: attribute should be applied to a free function, impl method or static
   |
   |
LL |         #[no_mangle] fn bar() {}
   |         ^^^^^^^^^^^^ ----------- not a free function, impl method or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:482:17
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:482:17
   |
LL |     mod inner { #![cold] }
   |     ------------^^^^^^^^-- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:489:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:489:5
   |
LL |     #[cold] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:494:5
   |
LL |     #[cold] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:499:5
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:518:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:523:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:523:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:528:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:533:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:538:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:550:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:557:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:557:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:562:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:567:5
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:337:1
   |
   |
LL | #[should_panic]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:340:17
   |
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:343:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:346:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:349:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:352:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:356:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:359:17
   |
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:362:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:365:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:368:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:371:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:375:1
   |
   |
LL | #[no_implicit_prelude]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:378:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:381:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:384:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:387:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:390:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:394:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:397:17
   |
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:400:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:403:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:406:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:409:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:421:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:424:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:427:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:430:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:434:1
   |
   |
LL | #[no_std]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[no_std]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:438:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:438:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:442:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:446:5
   |
   |
LL |     #[no_std] struct S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:450:5
   |
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:454:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:635:1
   |
   |
LL | #[crate_name = "0900"]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[crate_name = "0900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:639:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:639:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:643:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
---
test result: FAILED. 12028 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 125.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:40
