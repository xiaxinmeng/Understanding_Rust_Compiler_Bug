
map = BPlusTreeMap::new()
// loads of inserts
let a = map.remove("I'm in a branch key also");
drop(a)
map.get("blahblah") // might segfault as a underlining data  won't be there
