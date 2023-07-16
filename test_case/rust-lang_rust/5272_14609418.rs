
(gdb) backtrace full
#0  rust_task_fail (task=0x7fffec10ba70, 
    expr=0x7ffff70f9830 "Assertion bpos == mbc.pos || bpos.to_uint() >= mbc.pos.to_uint() + mbc.bytes failed", 
    file=0x7ffff70f9580 "/home/tony/Projects/rust/src/libsyntax/codemap.rs", 
    line=500) at /home/tony/Projects/rust/src/rt/rust_task.cpp:85
No locals.
#1  0x00007ffff5ee2109 in __morestack () from /usr/local/bin/../lib/librustrt.so
No symbol table info available.
#2  0x00007ffff5ed2d78 in call_on_c_stack (fn_ptr=0x7ffff5ed26b0, 
    args=0x7fffe415f4c0, this=0x7fffec10ba70)
    at /home/tony/Projects/rust/src/rt/rust_task.h:475
        prev_rust_sp = 0
        borrowed_a_c_stack = true
        sp = <optimised out>
#3  call_upcall_on_c_stack (fn_ptr=0x7ffff5ed26b0, args=0x7fffe415f4c0, 
    task=<optimised out>) at /home/tony/Projects/rust/src/rt/rust_upcall.cpp:45
No locals.
#4  upcall_fail (expr=<optimised out>, 
    file=0x7ffff70f9580 "/home/tony/Projects/rust/src/libsyntax/codemap.rs", 
    line=500) at /home/tony/Projects/rust/src/rt/rust_upcall.cpp:123
        task = 0x7fffec10ba70
        args = {task = 0x7fffec10ba70, 
          expr = 0x7ffff70f9830 "Assertion bpos == mbc.pos || bpos.to_uint() >= mbc.---Type <return> to continue, or q <return> to quit---
pos.to_uint() + mbc.bytes failed", 
          file = 0x7ffff70f9580 "/home/tony/Projects/rust/src/libsyntax/codemap.rs", line = 500}
#5  0x00007ffff7a13a5b in sys::begin_unwind_::_15feb4cf285334c::_06 ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
No symbol table info available.
#6  0x00007ffff795c3fe in rt::rt_fail_::_f79ba45ac636dce::_06 ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
No symbol table info available.
#7  0x00007ffff6ee349e in codemap::__extensions__::meth_9251::bytepos_to_local_charpos::_2ef2f680debabbad::_06 ()
   from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#8  0x00007ffff6ee155b in codemap::__extensions__::meth_9111::lookup_pos::_fa9d322f72fd5316::_06 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#9  0x00007ffff6ee1c64 in codemap::__extensions__::meth_9113::lookup_char_pos_adj::_49afbe1de9c0f59::_06 ()
   from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#10 0x00007ffff6ede5b6 in codemap::__extensions__::meth_8514::span_to_str::_cac44644182bf88b::_06 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
---Type <return> to continue, or q <return> to quit---
#11 0x00007ffff6ed9fb5 in diagnostic::emit::_d2f47fc894c9320::_06 ()
   from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#12 0x00007ffff70f81b8 in __morestack ()
   from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#13 0x00007ffff69fd193 in monitor::anon::anon::expr_fn_89179 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#14 0x00007ffff6ed83bd in diagnostic::__extensions__::meth_8170::emit::_d2f47fc894c9320::_06 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#15 0x00007ffff6ed747e in diagnostic::__extensions__::meth_8120::span_warn::_2f48a8a4d3384::_06 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#16 0x00007ffff67ef93e in middle::liveness::__extensions__::warn_about_unused::anon::expr_fn_69339 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#17 0x00007ffff67ec174 in middle::liveness::__extensions__::meth_69212::warn_about_unused::_eaa396e49e408468::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#18 0x00007ffff67ef5b6 in middle::liveness::__extensions__::warn_about_unused_or_dea---Type <return> to continue, or q <return> to quit---
d_vars_in_pat::anon::expr_fn_69337 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#19 0x00007ffff67e33dc in middle::liveness::__extensions__::pat_bindings::anon::expr_fn_68834 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#20 0x00007ffff6814f2a in middle::pat_util::pat_bindings::anon::expr_fn_70517 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#21 0x00007ffff6f2a205 in ast_util::walk_pat::_b8bf7b22f55e85f0::_06 ()
   from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.6.so
No symbol table info available.
#22 0x00007ffff67e329a in middle::liveness::__extensions__::meth_68831::pat_bindings::_ddf8809261f68fb::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#23 0x00007ffff67df99c in middle::liveness::check_local::_22f6234b85df82a::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#24 0x00007ffff630c226 in visit::default_visitor_27195::anon::expr_fn_27642 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#25 0x00007ffff630ace1 in visit::visit_stmt_27583::_407f2e602dd0ae55::_06 ()
---Type <return> to continue, or q <return> to quit---
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#26 0x00007ffff62fdafa in visit::visit_block_26801::_7fa773563467a885::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#27 0x00007ffff67d0832 in middle::liveness::visit_fn::_24b64fce76d3c6b5::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#28 0x00007ffff6309388 in visit::visit_item_27461::_7993a9ab1149fd3::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#29 0x00007ffff6308c24 in visit::default_visitor_27195::anon::expr_fn_27410 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#30 0x00007ffff63093f9 in visit::visit_item_27461::_7993a9ab1149fd3::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#31 0x00007ffff6308c24 in visit::default_visitor_27195::anon::expr_fn_27410 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#32 0x00007ffff63093f9 in visit::visit_item_27461::_7993a9ab1149fd3::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
---Type <return> to continue, or q <return> to quit---
#33 0x00007ffff6308c24 in visit::default_visitor_27195::anon::expr_fn_27410 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#34 0x00007ffff67c648e in middle::liveness::check_crate::_1628ef44bfc587c9::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#35 0x00007ffff69bc2f9 in driver::driver::compile_rest::anon::expr_fn_86790 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#36 0x00007ffff69baf38 in driver::driver::time_86660::_bd3632ba6dcca720::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#37 0x00007ffff69b82e8 in driver::driver::compile_rest::_ae80a5494db8f5f0::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#38 0x00007ffff69ff184 in __morestack ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#39 0x00007ffff69bc677 in driver::driver::compile_upto::_3511ccef3fbc817::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#40 0x00007ffff69bca79 in driver::driver::compile_input::_da16f137dd9fced5::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
---Type <return> to continue, or q <return> to quit---
No symbol table info available.
#41 0x00007ffff69ead2a in run_compiler::_a31e58461d354de::_06 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#42 0x00007ffff69fc0e1 in monitor::anon::expr_fn_89125 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#43 0x00007ffff69fa74e in task::__extensions__::try_88691::anon::expr_fn_88960 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#44 0x00007ffff69ff184 in __morestack ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.6.so
No symbol table info available.
#45 0x00007ffff79dee5f in task::spawn::spawn_raw::make_child_wrapper::anon::expr_fn_12299 () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
No symbol table info available.
#46 0x00007ffff7a44f60 in __morestack ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
No symbol table info available.
#47 0x00007ffff5ed17e4 in task_start_wrapper (a=<optimised out>)
    at /home/tony/Projects/rust/src/rt/rust_task.cpp:162
        __PRETTY_FUNCTION__ = "void task_start_wrapper(spawn_args*)"
        env = <optimised out>
---Type <return> to continue, or q <return> to quit---
        ca = {spargs = 0x0, threw_exception = false}
        task = 0x7fffec10ba70
        threw_exception = false
#48 0x0000000000000000 in ?? ()
No symbol table info available.
