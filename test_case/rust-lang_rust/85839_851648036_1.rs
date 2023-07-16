

(gdb) bt
#0  <nanoda_lib::env::Declar as core::fmt::Debug>::fmt (self=0x1, f=0x7fffffffc2d0)
    at src/env.rs:236
#1  0x00005555555cc63f in core::fmt::write () at library/core/src/fmt/mod.rs:1092
#2  0x00005555555cab9c in core::fmt::Write::write_fmt ()
    at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/fmt/mod.rs:182
#3  alloc::fmt::format () at library/alloc/src/fmt.rs:578
#4  0x0000555555595fcd in nanoda_lib::tc::reduce::reduce_ind_rec (c_name=..., c_levels=..., 
    args=..., tc=0x7fffffffd2b0) at src/tc/reduce.rs:169
#5  0x000055555558a4ce in nanoda_lib::tc::reduce::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::whnf_core::{{closure}} () at src/tc/reduce.rs:22
#6  core::option::Option<T>::or_else (self=..., f=...)
    at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/option.rs:818
#7  nanoda_lib::tc::reduce::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::whnf_core (self=..., tc=0x7fffffffd2b0) at src/tc/reduce.rs:22
#8  0x000055555558671a in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::delta_step (self=..., other=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:222
#9  0x0000555555587a69 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:295
#10 0x000055555556bf89 in nanoda_lib::tc::eq::args_eq_aux (lhs=..., rhs=..., 
    tc=<optimized out>) at src/tc/eq.rs:39
#11 0x0000555555587d67 in nanoda_lib::tc::eq::args_eq (lhs=..., rhs=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:27
#12 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_app (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:81
#13 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:302
#14 0x000055555556bf89 in nanoda_lib::tc::eq::args_eq_aux (lhs=..., rhs=..., 
    tc=<optimized out>) at src/tc/eq.rs:39
#15 0x000055555558722b in nanoda_lib::tc::eq::args_eq (lhs=..., rhs=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:27
#16 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::delta_step (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:251
#17 0x0000555555587a69 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:295
#18 0x000055555556bf89 in nanoda_lib::tc::eq::args_eq_aux (lhs=..., rhs=..., 
    tc=<optimized out>) at src/tc/eq.rs:39
#19 0x0000555555587d67 in nanoda_lib::tc::eq::args_eq (lhs=..., rhs=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:27
#20 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_app (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:81
#21 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:302
#22 0x0000555555585b3c in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_pi_aux (self=..., other=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:143
#23 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_pi (self=..., other=..., tc=<optimized out>) at src/tc/eq.rs:120
#24 0x00005555555878f9 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:286
#25 0x000055555556bf89 in nanoda_lib::tc::eq::args_eq_aux (lhs=..., rhs=..., 
    tc=<optimized out>) at src/tc/eq.rs:39
#26 0x000055555558722b in nanoda_lib::tc::eq::args_eq (lhs=..., rhs=..., tc=0x7fffffffd2b0)
    at src/tc/eq.rs:27
#27 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::delta_step (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:251
#28 0x0000555555587a69 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0)
--Type <RET> for more, q to quit, c to continue without paging--c
    at src/tc/eq.rs:295
#29 0x0000555555585b3c in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_pi_aux (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:143
#30 nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq_pi (self=..., other=..., tc=<optimized out>) at src/tc/eq.rs:120
#31 0x00005555555878f9 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::def_eq (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:286
#32 0x0000555555587fd6 in nanoda_lib::tc::eq::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::assert_def_eq (self=..., other=..., tc=0x7fffffffd2b0) at src/tc/eq.rs:274
#33 nanoda_lib::tc::infer::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::infer_app (self=..., flag=<optimized out>, tc=0x7fffffffd2b0) at src/tc/infer.rs:74
#34 0x0000555555588e44 in nanoda_lib::tc::infer::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::infer (self=..., flag=nanoda_lib::tc::infer::InferFlag::Check, tc=0x7fffffffd2b0) at src/tc/infer.rs:219
#35 0x0000555555589c1d in nanoda_lib::tc::infer::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::infer_lambda (self=..., flag=<optimized out>, tc=0x7fffffffd2b0) at src/tc/infer.rs:145
#36 nanoda_lib::tc::infer::<impl nanoda_lib::utils::Ptr2<core::marker::PhantomData<&nanoda_lib::expr::Expr>>>::infer (self=..., flag=<optimized out>, tc=0x7fffffffd2b0) at src/tc/infer.rs:221
#37 0x000055555556f3ca in nanoda_lib::env::Declar::check (self=..., _should_check=<optimized out>, live=<optimized out>) at src/env.rs:628
#38 0x000055555558de19 in nanoda_lib::parser::<impl nanoda_lib::utils::Env>::check_loop (self=0x7fffffffda60, num_threads=<optimized out>) at src/parser.rs:331
#39 0x0000555555569e9e in nanoda_lib::parser::Parser::parser_loop (self=<optimized out>) at src/parser.rs:73
#40 0x000055555556658c in debug::main () at examples/debug.rs:29
(gdb) 

