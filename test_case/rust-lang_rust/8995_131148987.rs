 rust

pub struct GenericStruct<A, B> {
    fa : A,
    fb : B,
}

struct GenericResult<A, B> {
    fa : A,
    fb : B,
}

impl <A, B> GenericStruct<A, B> {


    // I d like to write
    // type Result = Option<GenericResult<A, B>>;
    // and use it in the code

    fn new(a: A, b : B) -> Option<GenericResult<A, B>> {
        Some(GenericResult {fa: a, fb : b})

    }
}
