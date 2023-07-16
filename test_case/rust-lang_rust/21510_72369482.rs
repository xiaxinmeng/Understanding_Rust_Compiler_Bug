
error: internal compiler error: Where clause `Binder(TraitRef(<Self as iter::prev_iter::ZipPrev>::I, core::marker::Sized))` was applicable to `Obligation(predicate=Binder(TraitPredicate(TraitRef(Self, core::marker::Sized))),depth=62)
` but now is not                                                                                                                                                                                                                         
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                                                                                 
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html                                                                                                                                               
note: run with `RUST_BACKTRACE=1` for a backtrace                                                                                                                                                                                        
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:189                                                                                             

stack backtrace:                                                                                                                                                                                                                         
   1:        0x10aca7987 - sys::backtrace::write::h073b317a579f9618vbu                                                                                                                                                                   
   2:        0x10accde3c - failure::on_fail::h54ee7c9c2612226cqbB                                                                                                                                                                        
   3:        0x10ac2f248 - rt::unwind::begin_unwind_inner::h7973274734543ef5mTA                                                                                                                                                          
   4:        0x10a3bae77 - rt::unwind::begin_unwind::h18302672585329669608                                                                                                                                                               
   5:        0x10a3bb8b2 - diagnostic::Handler::bug::h79ffff14cb2a7a2eXKE                                                                                                                                                                
   6:        0x10802ae12 - middle::traits::select::SelectionContext<'cx, 'tcx>::confirm_candidate::h903a1e7437697906V0R                                                                                                                  
   7:        0x1080394e1 - middle::traits::select::SelectionContext<'cx, 'tcx>::winnow_candidate::closure.77926                                                                                                                          
   8:        0x10802dd9f - middle::traits::select::SelectionContext<'cx, 'tcx>::winnow_candidate::h24ebb50a8d4483fegzR                                                                                                                   
   9:        0x10802d71b - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_stack::he91af678666f3fa6WmQ                                                                                                                     
  10:        0x10802ca64 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_predicate_recursively::h569213b2fa1cd253NfQ                                                                                                     
  11:        0x10802d1b9 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_predicates_recursively::h4203966891937153467                                                                                                    
  12:        0x108031fd4 - middle::traits::select::SelectionContext<'cx, 'tcx>::winnow_selection::hd5514dc7d6417a2eWCR                                                                                                                   
  13:        0x1080395cf - middle::traits::select::SelectionContext<'cx, 'tcx>::winnow_candidate::closure.77926                                                                                                                          
  14:        0x10802dd9f - middle::traits::select::SelectionContext<'cx, 'tcx>::winnow_candidate::h24ebb50a8d4483fegzR                                                                                                                   
  15:        0x108034ea7 - vec::Vec<T>::retain::h9904974951109916767                                                                                                                                                                     
  16:        0x108028127 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h778c5b721cdbb3b27tQ                                                                                                          
  17:        0x10800c82a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h8aac486086ff212598P                                                                                                                             
  18:        0x108016582 - middle::traits::project::project_type::hc7103abd0f4e4f32kYO                                                                                                                                                   
  19:        0x108014751 - middle::traits::project::opt_normalize_projection_type::h50cbe2ffd5deb327zQO                                                                                                                                  
  20:        0x108004a13 - middle::traits::project::normalize_projection_type::h9c5b439beb8ad877hPO                                                                                                                                      
  21:        0x107ff6042 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h6f05bcc00ca8ce21lNO                                                                                               
  22:        0x107ff5c4c - vec::Vec<T>.FromIterator<T>::from_iter::h1979070220669330341                                                                                                                                                  
  23:        0x107ff5b6f - middle::subst::VecPerParamSpace<T>::map_enumerated::h3161233761383125913                                                                                                                                      
  24:        0x107ff5987 - middle::ty_fold::super_fold_substs::h8698275836782648687                                                                                                                                                      
  25:        0x107ff5620 - middle::ty_fold::super_fold_trait_ref::h3810601818069357661                                                                                                                                                   
  26:        0x1080154ef - middle::ty_fold::super_fold_binder::h690943729941480250                                                                                                                                                       
  27:        0x10803a509 - middle::traits::project::normalize_with_depth::h2151439079998429777                                                                                                                                           
  28:        0x108038c65 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_where_clause_trait_ref::h6b7e69588df5f68fLOS                                                                                                       
  29:        0x108038b15 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::closure.77901                                                                                                                     
  30:        0x1080386b2 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::ha4e67af63e6755ec6eR                                                                                                              
  31:        0x1080389ad - iter::Filter<A, I, P>.Iterator::next::h6054995908299673259                                                                                                                                                    
  32:        0x10803884c - vec::Vec<T>.Extend<T>::extend::h3142011113858850698                                                                                                                                                           
  33:        0x108034401 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h8fb2d9d893bfd3d2WPQ                                                                                                                
  34:        0x108025b10 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h778c5b721cdbb3b27tQ                                                                                                          
  35:        0x10800c82a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h8aac486086ff212598P                                                                                                                             
 36:        0x108016582 - middle::traits::project::project_type::hc7103abd0f4e4f32kYO                                                    
 37:        0x108014751 - middle::traits::project::opt_normalize_projection_type::h50cbe2ffd5deb327zQO                                   
 38:        0x108004a13 - middle::traits::project::normalize_projection_type::h9c5b439beb8ad877hPO                                       
 39:        0x107ff6042 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h6f05bcc00ca8ce21lNO
 40:        0x107ff5c4c - vec::Vec<T>.FromIterator<T>::from_iter::h1979070220669330341                                                   
 41:        0x107ff5b6f - middle::subst::VecPerParamSpace<T>::map_enumerated::h3161233761383125913                                       
 42:        0x107ff5987 - middle::ty_fold::super_fold_substs::h8698275836782648687                                                       
 43:        0x107ff5620 - middle::ty_fold::super_fold_trait_ref::h3810601818069357661                                                    
 44:        0x1080154ef - middle::ty_fold::super_fold_binder::h690943729941480250                                                        
 45:        0x10803a509 - middle::traits::project::normalize_with_depth::h2151439079998429777                                            
 46:        0x108038c65 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_where_clause_trait_ref::h6b7e69588df5f68fLOS        
 47:        0x108038b15 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::closure.77901                      
 48:        0x1080386b2 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::ha4e67af63e6755ec6eR               
 49:        0x1080389ad - iter::Filter<A, I, P>.Iterator::next::h6054995908299673259                                                     
 50:        0x10803884c - vec::Vec<T>.Extend<T>::extend::h3142011113858850698                                                            
 51:        0x108034401 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h8fb2d9d893bfd3d2WPQ                 
 52:        0x108025b10 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h778c5b721cdbb3b27tQ           
 53:        0x10800c82a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h8aac486086ff212598P                              
 54:        0x108016582 - middle::traits::project::project_type::hc7103abd0f4e4f32kYO                                                    
 55:        0x108014751 - middle::traits::project::opt_normalize_projection_type::h50cbe2ffd5deb327zQO                                   
 56:        0x108004a13 - middle::traits::project::normalize_projection_type::h9c5b439beb8ad877hPO                                       
 57:        0x107ff6042 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h6f05bcc00ca8ce21lNO
 58:        0x107ff5c4c - vec::Vec<T>.FromIterator<T>::from_iter::h1979070220669330341                                                   
 59:        0x107ff5b6f - middle::subst::VecPerParamSpace<T>::map_enumerated::h3161233761383125913                                       
 60:        0x107ff5987 - middle::ty_fold::super_fold_substs::h8698275836782648687                                                       
 61:        0x107ff5620 - middle::ty_fold::super_fold_trait_ref::h3810601818069357661                                                    
 62:        0x1080154ef - middle::ty_fold::super_fold_binder::h690943729941480250                                                        
 63:        0x10803a509 - middle::traits::project::normalize_with_depth::h2151439079998429777                                            
 64:        0x108038c65 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_where_clause_trait_ref::h6b7e69588df5f68fLOS        
 65:        0x108038b15 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::closure.77901                      
 66:        0x1080386b2 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::ha4e67af63e6755ec6eR               
 67:        0x1080389ad - iter::Filter<A, I, P>.Iterator::next::h6054995908299673259                                                     
 68:        0x10803884c - vec::Vec<T>.Extend<T>::extend::h3142011113858850698                                                            
 69:        0x108034401 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h8fb2d9d893bfd3d2WPQ                 
 70:        0x108025b10 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h778c5b721cdbb3b27tQ           
 71:        0x10800c82a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h8aac486086ff212598P                              
 72:        0x108016582 - middle::traits::project::project_type::hc7103abd0f4e4f32kYO                                                    
 73:        0x108014751 - middle::traits::project::opt_normalize_projection_type::h50cbe2ffd5deb327zQO                                   
 74:        0x108004a13 - middle::traits::project::normalize_projection_type::h9c5b439beb8ad877hPO                                       
 75:        0x107ff6042 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h6f05bcc00ca8ce21lNO
 76:        0x107ff5c4c - vec::Vec<T>.FromIterator<T>::from_iter::h1979070220669330341                                                   
 77:        0x107ff5b6f - middle::subst::VecPerParamSpace<T>::map_enumerated::h3161233761383125913                                       
 78:        0x107ff5987 - middle::ty_fold::super_fold_substs::h8698275836782648687                                                       
 79:        0x107ff5620 - middle::ty_fold::super_fold_trait_ref::h3810601818069357661                                                    
 80:        0x1080154ef - middle::ty_fold::super_fold_binder::h690943729941480250                                                        
 81:        0x10803a509 - middle::traits::project::normalize_with_depth::h2151439079998429777                                            
 82:        0x108038c65 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_where_clause_trait_ref::h6b7e69588df5f68fLOS        
 83:        0x108038b15 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::closure.77901                      
 84:        0x1080386b2 - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_where_clause::ha4e67af63e6755ec6eR               
 85:        0x1080389ad - iter::Filter<A, I, P>.Iterator::next::h6054995908299673259                                                     
 86:        0x10803884c - vec::Vec<T>.Extend<T>::extend::h3142011113858850698                                                            
 87:        0x108034401 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h8fb2d9d893bfd3d2WPQ                 
 88:        0x108025b10 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h778c5b721cdbb3b27tQ           
 89:        0x10800c82a - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h8aac486086ff212598P                              
 90:        0x108016582 - middle::traits::project::project_type::hc7103abd0f4e4f32kYO                                                    
 91:        0x108014751 - middle::traits::project::opt_normalize_projection_type::h50cbe2ffd5deb327zQO                                   
 92:        0x108004a13 - middle::traits::project::normalize_projection_type::h9c5b439beb8ad877hPO                                       
 93:        0x107ff6042 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h6f05bcc00ca8ce21lNO
 94:        0x107ff5c4c - vec::Vec<T>.FromIterator<T>::from_iter::h1979070220669330341                                                   
 95:        0x107ff5b6f - middle::subst::VecPerParamSpace<T>::map_enumerated::h3161233761383125913                                       
 96:        0x107ff5987 - middle::ty_fold::super_fold_substs::h8698275836782648687                                                       
 97:        0x107ff5620 - middle::ty_fold::super_fold_trait_ref::h3810601818069357661                                                    
 98:        0x1080154ef - middle::ty_fold::super_fold_binder::h690943729941480250                                                        
 99:        0x10803a509 - middle::traits::project::normalize_with_depth::h2151439079998429777                                            
 100:        0x108038c65 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_where_clause_trait_ref::h6b7e69588df5f68fLOS       
... <frames omitted>                                                                                                                     
