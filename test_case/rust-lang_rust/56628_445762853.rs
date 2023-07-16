
$ ./x.py --stage 1 test src/libstd --test-args park
    Finished dev [unoptimized] target(s) in 0.27s                                                                                                                                                                  
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)                                                                                                                               
    Finished release [optimized] target(s) in 0.24s                                                                                                                                                                
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.26s                                                                                                                                                                
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_driver v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_driver)                                                                                                                                    
   Compiling rustc-main v0.0.0 (/home/r/src/rust/rustc.3/src/rustc)                                                                                                                                                
    Finished release [optimized] target(s) in 33.22s                                                                                                                                                               
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling cc v1.0.25                                                                                                                                                                                            
   Compiling build_helper v0.1.0 (/home/r/src/rust/rustc.3/src/build_helper)                                                                                                                                       
   Compiling libc v0.2.43                                                                                                                                                                                          
   Compiling rustc_codegen_llvm v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_codegen_llvm)                                                                                                                        
   Compiling rustc-demangle v0.1.9                                                                                                                                                                                 
   Compiling memmap v0.6.2                                                                                                                                                                                         
   Compiling num_cpus v1.8.0                                                                                                                                                                                       
   Compiling rustc_llvm v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_llvm)                                                                                                                                        
    Finished release [optimized] target(s) in 1m 21s                                                                                                                                                               
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/home/r/src/rust/rustc.3/src/libcore)                                                                                                                                                    
   Compiling unwind v0.0.0 (/home/r/src/rust/rustc.3/src/libunwind)                                                                                                                                                
   Compiling compiler_builtins v0.0.0 (/home/r/src/rust/rustc.3/src/rustc/compiler_builtins_shim)                                                                                                                  
   Compiling std v0.0.0 (/home/r/src/rust/rustc.3/src/libstd)                                                                                                                                                      
   Compiling rustc_msan v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_msan)                                                                                                                                        
   Compiling rustc_tsan v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_tsan)                                                                                                                                        
   Compiling rustc_lsan v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_lsan)                                                                                                                                        
   Compiling rustc_asan v0.0.0 (/home/r/src/rust/rustc.3/src/librustc_asan)                                                                                                                                        
   Compiling libc v0.0.0 (/home/r/src/rust/rustc.3/src/rustc/libc_shim)                                                                                                                                            
   Compiling alloc v0.0.0 (/home/r/src/rust/rustc.3/src/liballoc)                                                                                                                                                  
   Compiling panic_abort v0.0.0 (/home/r/src/rust/rustc.3/src/libpanic_abort)                                                                                                                                      
   Compiling panic_unwind v0.0.0 (/home/r/src/rust/rustc.3/src/libpanic_unwind)                                                                                                                                    
    Finished release [optimized] target(s) in 2m 07s                                                                                                                                                               
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getopts v0.2.17                                                                                                                                                                                       
   Compiling proc_macro v0.0.0 (/home/r/src/rust/rustc.3/src/libproc_macro)                                                                                                                                        
   Compiling term v0.0.0 (/home/r/src/rust/rustc.3/src/libterm)                                                                                                                                                    
   Compiling test v0.0.0 (/home/r/src/rust/rustc.3/src/libtest)                                                                                                                                                    
    Finished release [optimized] target(s) in 25.46s                                                                                                                                                               
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building rustdoc for stage1 (x86_64-unknown-linux-gnu)
   Compiling void v1.0.2                                                                                                                                                                                           
   Compiling stable_deref_trait v1.1.0                                                                                                                                                                             
   Compiling rand_core v0.2.1                                                                                                                                                                                      
   Compiling libc v0.2.43                                                                                                                                                                                          
   Compiling scopeguard v0.3.3                                                                                                                                                                                     
   Compiling pulldown-cmark v0.1.2                                                                                                                                                                                 
   Compiling bitflags v0.9.1                                                                                                                                                                                       
   Compiling remove_dir_all v0.5.1                                                                                                                                                                                 
   Compiling macro-utils v0.1.2                                                                                                                                                                                    
   Compiling owning_ref v0.3.3                                                                                                                                                                                     
   Compiling unreachable v1.0.0                                                                                                                                                                                    
   Compiling minifier v0.0.20                                                                                                                                                                                      
   Compiling smallvec v0.6.5                                                                                                                                                                                       
   Compiling lock_api v0.1.3                                                                                                                                                                                       
   Compiling rand v0.5.5                                                                                                                                                                                           
   Compiling parking_lot_core v0.3.0                                                                                                                                                                               
   Compiling tempfile v3.0.3                                                                                                                                                                                       
   Compiling parking_lot v0.6.4                                                                                                                                                                                    
   Compiling rustdoc v0.0.0 (/home/r/src/rust/rustc.3/src/librustdoc)                                                                                                                                              
   Compiling rustdoc-tool v0.0.0 (/home/r/src/rust/rustc.3/src/tools/rustdoc)                                                                                                                                      
    Finished release [optimized] target(s) in 2m 33s                                                                                                                                                               
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling libc v0.2.43                                                                                                                                                                                          
   Compiling rand_core v0.2.1                                                                                                                                                                                      
   Compiling rand v0.5.5                                                                                                                                                                                           
   Compiling std v0.0.0 (/home/r/src/rust/rustc.3/src/libstd)                                                                                                                                                      
    Finished release [optimized] target(s) in 1m 13s                                                                                                                                                               
     Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-b349dadea92738a2

running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 765 filtered out

     Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/env-8e97948f5b491967

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out

   Doc-tests std
error: couldn't load codegen backend "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so": "librustc_codegen_ssa-3de1bbf2fa0b86ab.so: cannot open shared object file: No such file or directory"

error: test failed, to rerun pass '--doc'
