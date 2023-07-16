rust
trait MyStuff {
    fn a(self: Option<Self>);
    fn b(self: Result<Self, Self>);
    fn c(self: (Self, Self, Self));
    fn d(self: Box<Box<Self>>);
}

impl MyStuff for i32 {
   ...
}

Some(1).a();  // ok?
Ok(2).b();  // ok?
(3, 4, 5).c(); // ok?
(box box 6).d(); // ok?
