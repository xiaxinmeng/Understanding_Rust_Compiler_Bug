rs

use std::any::Any;
use once_cell::sync::OnceCell;

fn main() {
    let s = Set::<Wrapper<for<'a> fn(&'a str)>>::new();
    let mut s: Set<Wrapper<fn(&'static str)>> = s;
    static S: OnceCell<&'static str> = OnceCell::new();
    let f = |s| {
        let _ = S.try_insert(s);
    };
    s.insert(Wrapper(f));
    s.contains(&Wrapper(f));
    println!("{}", S.get().unwrap());
}

// for the record, BTreeSet is indeed covariant, too
use std::collections::BTreeSet;
fn _b_tree_set_is_covariant<'a: 'b, 'b, T>(x: BTreeSet<&'a T>) -> BTreeSet<&'b T> {
    x
}

struct Wrapper<T>(T);

impl<T: 'static> Ord for Wrapper<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        if let Some(f) = (self as &dyn Any).downcast_ref::<Wrapper<for<'a> fn(&'a str)>>() {
            f.0(&String::from("Hello world!"));
        }
        Ordering::Equal
    }
}
impl<T: 'static> PartialOrd<Self> for Wrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: 'static> Eq for Wrapper<T> {}
impl<T: 'static> PartialEq<Self> for Wrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
