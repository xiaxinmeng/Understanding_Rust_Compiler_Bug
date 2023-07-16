 rust
fn test() {
   let results: Vec<Vec<u8>> = vec![];
   for vec in get_data_sets() {
      let map = HashMap::new();
      for k in vec {
        map.insert(k, compute(k));
      }
      results.push(map.values().cloned().collect()); 
   }
   assert!(results[0] == results[1]);
}
