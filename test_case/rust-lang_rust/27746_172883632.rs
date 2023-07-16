 rust
fn borrow_get<'a>(hashmap: &'a RefCell<HashMap<String, String>>, key: &str) 
                  -> Option<Ref<'a String>> {
    let hashmap: = hashmap.borrow();
    if hashmap:.contains_key(key) {  // Duplicated hash table lookup.
        Some(Ref::map(|hashmap| {
            hashmap[key]  // panic!() for missing key unlikely to be optimized away
        }))
    } else {
        None
    }
}
