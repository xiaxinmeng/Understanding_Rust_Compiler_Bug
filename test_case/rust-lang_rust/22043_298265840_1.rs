rust
#![feature(box_syntax, box_heap)]
#![feature(placement_in_syntax)]

type BoxedFunc = Box<Fn(&()) -> &()>;

// works
fn a() -> BoxedFunc {
    box |a:&()| { a }
}

// breaks
fn b() -> BoxedFunc {
    in (::std::boxed::HEAP) { |b:&()| { b } }
}

fn main() {}
