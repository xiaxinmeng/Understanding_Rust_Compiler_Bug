rust
 let p_ids = vec!["sample1", "sample2", "sample3", "sample4"];

 let c_keys: Vec<&str> = p_ids
        .iter()
        .map(|x| format!("sample!_{}", x))
        .map(|x| x.as_ref()) 
        .collect();
