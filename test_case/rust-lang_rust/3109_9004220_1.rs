

* thread #2: tid = 0x2403, 0x0000000100076525 libcore-c3ca5d77d81b46c1-0.4.dylib`repr::__extensions__::meth_11435::visit_estr::_bb314cd7b2d5487c::_04 + 117, stop reason = EXC_BAD_ACCESS (code=13, address=0x0)
    frame #0: 0x0000000100076525 libcore-c3ca5d77d81b46c1-0.4.dylib`repr::__extensions__::meth_11435::visit_estr::_bb314cd7b2d5487c::_04 + 117
    frame #1: 0x000000010200540f
    frame #2: 0x000000010000ed6d libcore-c3ca5d77d81b46c1-0.4.dylib`repr::__extensions__::meth_4985::visit_estr_uniq::_bb314cd7b2d5487c::_04 + 109
    frame #3: 0x000000010000236b log_bare_str`glue_visit_1814 + 75
    frame #4: 0x000000010000ff48 libcore-c3ca5d77d81b46c1-0.4.dylib`repr::__extensions__::meth_5029::visit_tup_field::_4a8896f1ceaa562::_04 + 184
    frame #5: 0x0000000100002274 log_bare_str`glue_visit_1808 + 260
    frame #6: 0x00000001000038bb log_bare_str`intrinsic::rusti::visit_tydesc_1924::_b9735e93cd22bed::_00 + 75
    frame #7: 0x0000000100001cfe log_bare_str`repr::write_repr_1776::_1f9fb77bcf4d318::_00 + 702
    frame #8: 0x0000000100001a34 log_bare_str`logging::log_type_1773::anon + 68
    frame #9: 0x00000001000041df log_bare_str`__morestack + 224
    frame #10: 0x0000000100001a14 log_bare_str`logging::log_type_1773::anon + 36
    frame #11: 0x0000000100032c1d libcore-c3ca5d77d81b46c1-0.4.dylib`io::with_bytes_writer::_95ef8a6f639649c::_04 + 189
    frame #12: 0x00000001000018c7 log_bare_str`logging::log_type_1773::_46883f83cbac29e::_00 + 103
    frame #13: 0x0000000100001617 log_bare_str`main::_c7f570f6468c2ea::_00 + 215
    frame #14: 0x00000001000016ae log_bare_str`_rust_main + 46
    frame #15: 0x00000001001fb0f5 librustrt.dylib`task_start_wrapper(spawn_args*) + 37 at rust_task.cpp:150
