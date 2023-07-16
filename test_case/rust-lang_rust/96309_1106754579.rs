rust
if !HASHMAP.lock().unwrap().insert(def_id) {
    eprintln!("+> {:?}", def_id);
}
