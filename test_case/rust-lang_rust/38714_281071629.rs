rust
struct UsizeRef<'a> {
    a: &'a usize
}

type RefTo = Box<for<'r> Fn(&'r Vec<usize>) -> UsizeRef<'r>>;

//Compiles
fn ref_to<'a>(vec: &'a Vec<usize>) -> UsizeRef<'a> {
    UsizeRef{ a: &vec[0]}
}

fn main() {
    //Does not compile
    let a: RefTo = Box::new(|vec: &Vec<usize>| {
            UsizeRef{ a: &vec[0] }
        }
    );
}
