
// somewhere in core
iface eq { fn eq (a: self) -> bool; }
iface hash { fn hash () -> uint; }

// a set module constructor
fn mk_set <V:eq hash copy> () -> set<V> { ... }

// to construct a set of some type with eq and hash in scope:
let s = set::mk_set();
s.insert(...);
