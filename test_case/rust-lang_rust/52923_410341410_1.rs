rust
pub const A: usize = 0;

pub mod foo {
    pub const B: usize = 1;

    pub mod bar {
        pub const C: usize = 2;

        pub enum E {
            V1(usize),
            V2(String),
        }

        pub fn test() {
            println!("{} {} {}", crate::A, crate::foo::B, C);
        }

        pub fn test_use() {
            use crate::A;
            use crate::foo::B;

            println!("{} {} {}", A, B, C);
        }

        pub fn test_enum() {
            use E::*;
            match E::V1(10) {
                V1(i) => { println!("V1: {}", i); }
                V2(s) => { println!("V2: {}", s); }
            }
        }
    }

    pub fn test() {
        println!("{} {} {}", crate::A, B, bar::C);
    }

    pub fn test_use() {
        use crate::A;
        use bar::C;

        println!("{} {} {}", A, B, C);
    }

    pub fn test_enum() {
        use bar::E::*;
        match bar::E::V1(10) {
            V1(i) => { println!("V1: {}", i); }
            V2(s) => { println!("V2: {}", s); }
        }
    }
}

pub fn test() {
    println!("{} {} {}", A, foo::B, foo::bar::C);
}

pub fn test_use() {
    use foo::B;
    use foo::bar::C;

    println!("{} {} {}", A, B, C);
}

pub fn test_enum() {
    use foo::bar::E::*;
    match foo::bar::E::V1(10) {
        V1(i) => { println!("V1: {}", i); }
        V2(s) => { println!("V2: {}", s); }
    }
}

fn main() {
    test();
    foo::test();
    foo::bar::test();
    test_use();
    foo::test_use();
    foo::bar::test_use();
    test_enum();
    foo::test_enum();
    foo::bar::test_enum();
}
