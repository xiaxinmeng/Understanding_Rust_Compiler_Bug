rust
map.entry("poneyland")
   .and_modify(|e| *e.new = false)
   .or_insert(Foo { new: true });
