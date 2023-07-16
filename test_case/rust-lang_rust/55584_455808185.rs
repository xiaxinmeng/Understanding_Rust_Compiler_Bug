
assert_eq!(format!("{: >4?}", [1.0, 2.0]), "[ 1.0,  2.0]");
assert_eq!(format!("{: <4?}", [1.0, 2.0]), "[1.0 , 2.0 ]");
    
// Specific use case
assert_eq!(format!("{:02x?}", [1, 2]), "[01, 02]");
