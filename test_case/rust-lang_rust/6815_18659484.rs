
map.insert(key, map.pop(&key).map_default(0, |x| *x + 1))
