
error[E0425]: cannot find value `bogus` in this scope                                                                                                                                                                                  
 --> src/lib.rs:3:28                                                                                                                                                                                                                   
  |                                                                                                                                                                                                                                    
3 |         const CONST: u32 = bogus.field;                                                                                                                                                                                            
  |                            ^^^^^ not found in this scope                                                                                                                                                                           
                                                                                                                                                                                                                                       
error: internal compiler error: src/librustc/ty/context.rs:262: node expr bogus.field (hir_id=HirId { owner: DefIndex(0:5), local_id: 6 }) with HirId::owner DefId(0/0:5 ~ b[6c41]::func[0]::Trait[0]::CONST[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:3 ~ b[6c41]::func[0]) 
...
  13: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  14: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
             at src/librustc/ty/context.rs:262
  15: rustc::ty::context::tls::with::{{closure}}
             at src/librustc/ty/context.rs:2101
  16: rustc::ty::context::tls::with_context::{{closure}}
             at src/librustc/ty/context.rs:2055
  17: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:2045
  18: rustc::ty::context::tls::with_context
             at src/librustc/ty/context.rs:2055
  19: rustc::ty::context::tls::with
             at src/librustc/ty/context.rs:2101
  20: rustc::ty::context::TypeckTables::expr_adjustments
             at src/librustc/ty/context.rs:259
             at src/librustc/ty/context.rs:637
  21: rustc::ty::context::TypeckTables::expr_ty_adjusted_opt
             at src/librustc/ty/context.rs:650
  22: rustc_save_analysis::SaveContext::get_expr_data
             at src/librustc_save_analysis/lib.rs:518
  23: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_expr                                                                                                                                        
             at src/librustc_save_analysis/dump_visitor.rs:1548                                                                                                                                                                        
  24: rustc_save_analysis::dump_visitor::DumpVisitor<O>::process_assoc_const                                                                                                                                                           
             at src/librustc_save_analysis/dump_visitor.rs:468                                                                                                                                                                         
  25: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item                                                                                                                                        
             at src/librustc_save_analysis/dump_visitor.rs:1085                                                                                                                                                                        
             at src/librustc_save_analysis/dump_visitor.rs:765                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:1401                                                                                                                                                                        
  26: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item                                                                                                                                        
             at /home/xanewok/repos/rust/src/libsyntax/visit.rs:649                                                                                                                                                                    
             at src/librustc_save_analysis/dump_visitor.rs:1630                                                                                                                                                                        
             at /home/xanewok/repos/rust/src/libsyntax/visit.rs:171                                                                                                                                                                    
             at /home/xanewok/repos/rust/src/libsyntax/visit.rs:67                                                                                                                                                                     
             at src/librustc_save_analysis/dump_visitor.rs:408                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:118                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:408                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:131                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:408                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:1389                                                                                                                                                                        
  27: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_mod                                                                                                                                         
             at /home/xanewok/repos/rust/src/libsyntax/visit.rs:171                                                                                                                                                                    
             at src/librustc_save_analysis/dump_visitor.rs:1350                                                                                                                                                                        
             at src/librustc_save_analysis/dump_visitor.rs:118                                                                                                                                                                         
             at src/librustc_save_analysis/dump_visitor.rs:1350  
