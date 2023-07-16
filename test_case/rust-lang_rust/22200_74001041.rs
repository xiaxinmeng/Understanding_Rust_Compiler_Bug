 rust
#[bench]                                                            
fn libc1(b: &mut test::Bencher) {                                   
    b.iter(|| unsafe {                                              
        extern { fn memset(p: *mut u8, n: u8, amt: usize); }        
        let ptr = libc::malloc(1000);                               
        assert!(!ptr.is_null());                                    
        memset(ptr as *mut u8, 0, 1000);                            
        libc::free(ptr);                                            
    });                                                             
}                                                                   
#[bench]                                                            
fn heap(b: &mut test::Bencher) {                                    
    b.iter(|| unsafe {                                              
        extern { fn memset(p: *mut u8, n: u8, amt: usize); }        
        let ptr = std::rt::heap::allocate(1000, 8);                 
        assert!(!ptr.is_null());                                    
        memset(ptr as *mut u8, 0, 1000);                            
        std::rt::heap::deallocate(ptr, 1000, 8);                    
    });                                                             
}                                                                   
