rust
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let mut hm: HashMap<Rc<str>, ()> = HashMap::new();
    hm.insert("foo".into(), ());
    println!("{:?}", hm.get("foo"));
    // output = Some(())
}
