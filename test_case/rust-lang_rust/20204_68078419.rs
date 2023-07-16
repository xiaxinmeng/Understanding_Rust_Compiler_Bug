
==48842== Memcheck, a memory error detector
==48842== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==48842== Using Valgrind-3.11.0.SVN and LibVEX; rerun with -h for copyright info
==48842== Command: /usr/local/bin/rustc bad.best
==48842==
--48842-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option
--48842-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 2 times)
--48842-- UNKNOWN mach_msg unhandled MACH_SEND_TRAILER option (repeated 4 times)
bad.best:3:55: 3:65 error: wrong number of type arguments: expected 1, found 0
bad.best:3  fn builder<'a, T: ToIdent>(ctx: &'a Ctx, name: T) -> ArgBuilder { }
                                                                 ^~~~~~~~~~
==48842== Thread 2:
==48842== Invalid read of size 8
==48842==    at 0x1008AF013: astconv::ty_of_bare_fn::h7087483585960393220 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x10077560D: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AF7D8: rt::unwind::try::try_fn::h9666833473335668351 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038D1C23: sys::thread::thread_start::h7394d69fbc5b85e9vFw (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==  Address 0x106050b10 is 16 bytes after a recently re-allocated block of size 128 alloc'd
==48842==    at 0x10391134D: je_mallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100F6D498: middle::traits::util::poly_trait_ref_for_builtin_bound::h443b614316e2f885QcT (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x100FD057E: middle::ty::predicates::hc6c79523cea8d690a55 (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x1007E3AA7: collect::ty_generics::create_predicates::h341fb82081530039YRv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100775201: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==
==48842== Invalid read of size 8
==48842==    at 0x1008AF01C: astconv::ty_of_bare_fn::h7087483585960393220 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x10077560D: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AF7D8: rt::unwind::try::try_fn::h9666833473335668351 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038D1C23: sys::thread::thread_start::h7394d69fbc5b85e9vFw (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==  Address 0x106050b00 is 0 bytes after a recently re-allocated block of size 128 alloc'd
==48842==    at 0x10391134D: je_mallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100F6D498: middle::traits::util::poly_trait_ref_for_builtin_bound::h443b614316e2f885QcT (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x100FD057E: middle::ty::predicates::hc6c79523cea8d690a55 (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x1007E3AA7: collect::ty_generics::create_predicates::h341fb82081530039YRv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100775201: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==
==48842== Invalid free() / delete / delete[] / realloc()
==48842==    at 0x1039273E7: je_valgrind_freelike_block (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1039143BA: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1008AF025: astconv::ty_of_bare_fn::h7087483585960393220 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x10077560D: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AF7D8: rt::unwind::try::try_fn::h9666833473335668351 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==  Address 0x10607ee38 is 16 bytes after a recently re-allocated block of size 8 alloc'd
==48842==    at 0x10391134D: je_mallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1007AB4D1: middle::ty::TypeParameterDef$LT$$u{27}tcx$GT$...std..clone..Clone::clone::h5ac3ac9e9952e95bNh1 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x10089C89C: collect::get_or_create_type_parameter_def::h5154923215161465272 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100774C5F: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AF7D8: rt::unwind::try::try_fn::h9666833473335668351 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==
==48842== Invalid free() / delete / delete[] / realloc()
==48842==    at 0x1039273E7: je_valgrind_freelike_block (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1039143BA: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1008AF03F: astconv::ty_of_bare_fn::h7087483585960393220 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x10077560D: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AF7D8: rt::unwind::try::try_fn::h9666833473335668351 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==  Address 0x106050b00 is 0 bytes after a recently re-allocated block of size 128 alloc'd
==48842==    at 0x10391134D: je_mallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100F6D498: middle::traits::util::poly_trait_ref_for_builtin_bound::h443b614316e2f885QcT (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x100FD057E: middle::ty::predicates::hc6c79523cea8d690a55 (in /usr/local/lib/librustc-4e7c5e5c.dylib)
==48842==    by 0x1007E3AA7: collect::ty_generics::create_predicates::h341fb82081530039YRv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100775201: collect::ty_of_item::h0fb83933bf7ef09ebjv (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100765C13: collect::convert::h0969c664bcd7a0bbkTu (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100953402: check_crate::unboxed_closure.43401 (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100951489: check_crate::hcab218417b8f1f7f51y (in /usr/local/lib/librustc_typeck-4e7c5e5c.dylib)
==48842==    by 0x100031112: driver::phase_3_run_analysis_passes::hb0ac12e829cd00e7Jta (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x100012FFD: driver::compile_input::hdcb29128a43ec322wba (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B2FF7: thunk::Thunk$LT$$LP$$RP$$C$$u{20}R$GT$::new::unboxed_closure.30336 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B1037: thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h12480167585331118411 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AEBB6: std..comm..Sender$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30105::hc3ea1a969cbfbeec (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B3D1A: Box$LT$std..io..comm_adapters..ChanWriter$GT$::glue_drop.30342::h968132b92a9d85ae (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10389C4B9: thread_local::imp::destroy_value::h3539758032154077484 (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103EBC16C: tlv_finalize (in /usr/lib/system/libdyld.dylib)
==48842==    by 0x1041707BD: _pthread_tsd_cleanup (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x1041704E4: _pthread_exit (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F306: _pthread_body (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F278: _pthread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416D4B0: thread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AEBC0: std..comm..Sender$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30105::hc3ea1a969cbfbeec (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B3D1A: Box$LT$std..io..comm_adapters..ChanWriter$GT$::glue_drop.30342::h968132b92a9d85ae (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10389C4B9: thread_local::imp::destroy_value::h3539758032154077484 (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103EBC16C: tlv_finalize (in /usr/lib/system/libdyld.dylib)
==48842==    by 0x1041707BD: _pthread_tsd_cleanup (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x1041704E4: _pthread_exit (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F306: _pthread_body (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F278: _pthread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416D4B0: thread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AECE4: std..comm..Sender$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30105::hc3ea1a969cbfbeec (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B3D1A: Box$LT$std..io..comm_adapters..ChanWriter$GT$::glue_drop.30342::h968132b92a9d85ae (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10389C4B9: thread_local::imp::destroy_value::h3539758032154077484 (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103EBC16C: tlv_finalize (in /usr/lib/system/libdyld.dylib)
==48842==    by 0x1041707BD: _pthread_tsd_cleanup (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x1041704E4: _pthread_exit (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F306: _pthread_body (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F278: _pthread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416D4B0: thread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001ADEC3: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001B3D1A: Box$LT$std..io..comm_adapters..ChanWriter$GT$::glue_drop.30342::h968132b92a9d85ae (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10389C4B9: thread_local::imp::destroy_value::h3539758032154077484 (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103EBC16C: tlv_finalize (in /usr/lib/system/libdyld.dylib)
==48842==    by 0x1041707BD: _pthread_tsd_cleanup (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x1041704E4: _pthread_exit (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F306: _pthread_body (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416F278: _pthread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==    by 0x10416D4B0: thread_start (in /usr/lib/system/libsystem_pthread.dylib)
==48842==
==48842== Thread 1:
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001ACF30: comm::Receiver$LT$T$GT$.Drop::drop::h957455946670060499 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACDD5: std..comm..oneshot..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.29648::h2e0097800884deeb (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001ADEC3: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AE11E: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AE1B5: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AE2C9: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1001AE2F2: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1001AE2F7: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1001AE2FB: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x10391436A: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x103914384: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x10391415E: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x103914229: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA6DE: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA6FB: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA156: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA1B7: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA1C9: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA253: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA25B: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA2BE: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Use of uninitialised value of size 8
==48842==    at 0x1038FA2C5: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Conditional jump or move depends on uninitialised value(s)
==48842==    at 0x1038FA263: je_arena_dalloc_bin_locked (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1038FA715: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE2EE: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==
==48842== Invalid read of size 8
==48842==    at 0x1038FA6FB: je_arena_dalloc_small (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE342: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)
==48842==  Address 0x105feda38 is not stack'd, malloc'd or (recently) free'd
==48842==
--48842-- VALGRIND INTERNAL ERROR: Valgrind received a signal 11 (SIGSEGV) - exiting
--48842-- si_code=1;  Faulting address: 0x2380BE882;  sp: 0x700000aa5af8

valgrind: the 'impossible' happened:
   Killed by fatal signal

host stacktrace:
==48842==    at 0x238059741: ???
==48842==    by 0x2380BEA05: ???
==48842==    by 0x2380B538C: ???
==48842==    by 0x2380B3FC8: ???
==48842==    by 0x2380B1FE7: ???
==48842==    by 0x2380C311E: ???

sched status:
  running_tid=1

Thread 1: status = VgTs_Runnable
==48842==    at 0x104059C7E: __kill (in /usr/lib/system/libsystem_kernel.dylib)
==48842==    by 0x23805B6F8: ???
==48842==    by 0x103914233: je_sdallocx (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x1001AE342: std..comm..stream..Packet$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30028::hee87528c8aeecad1 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ADF2A: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001AE0AA: std..comm..Flavor$LT$collections..vec..Vec$LT$u8$GT$$GT$::glue_drop.30016::h36a67bdc7823f76f (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001ACBAB: monitor::h6149434868482590664 (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x1001BCCB9: main::h90fb4d85097008b1fVc (in /usr/local/lib/librustc_driver-4e7c5e5c.dylib)
==48842==    by 0x10394A668: rust_try_inner (in /usr/local/lib/libstd-4e7c5e5c.dylib)
==48842==    by 0x100001E23: (below main) (in /usr/local/bin/rustc)


Note: see also the FAQ in the source distribution.
It contains workarounds to several common problems.
In particular, if Valgrind aborted or crashed after
identifying problems in your program, there's a good chance
that fixing those problems will prevent Valgrind aborting or
crashing, especially if it happened in m_mallocfree.c.

If that doesn't help, please report this bug to: www.valgrind.org

In the bug report, send all the above text, the valgrind
version, and what OS and version you are using.  Thanks.
