rs
trait Def {
    fn def() -> Self;
}

impl<T> Def for std::marker::PhantomData<T> {
    fn def() -> Self {
        std::marker::PhantomData
    }
}

fn extend_with_proof<'a, 'b>(
    _proof: std::marker::PhantomData<&'b &'a ()>,
    x: &'a str,
) -> &'b str {
    x
}

fn call_with_fake_proof<'a, 'b, T: Def>(
    extend_with_proof_ptr: fn(_proof: T, &'a str) -> &'b str,
    s: &'a str,
) -> &'b str {
    extend_with_proof_ptr(Def::def(), s)
}

fn main() {
    let r = {
        let s = String::from("Hello World!");
        call_with_fake_proof(extend_with_proof, &s)
    };
    println!("{r}", r=r);
}
