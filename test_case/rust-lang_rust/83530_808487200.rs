plain
    Checking cranelift-frontend v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-jit v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-object v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
  |
4 |     type_alias_impl_trait,
  |     ^^^^^^^^^^^^^^^^^^^^^
  |
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking core v0.0.0 (/checkout/library/core)
error: trailing semicolon in macro used in expression position
  --> /checkout/library/std/src/../../stdarch/crates/std_detect/src/detect/macros.rs:21:65
   |
14 | /         macro_rules! $macro_name {
15 | |             $(
16 | |                 ($feature_lit) => {
17 | |                     $crate::detect::__is_feature_detected::$feature()
...  |
21 | |                 ($bind_feature) => { $macro_name!($feature_impl); };
...  |
43 | |             };
44 | |         }
44 | |         }
   | |_________- in this expansion of `is_x86_feature_detected!`
  ::: library/std/tests/run-time-detect.rs:60:27
   |
   |
60 |       println!("abm: {:?}", is_x86_feature_detected!("abm")); // this is a synonym for lzcnt but we test it anyways
   |
   |
   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error: aborting due to previous error


error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:426:14
    |
426 |     let () = metadata(&());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:427:14
    |
    |
427 |     let () = metadata(&Unit);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:428:14
    |
    |
428 |     let () = metadata(&4_u32);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:429:14
    |
    |
429 |     let () = metadata(&String::new());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:430:14
    |
    |
430 |     let () = metadata(&Some(4_u32));
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:431:14
    |
    |
431 |     let () = metadata(&ptr_metadata);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:432:14
    |
    |
432 |     let () = metadata(&|| {});
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:433:14
    |
    |
433 |     let () = metadata(&[4, 7]);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:434:14
    |
    |
434 |     let () = metadata(&(4, String::new()));
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:435:14
    |
    |
435 |     let () = metadata(&Pair(4, String::new()));
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:436:14
    |
    |
436 |     let () = metadata(0 as *const Extern);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:437:14
    |
    |
437 |     let () = metadata(0 as *const <&u32 as std::ops::Deref>::Target);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:439:16
    |
    |
439 |     assert_eq!(metadata("foo"), 3_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:440:16
    |
    |
440 |     assert_eq!(metadata(&[4, 7][..]), 2_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:444:16
    |
    |
444 |     assert_eq!(metadata(dst_tuple), 3_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:445:16
    |
    |
445 |     assert_eq!(metadata(dst_struct), 3_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:451:20
    |
    |
451 |         assert_eq!(metadata(dst_tuple), 3_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:452:20
    |
    |
452 |         assert_eq!(metadata(dst_struct), 3_usize);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:455:44
    |
    |
455 |     let vtable_1: DynMetadata<dyn Debug> = metadata(&4_u16 as &dyn Debug);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:455:19
    |
    |
455 |     let vtable_1: DynMetadata<dyn Debug> = metadata(&4_u16 as &dyn Debug);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:456:46
    |
    |
456 |     let vtable_2: DynMetadata<dyn Display> = metadata(&4_u16 as &dyn Display);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:456:19
    |
    |
456 |     let vtable_2: DynMetadata<dyn Display> = metadata(&4_u16 as &dyn Display);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:457:46
    |
    |
457 |     let vtable_3: DynMetadata<dyn Display> = metadata(&4_u32 as &dyn Display);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:457:19
    |
    |
457 |     let vtable_3: DynMetadata<dyn Display> = metadata(&4_u32 as &dyn Display);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:458:46
    |
    |
458 |     let vtable_4: DynMetadata<dyn Display> = metadata(&(true, 7_u32) as &(bool, dyn Display));
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:458:19
    |
    |
458 |     let vtable_4: DynMetadata<dyn Display> = metadata(&(true, 7_u32) as &(bool, dyn Display));
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:460:9
    |
    |
460 |         metadata(&Pair(true, 7_u32) as &Pair<bool, dyn Display>);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:459:19
    |
    |
459 |     let vtable_5: DynMetadata<dyn Display> =
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:495:58
    |
    |
495 |     let _ = static_assert_expected_bounds_for_metadata::<DynMetadata<dyn Display>>;
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:481:10
    |
    |
481 |         <<T as Pointee>::Metadata as PartialEq>::eq as usize
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:497:62
    |
    |
497 |         let _ = static_assert_expected_bounds_for_metadata::<<T as Pointee>::Metadata>;
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:516:16
    |
    |
516 |     let meta = metadata(trait_object);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:532:18
    |
    |
532 |     let vtable = metadata(trait_object);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:535:16
    |
    |
535 |     assert_eq!(ptr::from_raw_parts(address, vtable), trait_object.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:536:16
    |
    |
536 |     assert_eq!(ptr::from_raw_parts_mut(address, vtable), trait_object.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:544:16
    |
    |
544 |     assert_eq!(ptr::from_raw_parts(address, ()), array_ptr.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:545:16
    |
    |
545 |     assert_eq!(ptr::from_raw_parts_mut(address, ()), array_ptr.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:548:16
    |
    |
548 |     assert_eq!(ptr::from_raw_parts(address, 5), slice_ptr.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:549:16
    |
    |
549 |     assert_eq!(ptr::from_raw_parts_mut(address, 5), slice_ptr.as_ptr());
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:575:21
    |
    |
575 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:575:40
    |
    |
575 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:577:22
    |
    |
577 |         ptr: NonNull<DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:583:21
    |
    |
583 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:583:40
    |
    |
583 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:632:21
    |
    |
632 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:632:40
    |
    |
632 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:643:21
    |
    |
643 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:643:40
    |
    |
643 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:652:21
    |
    |
652 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:652:40
    |
    |
652 |         T: ?Sized + Pointee<Metadata = DynMetadata<T>>,
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:587:24
    |
    |
587 |             let meta = metadata(unsized_);
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:599:29
    |
    |
599 |                     .cast::<DynMetadata<T>>();
    |
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = note: see issue #81513 <https://github.com/rust-lang/rust/issues/81513> for more information
    = help: add `#![feature(ptr_metadata)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'ptr_metadata'
   --> library/core/tests/ptr.rs:606:27
