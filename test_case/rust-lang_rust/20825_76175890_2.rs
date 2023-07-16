
foo.rs:5:40: 5:66 error: illegal recursive type; insert an enum or struct in the cycle, if this is desired [E0246]
foo.rs:5 pub trait Processor : Subscriber<Input=<Self as Processor>::Input> {
