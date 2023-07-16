rust
// this works fine
fn foo3((a, b): &(i32, i32)) {
}

// this doesn't
fn foo4((&a, &b): &(i32, i32)) {                                                                                                                                                                                                                } 
