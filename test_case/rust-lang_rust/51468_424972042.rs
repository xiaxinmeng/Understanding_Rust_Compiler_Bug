
% rustup show
Default host: x86_64-apple-darwin

installed toolchains
--------------------

stable-x86_64-apple-darwin
nightly-x86_64-apple-darwin (default)

active toolchain
----------------

nightly-x86_64-apple-darwin (default)
rustc 1.30.0-nightly (ae7fe84e8 2018-09-26)

% cargo doc -p net2
    Checking libc v0.2.43                                                       
    Checking cfg-if v0.1.5                                                      
 Documenting cfg-if v0.1.5                                                      
 Documenting libc v0.2.43                                                       
 Documenting net2 v0.2.33                                                       
    Finished dev [unoptimized + debuginfo] target(s) in 6.60s                   
% cargo doc -p h2
    Checking byteorder v1.2.6                                                   
 Documenting byteorder v1.2.6                                                   
 Documenting fnv v1.0.6                                                         
 Documenting futures v0.1.24                                                    
    Checking fnv v1.0.6                                                         
    Checking futures v0.1.24                                                    
 Documenting itoa v0.4.3                                                        
    Checking itoa v0.4.3                                                        
 Documenting slab v0.4.1                                                        
    Checking indexmap v1.0.1                                                    
 Documenting string v0.1.1                                                      
 Documenting indexmap v1.0.1                                                    
    Checking slab v0.4.1                                                        
    Checking string v0.1.1                                                      
 Documenting iovec v0.1.2                                                       
    Checking iovec v0.1.2                                                       
    Checking log v0.4.5                                                         
 Documenting log v0.4.5                                                         
    Checking bytes v0.4.10                                                      
 Documenting bytes v0.4.10                                                      
    Checking tokio-io v0.1.9                                                    
    Checking http v0.1.13                                                       
 Documenting http v0.1.13                                                       
 Documenting tokio-io v0.1.9                                                    
 Documenting h2 v0.1.12                                                         
    Finished dev [unoptimized + debuginfo] target(s) in 15.75s                  
%
