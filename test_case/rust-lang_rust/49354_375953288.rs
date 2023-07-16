rust
trait Visitor<'de> {
    type Value;
}

impl<'a, 'de: 'a> Visitor<'de> for &'a () {
    type Value = ();
}

//error: free region `'a` does not outlive free region `'de`
fn visit_seq<'de: 'a, 'a>() -> <&'a () as Visitor<'de>>::Value {}
//                                                             ^^

fn main() {}
