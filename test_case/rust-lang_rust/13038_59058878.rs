
// move version of collect
fn collect<T, I: Iterator<T>, C: FromIterator<T>>(mut it: I) -> C {
    it.collect()
}

fn main() {
    let mut arr = [-1i, -2, -3];

    println!("move version");
    {
        let it1 = arr.iter_mut();  // non-copy iterator
        let it2 = range(0i, 3);  // copy iterator

        let c1: Vec<_> = collect(it1);
        let c2: Vec<_> = collect(it2);
        println!("{}", (c1, c2));
        //let c1: Vec<_> = collect(it1);  // ~error: moved
        let c2: Vec<_> = collect(it2);
        println!("{}", ((), c2));
    }

    println!("\n&mut version (is this behavior surprising?)");
    {
        let mut it1 = arr.iter_mut();
        let mut it2 = range(0i, 3);

        let c1: Vec<_> = it1.collect();
        let c2: Vec<_> = it2.collect();
        println!("{}", (c1, c2));
        let c1: Vec<_> = it1.collect();
        let c2: Vec<_> = it2.collect();
        println!("{}", (c1, c2));
    }

    println!("\n&mut version emulated via the move version");
    {
        let mut it1 = arr.iter_mut();
        let mut it2 = range(0i, 3);

        let c1: Vec<_> = collect(it1.by_ref());
        let c2: Vec<_> = collect(it2.by_ref());
        println!("{}", (c1, c2));
        let c1: Vec<_> = collect(it1.by_ref());
        let c2: Vec<_> = collect(it2.by_ref());
        println!("{}", (c1, c2));
    }
}
