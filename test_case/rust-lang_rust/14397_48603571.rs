 rust
#![allow(dead_code)]

extern crate debug;

fn main() {
    struct Foo<type T> { x: int, y: T }
    trait W for type { fn weight(&self) -> int; }
    #[cfg(buggy)]
    impl W for W {
        fn weight(&self) -> int { (*self).weight() }
    }
    impl W for (int, int) {
        fn weight(&self) -> int { self.val0() + self.val1() }
    }
    impl<type X:W> W for Foo<X> {
        fn weight(&self) -> int { self.x + self.y.weight() }
    }

    let f : Foo<(int,int)> = Foo { x: 1i, y: (2i, 3i) };
    let g : &Foo<(int, int)> = &f;
    let h : &Foo<W> = g;
    println!("h.y.weight(): {:?}", h.y.weight());
    println!("h.weight(): {:?}", h.weight());
}
