rust
#![allow(dead_code)]

mod a { fn f() { println!("Hello from a::f"); } }

trait Trait {
    fn m1(&self);
    fn m2(&self) { }
}

struct S1;
struct S2;

mod b {
    fn f() { println!("Hello from b::f"); }
    struct S3;
    impl ::Trait for S3 {
        fn m1(&self) { println!("Hello from S3::m1"); }
        fn m2(&self) { println!("Hello from S3::m2"); }
    }
}

struct S4;

struct S6;

fn main () {
    mod c { fn f() { println!("Hello from c::f"); } }

    const C: [i32; 2] = [1, 2];

    { mod c2 { const C: [i32; 2] = [1, 2]; } };

    { mod c2 { const C: [i32; 2] = [1, 2]; } }

    { mod c2 { const C: [i32; { mod uh_oh {  } 2 }] = [1, 2]; } }

    { mod c2 { const C: [i32; { mod uh_oh { impl ::Trait for ::S6 { fn m1(&self) { } } } 2 }] = [1, 2]; } }

    struct S5;

    mod d {
        trait Trait { fn m1(&self); }
        impl Trait for ::S4 {
            fn m1(&self) { println!("Hello from <S4 as d::Trait>::m1"); }
        }
    }

    impl Trait for S4 {
        fn m1(&self) { println!("Hello from <S4 as Trait>::m1"); }
    }

    impl Trait for S5 {
        fn m1(&self) { println!("Hello from <S4 as Trait>::m1"); }
    }

    impl S1 {
        fn f() { println!("Hello from S1::f"); }
    }

    impl Trait for S1 {
        fn m1(&self) { println!("Hello from S1::m1"); }
        fn m2(&self) { println!("Hello from S1::m2"); }
    }

    mod e {
        fn f() {
            impl ::Trait for ::S2 {
                fn m1(&self) { println!("Hello from S2::m1"); }
                fn m2(&self) { println!("Hello from S2::m2"); }
            }
        }
    }

    println!("Hello from main");
}
