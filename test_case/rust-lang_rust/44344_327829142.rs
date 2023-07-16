rust
let option = Some("foo");                                        
let len_or_zero = option.map(str::len).unwrap_or_default();      
