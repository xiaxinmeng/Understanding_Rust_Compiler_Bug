 Shell
   Compiling jen v0.0.0 (file:///Users/Mitch/Programming/Rust/RustyJen)                                                    
error: internal compiler error: unexpected panic                                                                           
note: the compiler unexpectedly panicked. this is a bug.                                                                   
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html                                 
note: run with `RUST_BACKTRACE=1` for a backtrace                                                                          
task 'rustc' panicked at 'assertion failed: *start <= *end', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src
/libcore/slice.rs:396                                                                                                      

stack backtrace:                                                                                                           
   1:        0x10d976929 - rt::backtrace::imp::write::hfe87245730e40e498nq                                                 
   2:        0x10d979b6c - failure::on_fail::h250240fc1d2cc13cHEq                                                          
   3:        0x10dbdb395 - unwind::begin_unwind_inner::h629bbfd5c5c7ad4bSJd                                                
   4:        0x10dbdb02f - unwind::begin_unwind_fmt::h26eed6b8c9614d01kHd                                                  
   5:        0x10dbdad82 - rust_begin_unwind                                                                               
   6:        0x10dc315dc - panicking::panic_fmt::h9773bd98fc91f575h7j                                                      
   7:        0x10dc2806f - panicking::panic::hf2209879fd0a6385l4j                                                          
   8:        0x10aaee5eb - middle::typeck::check::vtable::check_object_safety::hd2ef41a3d3297642CBN                        
   9:        0x10ab588db - middle::typeck::check::FnCtxt<'a, 'tcx>::register_unsize_obligations::h93da532586d1f760GhX      
  10:        0x10ab58770 - middle::typeck::check::FnCtxt<'a, 'tcx>::register_autoref_obligations::hdcb3d01fc6bc2a0bEgX     
  11:        0x10ab58791 - middle::typeck::check::FnCtxt<'a, 'tcx>::register_autoref_obligations::hdcb3d01fc6bc2a0bEgX     
  12:        0x10ab345b7 - middle::typeck::check::FnCtxt<'a, 'tcx>::write_adjustment::h559300c55596d0cbIdX                 
  13:        0x10ab255fb - middle::typeck::check::FnCtxt<'a, 'tcx>::mk_assignty::h495121b0f62037e0xvX                      
  14:        0x10ab253c5 - middle::typeck::check::demand::coerce::h94b3644e227f71a2RPR                                     
  15:        0x10ab85f09 - middle::typeck::check::check_expr_coercable_to_type::closure.132412                             
  16:        0x10ab7ded6 - middle::typeck::check::check_expr_with_unifier::h69de001b4060f90bdJY                            
  17:        0x10abce4d3 - middle::typeck::check::check_decl_local::h0cae6d84e687454cWJ0                                   
  18:        0x10abce647 - middle::typeck::check::check_stmt::h46c2f90bba03a179WL0                                         
  19:        0x10ab49acf - middle::typeck::check::check_block_with_expected::hde19a8324d3b35305P0                          
  20:        0x10ab45862 - middle::typeck::check::check_fn::h6804afcefd6fc458pvV                                           
  21:        0x10ab44d8f - middle::typeck::check::check_bare_fn::hb7e7210980026133EkV                                      
  22:        0x10ab4c530 - middle::typeck::check::check_method_body::h36c9acd490135be6xPV                                  
  23:        0x10ab41d58 - middle::typeck::check::check_item::h39bec44c2e37ec07NEV                                         
  24:        0x10ab44462 - visit::Visitor::visit_mod::h8897482306723449792                                                 
  25:        0x10ab42fb0 - visit::walk_item::h18177613619592010200                                                         
  26:        0x10ab4446d - visit::Visitor::visit_mod::h8897482306723449792                                                 
  27:        0x10ab42fb0 - visit::walk_item::h18177613619592010200                                                         
  28:        0x10ab44bea - middle::typeck::check::check_item_types::h232f3854bb5459bfOjV                                   
  29:        0x10a6571a6 - util::common::time::h13516290540542203264                                                       
  30:        0x10ae04d7e - middle::typeck::check_crate::h6b2f140055619821A3n                                               
  31:        0x10ae6dadf - driver::driver::phase_3_run_analysis_passes::h9af5661d865aaa0elNA                               
  32:        0x10ae683a8 - driver::driver::compile_input::h64933a02c4abd95e6tA                                             
  33:        0x10aee5b88 - driver::run_compiler::h37c520fe594ec038KgE                                                      
  34:        0x10aee3dae - driver::run::closure.145405                                                                     
  35:        0x10a66f64b - task::TaskBuilder<S>::try_future::closure.103660                                                
  36:        0x10a66f543 - task::TaskBuilder<S>::spawn_internal::closure.103631                                            
  37:        0x10d69ba4d - task::NativeSpawner.Spawner::spawn::closure.8535                                                
  38:        0x10dc4124c - rust_try_inner                                                                                  
  39:        0x10dc41236 - rust_try                                                                                        
  40:        0x10dbd8a17 - unwind::try::h554e9d30ab8d9725cyd                                                               
  41:        0x10dbd88ac - task::Task::run::h39093ecd59554b49ZJc                                                           
  42:        0x10d69b873 - task::NativeSpawner.Spawner::spawn::closure.8472                                                
  43:        0x10dbda1c7 - thread::thread_start::h589be8c0a398d299e5c                                                      
  44:     0x7fff981f52fc - _pthread_body                                                                                   
  45:     0x7fff981f5279 - _pthread_body                                                                                   
