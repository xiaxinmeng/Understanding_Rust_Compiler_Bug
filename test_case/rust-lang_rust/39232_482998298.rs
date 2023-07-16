rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

let mut hm: HashMap<&str, i32> = HashMap::new();
hm.insert("myKey1", 100);
hm.insert("myKey2", 200);

let rc = Rc::new(RefCell::new(hm));
let v1 = rc.borrow_mut().get("myKey1").unwrap().clone();
println!("read {} into v1!", v1);
