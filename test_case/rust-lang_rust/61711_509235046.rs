
(gdb) bt
#0  0x00007ffff6ba3e7c in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query (self=..., span=...,
    key=...) at src/librustc/ty/query/plumbing.rs:350
#1  0x00007ffff68185bb in rustc::ty::query::TyCtxtAt::extern_crate (self=..., key=...)
    at src/librustc/ty/query/plumbing.rs:1074
#2  rustc::ty::query::<impl rustc::ty::context::TyCtxt>::extern_crate (self=..., key=...)
    at src/librustc/ty/query/plumbing.rs:1066
#3  rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:257
#4  0x00007ffff68187d4 in rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:309
#5  0x00007ffff68187d4 in rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:309
#6  0x00007ffff68187d4 in rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:309
#7  0x00007ffff68187d4 in rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:309
#8  0x00007ffff68187d4 in rustc::ty::print::pretty::PrettyPrinter::try_print_visible_def_path (self=..., def_id=...)
    at src/librustc/ty/print/pretty.rs:309
