 rust
extern crate test;                                                

#[bench]                                                          
fn foo(b: &mut test::Bencher) {                                   
    b.iter(|| {                                                   
        let mut v = Vec::from_fn(100 * 1024 * 1024, |_| 0u8);     
        v.insert(0, 1);                                           
        v.insert(0, 1);                                           
        v.insert(0, 1);                                           
        v.insert(0, 1);                                           
        v                                                         
    });                                                           
}                                                                 
