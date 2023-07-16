rust
#![feature(closure_lifetime_binder)]
#![feature(unboxed_closures)]

fn sort_by_key<T, F>(s: &mut [T], mut f: F)
where
    F: for<'a> FnMut<(&'a T,)>,
    // instead of `B: Ord`
    for<'a> <F as FnOnce<(&'a T,)>>::Output: Ord,
{
    s.sort_by(|a, b| f(a).cmp(&f(b)))
}

#[derive(Debug)]
struct Client(String);

impl Client {
    fn key(&self) -> &str {
        &self.0
    }
}

fn main() {
    let mut test = vec![
        Client("c".to_string()),
        Client("a".to_string()),
        Client("b".to_string()),
    ];
    sort_by_key(&mut test, for<'a> |c: &'a Client| -> &'a str { c.key() });
    dbg!(test);
}
