
Thread 2 (Thread 0x7fffedfff700 (LWP 4305)):
#0  0x00007ffff48a0467 in middle::infer::region_inference::RegionVarBindings$LT$$u27$a$C$$u20$$u27$tcx$GT$::infer_variable_values::h13047075df6ab7ca6zA ()
   from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc-18402db3.so
#1  0x00007ffff484ec4a in middle::infer::region_inference::RegionVarBindings$LT$$u27$a$C$$u20$$u27$tcx$GT$::resolve_regions::hd1aae808037814e6lsA ()
   from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc-18402db3.so
#2  0x00007ffff484eb97 in middle::infer::InferCtxt$LT$$u27$a$C$$u20$$u27$tcx$GT$::resolve_regions_and_report_errors::ha3c7008b38ce75654hE ()
   from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc-18402db3.so
#3  0x00007ffff6b2825e in check::regionck::regionck_expr::hd7cd1f3071a82f93Wmc () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#4  0x00007ffff6b280a2 in check::check_const_with_ty::h313c7bb2ab876a2e07s () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#5  0x00007ffff6ad503a in check::check_const::h337b7a49255e2b0cK6s () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#6  0x00007ffff6abde1f in check::check_item_type::h8475073b43b488e4h4o () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#7  0x00007ffff6abd1d2 in check::CheckItemTypesVisitor$LT$$u27$a$C$$u20$$u27$tcx$GT$.Visitor$LT$$u27$tcx$GT$::visit_item::hc7dbddc6e361f3d3KHo ()
   from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#8  0x00007ffff6abbb5d in check::check_item_types::h5114724f2e7fe669LJo () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#9  0x00007ffff6ab3355 in check_crate::hcd126beee06d4b43IGC () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_typeck-18402db3.so
#10 0x00007ffff7a7fa1d in driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.30211 () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#11 0x00007ffff7a7e015 in middle::ty::context::TyCtxt$LT$$u27$tcx$GT$::create_and_enter::h17882283280588378896 () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#12 0x00007ffff7a7abf3 in driver::phase_3_run_analysis_passes::h13919317650711339916 () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#13 0x00007ffff7a4d3c0 in driver::compile_input::h148bbe225adb9061Pca () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#14 0x00007ffff7a3ab47 in run_compiler::h01319ef2912a48felQc () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#15 0x00007ffff7a382c2 in sys_common::unwind::try::try_fn::h15450783821430179622 () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#16 0x00007ffff753f47c in __rust_try () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/libstd-18402db3.so
#17 0x00007ffff753f40e in sys_common::unwind::inner_try::hfb91b5cfe185b795Ztu () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/libstd-18402db3.so
#18 0x00007ffff7a38b0b in boxed::F.FnBox$LT$A$GT$::call_box::h7115862480082857486 () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/librustc_driver-18402db3.so
#19 0x00007ffff7549e5a in sys::thread::Thread::new::thread_start::h33a79b9c7e674b88Huz () from /home/alex/.multirust/toolchains/nightly-2016-03-11/lib/libstd-18402db3.so
#20 0x00007fffefc9a182 in start_thread (arg=0x7fffedfff700) at pthread_create.c:312
#21 0x00007ffff71af47d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111
