rust
assert!(contains_matching_babs(&abas, &babs));
assert!(contains_matching_babs(&babs, &abas));
assert!(!contains_matching_babs(&abas, &cdcs));
assert!(!contains_matching_babs(&cdcs, &abas));
