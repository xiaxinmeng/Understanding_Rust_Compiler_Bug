
hang.rs:8:1: 10:2 error: reached the recursion limit during monomorphization
hang.rs: 8 fn recurse<T>() {
hang.rs: 9     recurse::<<(T, T) as Mirror>::Image>();
hang.rs:10 }
