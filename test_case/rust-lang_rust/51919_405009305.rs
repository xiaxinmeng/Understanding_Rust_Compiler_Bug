
to_be().to_bytes()              → to_be_bytes()            [reduction by  5 chars]
to_le().to_bytes()              → to_le_bytes()            [reduction by  5 chars]
to_bytes()                      → to_native_bytes()        [increase  by  7 chars]
i32::from_be(i32::from_bytes()) → i32::from_be_bytes()     [reduction by 10 chars]
i32::from_le(i32::from_bytes()) → i32::from_le_bytes()     [reduction by 10 chars]
i32::from_bytes()               → i32::from_native_bytes() [increase  by  7 chars]
