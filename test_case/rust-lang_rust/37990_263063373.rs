
let iter = FlatMap([2, 2] None [4, 4, 4, 4] [6, 6, 6, 6, 6, 6]);
iter.next();
iter.next_back();
iter.next();
iter.next(); // fuses the underlying iter.
// No matter what you do here you won't see 4 yielded, even though to my untrained eye it seems like should be handled just fine.
