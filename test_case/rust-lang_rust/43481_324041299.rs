
        _3 = &'7_0rce _4;                // scope 0 at issue-43481.rs:2:21: 2:52
        _2 = &'7_0rce (*_3);             // scope 0 at issue-43481.rs:2:21: 2:52
        _1 = _2 as &'7_0rce [u8] (Unsize); // scope 0 at issue-43481.rs:2:21: 2:52
           ...
        EndRegion('7_0rce);              // scope 0 at issue-43481.rs:1:24: 4:2
        StorageDead(_1);                 // scope 0 at issue-43481.rs:4:2: 4:2
        StorageDead(_4);                 // scope 0 at issue-43481.rs:4:2: 4:2
