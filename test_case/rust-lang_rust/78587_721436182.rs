
struct S1(Box<u8>);
struct S2(Option<Box<u8>>);
enum S3 { None, Some(Box<u8>) }
enum S4 { None, Empty, Some(Box<u8>) }
struct S5(Option<S3>);
struct S6(Option<S4>);
