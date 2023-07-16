
let map : HashMap<int, int> = ...;
let bar = 2;
// Don't need the address to bar, and also implements
// the Index<K, V> trait which would also be nice
let foo = map[bar];
// Perhaps there could be an trait for 'foo[k] = v' syntax?
map.insert(foo, 3);
// Don't need the address here again because '&4' looks kinda silly
map.remove(4);
