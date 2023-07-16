
use std::ffi::OsStr;
use std::collections::HashMap;
use std::env;

fn test<I, K, V>(x: &I)
    where I: IntoIterator<Item=(K, V)>,
          K: AsRef<OsStr>, V: AsRef<OsStr>
{
    for (ref k, ref v) in *x {
        println!("{:?}: {:?}", k.as_ref(), v.as_ref());
    }
}

fn test_vec() {
    let filtered_env : Vec<(String, String)> =
        env::vars().filter(
            | &(ref k, _) |
            k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
        ).collect();

    test(&filtered_env);
}

fn test_hashmap() {
    let mut map = HashMap::new();
    map.insert("a", "foo");
    map.insert("b", "bar");
    map.insert("c", "baz");

    test(&map);
}

fn main() {
    test_vec();
    test_hashmap();
}
