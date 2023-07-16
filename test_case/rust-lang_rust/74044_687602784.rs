rust
struct MyNum(u8);

struct MyOtherNum(u8);

impl PartialEq<u8> for MyNum {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<MyOtherNum> for MyNum {
    fn eq(&self, other: &MyOtherNum) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&u8> for MyNum {
    fn eq(&self, other: &&u8) -> bool {
        self.0 == **other
    }
}

impl PartialEq<&MyOtherNum> for MyNum {
    fn eq(&self, other: &&MyOtherNum) -> bool {
        self.0 == other.0
    }
}

// Same functionality as the `contains_ref` method in the PR.
fn search<T, U>(sl: &[T], other: U) -> bool
                            //   ^ takes `U` instead of `&U`
where
    T: PartialEq<U>,
    // No `U: ?Sized` here
{
    sl.iter().any(|e| e == &other)
                            // ^ uses a `&` here
}

fn main() {
    let v = [String::from("Hello"), String::from("world")];
    assert!(search(&v, "Hello"));
    assert!(!search(&v, "Rust"));

    let v = [0, 2];
    assert!(search(&v, 0));
    assert!(!search(&v, 1));

    let v = [MyNum(0), MyNum(2)];
    assert!(search(&v, 0));
    assert!(!search(&v, 1));
    assert!(search(&v, &0));
    assert!(!search(&v, &1));
    assert!(search(&v, MyOtherNum(0)));
    assert!(!search(&v, MyOtherNum(2)));
    assert!(search(&v, &MyOtherNum(0)));
    assert!(!search(&v, &MyOtherNum(2)));
}
