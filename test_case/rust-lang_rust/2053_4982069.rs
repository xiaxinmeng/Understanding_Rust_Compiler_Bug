
use std;
import std::map;
import map::hashmap;

type map1 = map::hashmap<str, @t>;
type map2 = map::hashmap<str, ~t>;

type t = {
    s: str
};

fn main(args: [str])
{
    fn nover<T: copy, U: copy>(m: map::hashmap<T, U>, k: T, v: U) {
        if ! m.insert(k, v) {
            fail;
        }
    }

    fn f1(m: map1) {
        nover(m, "x", @{
            s: "pq"
        });
    };

    fn f2(m: map2) {
        nover(m, "27", ~{
            s : "blah"
        });
    };

    let m : map2 = map::str_hash();
    f2(m);
}
