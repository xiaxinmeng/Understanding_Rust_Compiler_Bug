Rust


fn transmute_lifetime<'a, 'b, T>(t: &'a (T,)) -> &'b T {
    match (&t, ()) {
        ((t,), ()) => t,
    }
}

fn main() {
    let x = {
        let y = Box::new((42,));
        transmute_lifetime(&y)
    };

    println!("{}", x);
}
