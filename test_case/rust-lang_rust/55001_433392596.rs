shell
bash-4.2$ RUST_BACKTRACE=1 cargo doc --open
 Documenting num_cpus v1.8.0                                                                                                                                                             
   Compiling lazy_static v1.1.0                                                                                                                                                          
 Documenting typenum v1.10.0                                                                                                                                                             
    Checking matrixmultiply v0.1.14                                                                                                                                                      
 Documenting matrixmultiply v0.1.14                                                                                                                                                      
 Documenting rand v0.5.5                                                                                                                                                                 
   Compiling rayon v1.0.2                                                                                                                                                                
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached                                                                                         
                                                                                                                                                                                         
thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:587:9                                                                                                                  
    Checking approx v0.3.0                                                                                                                                                               
    Checking num-complex v0.2.1                                                                                                                                                          
 Documenting approx v0.3.0                                                                                                                                                               
stack backtrace:                                                                                                                                                                         
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace                                                                                                                          
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49                                                                                                                            
   1: std::sys_common::backtrace::print                                                                                                                                                  
             at libstd/sys_common/backtrace.rs:71                                                                                                                                        
             at libstd/sys_common/backtrace.rs:59                                                                                                                                        
   2: std::panicking::default_hook::{{closure}}                                                                                                                                          
             at libstd/panicking.rs:211                                                                                                                                                  
   3: std::panicking::default_hook                                                                                                                                                       
             at libstd/panicking.rs:227                                                                                                                                                  
   4: std::panicking::rust_panic_with_hook                                                                                                                                               
             at libstd/panicking.rs:477                                                                                                                                                  
   5: std::panicking::begin_panic                                                                                                                                                        
   6: rustc_errors::Handler::bug                                                                                                                                                         
   7: rustc::util::bug::opt_span_bug_fmt::{{closure}}                                                                                                                                    
   8: rustc::ty::context::tls::with_opt::{{closure}}                                                                                                                                     
   9: rustc::ty::context::tls::with_context_opt                                                                                                                                          
  10: rustc::ty::context::tls::with_opt                                                                                                                                                  
  11: rustc::util::bug::opt_span_bug_fmt                                                                                                                                                 
  12: rustc::util::bug::bug_fmt                                                                                                                                                          
  13: rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx                                                          
  14: rustc::ty::context::TyCtxt::lift_to_global                                                                                                                                         
  15: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  16: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  17: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  18: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  19: rustc::infer::InferCtxt::probe                                                                                                                                                     
  20: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  21: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  22: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  23: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  24: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  25: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  26: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  27: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  28: rustc::infer::InferCtxt::probe                                                                                                                                                     
  29: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  30: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  31: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  32: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  33: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  34: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  35: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  36: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  37: rustc::infer::InferCtxt::probe                                                                                                                                                     
  38: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  39: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  40: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  41: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  42: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  43: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  44: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  45: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  46: rustc::infer::InferCtxt::probe                                                                                                                                                     
  47: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  48: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  49: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  50: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  51: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  52: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  53: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  54: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  55: rustc::infer::InferCtxt::probe                                                                                                                                                     
  56: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  57: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  58: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  59: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  60: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  61: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  62: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  63: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  64: rustc::infer::InferCtxt::probe                                                                                                                                                     
  65: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  66: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  67: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  68: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  69: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  70: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  71: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  72: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  73: rustc::infer::InferCtxt::probe                                                                                                                                                     
  74: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  75: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  76: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  77: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  78: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  79: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  80: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  81: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  82: rustc::infer::InferCtxt::probe                                                                                                                                                     
  83: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  84: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  85: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  86: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  87: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  88: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  89: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  90: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
  91: rustc::infer::InferCtxt::probe                                                                                                                                                     
  92: <&'a mut I as core::iter::iterator::Iterator>::next                                                                                                                                
  93: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                                    
  94: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache                                                                                                        
  95: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  96: rustc::traits::select::SelectionContext::candidate_from_obligation                                                                                                                 
  97: rustc::traits::select::SelectionContext::evaluate_stack                                                                                                                            
  98: rustc::dep_graph::graph::DepGraph::with_anon_task                                                                                                                                  
  99: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                                                                                                            
                                                                                                                                                                                         
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                                 
                                                                                                                                                                                         
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                                                                        
                                                                                                                                                                                         
note: rustc 1.30.0 (da5f414c2 2018-10-24) running on x86_64-unknown-linux-gnu                                                                                                            
                                                                                                                                                                                         
error: Could not document `typenum`.                                                                                                                                                     

Caused by:
  process didn't exit successfully: `rustdoc --crate-name typenum /ccc/home/cont001/ocre/toumineta/.cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.10.0/src/lib.rs --cap-lints allow --color always -o /ccc/dsku/nfs-server/user/cont001/ocre/toumineta/Github/coupe/target/doc -L dependency=/ccc/dsku/nfs-server/user/cont001/ocre/toumineta/Github/coupe/target/debug/deps` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
error: build failed  
