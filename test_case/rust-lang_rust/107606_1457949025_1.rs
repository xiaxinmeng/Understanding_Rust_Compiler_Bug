rust
struct NonZeroI32(i32 is ..0 | 1..);

println!("{}", size_of::<Option<NonZeroI32>>()); // Prints "4"
println!("{}", size_of::<Option<i32 is ..0 | 1..>>(); // Prints "5"
