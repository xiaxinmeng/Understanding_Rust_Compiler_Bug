
#[deriving(CLike)]
enum MyFlag { Flag1, Flag2, Flag3 }

type MyFlags = EnumSet<MyFlag>;    
