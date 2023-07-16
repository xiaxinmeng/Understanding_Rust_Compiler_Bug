
src/lib.rs:3:33: 3:44 error: allocations are not allowed in statics [E0010]
src/lib.rs:3 static STATIC11: Box<MyOwned> = box MyOwned;
                                             ^~~~~~~~~~~

