
#0  0x00007f433e6d9910 in LLVMTypeOf () from /rust/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_llvm-97d4717776475ef1.so
#1  0x00007f4343a386e9 in rustc_trans::base::from_immediate (bcx=0x7f4339d27ea0, val=0x0) at /rust/src/librustc_trans/base.rs:464
    at /rust/src/librustc_trans/mir/operand.rs:340
    at /rust/src/librustc_trans/mir/rvalue.rs:161
    at /rust/src/librustc_trans/mir/statement.rs:35
#5  rustc_trans::mir::block::{{impl}}::trans_block (self=<optimized out>, bb=..., funclets=<optimized out>) at /rust/src/librustc_trans/mir/block.rs:109
    at /rust/src/librustc_trans/mir/mod.rs:330
#7  0x00007f4343a38ff5 in rustc_trans::base::trans_instance (ccx=0x7f4339d2cf38, instance=...) at /rust/src/librustc_trans/base.rs:606
#8  0x00007f4343ade88d in rustc_trans::trans_item::{{impl}}::define (self=0x7f4339d2c880, ccx=<optimized out>) at /rust/src/librustc_trans/trans_item.rs:104
#9  0x00007f4343a3f5b0 in rustc_trans::base::trans_crate::module_translation (scx=..., args=...) at /rust/src/librustc_trans/base.rs:1185
    task=0x7f4343a3ee10 <rustc_trans::base::trans_crate::module_translation>) at /rust/src/librustc/dep_graph/graph.rs:111
#11 0x00007f4343a29a76 in rustc_trans::base::trans_crate::{{closure}} (cgu=...) at /rust/src/librustc_trans/base.rs:1119
#12 core::ops::impls::{{impl}}::call_once<(rustc_trans::partitioning::CodegenUnit),closure> (self=0x7f4339d2d540, args=...) at /rust/src/libcore/ops.rs:2674
    at /rust/src/libcore/option.rs:392
    at /rust/src/libcore/iter/mod.rs:1011
    self=<optimized out>, iterator=...) at /rust/src/libcollections/vec.rs:1761
    iterator=...) at /rust/src/libcollections/vec.rs:1744
#17 0x00007f4343a3bc9e in collections::vec::{{impl}}::from_iter<rustc_trans::ModuleTranslation,core::iter::Map<collections::vec::IntoIter<rustc_trans::partitioning::CodegenUnit>, closure>> (iter=...) at /rust/src/libcollections/vec.rs:1631
#18 core::iter::iterator::Iterator::collect<core::iter::Map<collections::vec::IntoIter<rustc_trans::partitioning::CodegenUnit>, closure>,collections::vec::Vec<rustc_trans::ModuleTranslation>> (self=...) at /rust/src/libcore/iter/iterator.rs:1222
#19 rustc_trans::base::trans_crate (tcx=..., analysis=..., incremental_hashes_map=<optimized out>) at /rust/src/librustc_trans/base.rs:1114
#20 0x00007f434481e106 in rustc_driver::driver::phase_4_translate_to_llvm::{{closure}} () at /rust/src/librustc_driver/driver.rs:1080
#21 rustc::util::common::time<rustc_trans::CrateTranslation,closure> (do_it=<optimized out>, what=..., f=...) at /rust/src/librustc/util/common.rs:57
    at /rust/src/librustc_driver/driver.rs:1078
    at /rust/src/librustc_driver/driver.rs:206
#24 0x00007f434485ca21 in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (tcx=...) at /rust/src/librustc_driver/driver.rs:1024
#25 0x00007f434480d6ef in rustc::ty::context::tls::enter::{{closure}}<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (tls=0x7f4339d40570) at /rust/src/librustc/ty/context.rs:921
    f=...) at /rust/src/libstd/thread/local.rs:253
    gcx=<optimized out>, interners=<optimized out>, f=...) at /rust/src/librustc/ty/context.rs:918
