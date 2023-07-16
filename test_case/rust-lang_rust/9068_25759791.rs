
let mut table: TreeMap<~str, ~[int]> = TreeMap::new();
let key = ~"test1";

match table.find_mut(&key) {
    None    => table.insert(key.clone(), ~[1]),
    Some(v) => { v.push(1); false }
};
