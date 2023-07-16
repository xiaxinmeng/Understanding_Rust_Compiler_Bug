
Program received signal EXC_BAD_ACCESS, Could not access memory.
Reason: 13 at address: 0x0000000000000000
[Switching to process 79665 thread 0x20b]
0x0000000100d59910 in middle::typeck::check::vtable::lookup_vtable::_f27853cefd4c1615::_06 ()
(gdb) bt
#0  0x0000000100d59910 in middle::typeck::check::vtable::lookup_vtable::_f27853cefd4c1615::_06 ()
#1  0x000000000064905c in ?? ()
#2  0x0000000100d57887 in middle::typeck::check::vtable::lookup_vtables::anon::anon::expr_fn_49204 ()
#3  0x0000000100ca9873 in middle::ty::iter_bound_traits_and_supertraits::anon::expr_fn_43543 ()
#4  0x0000000100d5728e in middle::typeck::check::vtable::lookup_vtables::anon::expr_fn_49200 ()
#5  0x0000000100d5626e in middle::typeck::check::vtable::lookup_vtables::_51bba2be11ce71c8::_06 ()
#6  0x0000000100d619dc in middle::typeck::check::vtable::early_resolve_expr::_ae91f2998bf8b031::_06 ()
#7  0x0000000100e1826d in middle::typeck::check::check_expr_with_unifier::check_call_inner::_77d3ca6f8eb99f58::_06 ()
#8  0x0000000100e19d9f in middle::typeck::check::check_expr_with_unifier::check_call_or_method::_381ad5e839f2382::_06 ()
#9  0x0000000100e09cf4 in middle::typeck::check::check_expr_with_unifier::_84cb5aba41d6e5b::_06 ()
#10 0x0000000100e1e21a in middle::typeck::check::check_decl_local::_7d93465d482f4::_06 ()
#11 0x0000000100e31803 in middle::typeck::check::check_stmt::_c068f9e8a7ae43ab::_06 ()
#12 0x0000000100e31c33 in middle::typeck::check::check_block_with_expected::anon::anon::expr_fn_54202 ()
#13 0x0000000100e2f40e in middle::typeck::check::check_block_with_expected::_f7fee21f1eb5e7e7::_06 ()
#14 0x0000000100db31a6 in middle::typeck::check::check_fn::_a5807fb2b5df854::_06 ()
#15 0x0000000101201e44 in __morestack ()
Previous frame inner to this frame (gdb could not unwind past this frame)
