

mod b {
struct Foo;
struct Bar;
}

mod c {
 use b::Foo; // no error

 fn x(y: b::Bar) { // error (correct usage: `::b::Bar`)

 }
}
