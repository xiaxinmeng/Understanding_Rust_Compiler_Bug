rust
    let p_ids = vec!["sample1", "sample2", "sample3", "sample4"];
    let c_keys: Vec<String> = p_ids
        .iter()
        .map(|x| format!("sample!_{}", x))
        .collect();
    let c_keys_ref: Vec<&str> = c_keys
        .iter()
        .map(|x| x.as_ref())
        .collect();
