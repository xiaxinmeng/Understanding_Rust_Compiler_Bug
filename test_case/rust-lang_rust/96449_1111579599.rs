rs
#[allow(dead_code)]
enum InnerB {
    // V0(u32), // < uncomment this on 2nd compile
}

#[allow(unused)]
fn use_me() {
    // if let Outer::B(..) = loop {} {} // < uncomment this on 2nd compile
}

pub fn execute() -> u32 {
    fn any_outer<'a>() -> &'a Outer {
        loop {}
    }

    match any_outer() {
        // replacing v0 with v1 gets the same result
        Outer::A(InnerA::V0(_)) => loop {},
        Outer::B(InnerA::V0(_)) => loop {},
        _ if false => loop {},
        _ => loop {},
    };
}

#[allow(dead_code)]
enum Outer {
    A(InnerA),
    B(InnerA),
    C(InnerB),
}

#[allow(dead_code)]
enum InnerA {
    V0(u32),
    V1(u32),
}
