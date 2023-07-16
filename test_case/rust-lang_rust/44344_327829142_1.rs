rust
let option = Some("foo");                                        
let len_or_zero = option.map_or_default(str::len);
