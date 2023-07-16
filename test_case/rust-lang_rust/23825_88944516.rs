
error: internal compiler error: Impl DefId { krate: 2, node: 34078 } was matchable against Obligation(predicate=Binder(TraitPredicate(core::ops::Add<_>)),depth=1) but now is not
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                         
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                                                                
note: run with `RUST_BACKTRACE=1` for a backtrace                                                                                                                                
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:190                                     

stack backtrace:                                                                                                                                                                 
   1:        0x1121339f4 - sys::backtrace::write::h79b5fc37599ff6dfazD                                                                                                           
   2:        0x1121614b0 - panicking::on_panic::h56cd7321a5392f568oJ                                                                                                             
   3:        0x112071c8e - rt::unwind::begin_unwind_inner::h9b1dd02e7e9b2523i7I                                                                                                  
   4:        0x1118565ce - rt::unwind::begin_unwind::h5359193300489870763                                                                                                        
   5:        0x111856e19 - diagnostic::Handler::bug::h0696301796c915b4ShB                                                                                                        
   6:        0x10f3ae6be - middle::traits::select::SelectionContext<'cx, 'tcx>::rematch_impl::h00877b9f7d68b755V3R                                                               
   7:        0x10f3ae008 - middle::infer::InferCtxt<'a, 'tcx>::try::h1635483048343115861                                                                                         
   8:        0x10f38f454 - middle::traits::select::SelectionContext<'cx, 'tcx>::confirm_candidate::h2b843f592bb3d1ebJlR                                                          
   9:        0x10f3638e3 - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h48941a5b9b6aeafbApP                                                                     
  10:        0x10f37363a - middle::traits::project::project_type::h27f6c84f9a43d3fbK5N                                                                                           
  11:        0x10f371017 - middle::traits::project::opt_normalize_projection_type::h7647720dfb8d8959mYN                                                                          
  12:        0x10f36aa36 - middle::traits::project::poly_project_and_unify_type::closure.81002                                                                                   
  13:        0x10f368a40 - middle::traits::project::poly_project_and_unify_type::h14c21863493f04758EN                                                                            
  14:        0x10f3602e6 - middle::traits::fulfill::FulfillmentContext<'tcx>::select::hd40ad264985439f7hhN                                                                       
  15:        0x10f35f928 - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h0ebb717f715533c9vgN                                                        
  16:        0x10e724d63 - check::vtable::select_fcx_obligations_where_possible::hbca02d457f2550e2r4b                                                                            
  17:        0x10e75d48d - check::FnCtxt<'a, 'tcx>::resolve_type_vars_if_possible::h4ae213f9cb49bdadLYo                                                                          
  18:        0x10e7d91ae - check::op::check_binop::h03b611fd681fb03fM6m                                                                                                          
  19:        0x10e81e5cc - check::check_expr_with_unifier::h17623262095715920792                                                                                                 
  20:        0x10e8484f2 - check::check_decl_local::hceee65fb94b47336eis                                                                                                         
  21:        0x10e7e8ad1 - check::check_block_with_expected::ha2448513d5fb9fb9kos                                                                                                
  22:        0x10e82bf51 - check::check_expr_with_unifier::h15858122413925670105                                                                                                 
  23:        0x10e713b05 - check::_match::check_match::closure.28752                                                                                                             
  24:        0x10e713811 - check::_match::check_match::h6e204581d7b2887c9mb                                                                                                      
  25:        0x10e81eba4 - check::check_expr_with_unifier::h17623262095715920792                                                                                                 
  26:        0x10e7e8e0b - check::check_block_with_expected::ha2448513d5fb9fb9kos                                                                                                
  27:        0x10e803584 - check::check_block_no_value::he84d13599fd37c5alns                                                                                                     
  28:        0x10e82be51 - check::check_expr_with_unifier::h15858122413925670105                                                                                                 
  29:        0x10e713b05 - check::_match::check_match::closure.28752                                                                                                             
  30:        0x10e713811 - check::_match::check_match::h6e204581d7b2887c9mb                                                                                                      
  31:        0x10e81eba4 - check::check_expr_with_unifier::h17623262095715920792                                                                                                 
  32:        0x10e8484f2 - check::check_decl_local::hceee65fb94b47336eis                                                                                                         
  33:        0x10e7e8ad1 - check::check_block_with_expected::ha2448513d5fb9fb9kos                                                                                                
  34:        0x10e81ec21 - check::check_expr_with_unifier::h17623262095715920792                                                                                                 
  35:        0x10e7e8e0b - check::check_block_with_expected::ha2448513d5fb9fb9kos                                                                                                
   36:        0x10e82bf51 - check::check_expr_with_unifier::h15858122413925670105    
  37:        0x10e713b05 - check::_match::check_match::closure.28752                
  38:        0x10e713811 - check::_match::check_match::h6e204581d7b2887c9mb         
  39:        0x10e81eba4 - check::check_expr_with_unifier::h17623262095715920792    
  40:        0x10e7e8e0b - check::check_block_with_expected::ha2448513d5fb9fb9kos   
  41:        0x10e82bf51 - check::check_expr_with_unifier::h15858122413925670105    
  42:        0x10e713b05 - check::_match::check_match::closure.28752                
  43:        0x10e713811 - check::_match::check_match::h6e204581d7b2887c9mb         
  44:        0x10e81eba4 - check::check_expr_with_unifier::h17623262095715920792    
  45:        0x10e7e8e0b - check::check_block_with_expected::ha2448513d5fb9fb9kos   
  46:        0x10e7c33e5 - check::check_fn::h7a50c890b5419d55G1n                    
  47:        0x10e7e395e - check::check_bare_fn::hb7403022fad4c11dfRn               
  48:        0x10e7db85f - check::check_item::hede794dfa708b89909n                  
  49:        0x10e7e1ec2 - visit::walk_item::h9731833736362995753                   
  50:        0x10e8b8ff1 - check_crate::closure.36204                               
  51:        0x10e8b3437 - check_crate::h2a80ddc71481fbe04wC                        
  52:        0x10e5d7287 - driver::phase_3_run_analysis_passes::h6b1a4d735c5454a3oGa
  53:        0x10e5bb9c5 - driver::compile_input::hc433cbaf5268b434Qba              
  54:        0x10e679b75 - run_compiler::hfdf6389c24d7bf2fp2b                       
  55:        0x10e67748f - thunk::F.Invoke<A, R>::invoke::h17962875359394492347     
  56:        0x10e6765b7 - rt::unwind::try::try_fn::h2161268800680238185            
  57:        0x1121e8028 - rust_try_inner                                           
  58:        0x1121e8015 - rust_try                                                 
  59:        0x10e676988 - thunk::F.Invoke<A, R>::invoke::h12486231582466062018     
  60:        0x11214a91d - sys::thread::create::thread_start::h5cc5a46a7850c86a84H  
  61:     0x7fff82c3e267 - _pthread_body                                            
  62:     0x7fff82c3e1e4 - _pthread_start                                           

Could not compile `elmesque`.                                                       
