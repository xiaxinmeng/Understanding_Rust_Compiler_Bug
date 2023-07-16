rust
let ts = TupleStruct(640, 480); // Function-like constructor of a tuple struct
let us = UnitStruct; // Constant-like constructor of a unit struct

// Plus same for patterns
match ... {
    TupleStruct(..) => {}
}
match ... {
    UnitStruct => {}
}
