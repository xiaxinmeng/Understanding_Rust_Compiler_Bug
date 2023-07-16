
    bb6: {
        StorageLive(_11);                
        _11 = (((_7 as Some).0: (usize, &u32)).0: usize); 
        StorageLive(_12);                
        StorageLive(_13);                
        StorageLive(_14);                
        _14 = _11;                       
        StorageLive(_15);               
        StorageLive(_16);                
        _16 = _1;                        
        _15 = Len((*_16));
        StorageDead(_16);                
        _13 = Lt(move _14, move _15);    // <----
        StorageDead(_15);                
        StorageDead(_14);               
        _12 = Not(move _13);
        StorageDead(_13);                
        switchInt(move _12) -> [false: bb10, otherwise: bb9];
    }
