 rust
let mut lines = ~[];
for fileinput::input |slice : &str| {
     lines.push(str::from_slice(slice))
}
