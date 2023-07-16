rust
    assert!(region.len > 128 );
    
    for (a, &b) in my_array.iter_mut().zip(&region[..]) {
       *a = b;
    } 
