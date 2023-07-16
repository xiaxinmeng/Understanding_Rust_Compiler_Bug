text
        StorageDead(_2); // _2 is uninit after this.
        _1 = move _2 as std::boxed::Box<dyn std::marker::Send> (Pointer(Unsize));
        drop(_2) -> [return: bb7, unwind: bb6]; // What?
