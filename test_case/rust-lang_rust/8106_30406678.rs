 rust
// optimized to ret 0
fn foo(i: uint) -> uint {if i % 5 == 1 {if i % 4 == 0 {i % 4}else{0}}else{0}}
// not optimized to ret 0
fn foo(i: uint) -> uint {if i % 5 == 1 && i % 4 == 0 {i % 4}else{0}}
