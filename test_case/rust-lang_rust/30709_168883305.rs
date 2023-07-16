
rustc A.rs --cfg b 
rustc B.rs -L . --cfg b
rustc C_wrong.rs -L . --cfg b
rustc A.rs
rustc C_wrong.rs -L .
