
10: core::slice::<impl [T]>::copy_from_slice         
             at ./src/libcore/macros.rs:11                                          
             at ./src/libcore/slice/mod.rs:1971                     
  11: <alloc::vec::Vec<T> as core::clone::Clone>::clone                          
             at ./src/liballoc/vec.rs:1915                                                                                                        
             at ./src/liballoc/vec.rs:1351                        
             at ./src/liballoc/slice.rs:160               
             at ./src/liballoc/slice.rs:380        
             at ./src/liballoc/vec.rs:1642                                                                                                           
 12: <rustc::traits::Vtable<'tcx, N> as core::clone::Clone>::clone             
             at src/librustc/traits/mod.rs:569                                  
             at src/librustc/traits/mod.rs:520                                  
  13: rustc::ty::erase_regions::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::erase_regions
             at src/librustc/ty/erase_regions.rs:26                                                 
  14: rustc::traits::codegen::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::drain_fulfillment_cx_or_panic
             at src/librustc/traits/codegen/mod.rs:161 
  15: rustc::ty::context::GlobalCtxt::enter_local                                                                                                               at src/librustc/traits/codegen/mod.rs:72                                                                                                           at src/librustc/infer/mod.rs:522                                                                  
             at src/librustc/ty/context.rs:1727                 
             at src/librustc/ty/context.rs:1966                                                                                                   
             at src/librustc/ty/context.rs:1899          
             at src/librustc/ty/context.rs:1965        
             at src/librustc/ty/context.rs:1726                                                                                                    
             at src/librustc/ty/context.rs:2072
