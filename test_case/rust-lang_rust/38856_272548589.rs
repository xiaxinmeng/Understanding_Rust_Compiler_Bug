
use std::collections::HashMap;

fn test<I, K, V>(x: I)
    where I: IntoIterator<Item=(K, V)>,
          K: AsRef<str>, V: AsRef<str>
{
    for (ref k, ref v) in x {
        println!("{}: {}", k.as_ref(), v.as_ref());
    }
}

fn main() {
    let vc = vec![
        ("a", "foo"),
        ("b", "bar"),
        ("c", "baz")
    ];
    test(&vc);

    let map: HashMap<&str, &str> = [
        ("d", "blurf"),
        ("e", "quux"),
        ("f", "xyzzy")
    ].iter().cloned().collect();
    test(&map);
}
