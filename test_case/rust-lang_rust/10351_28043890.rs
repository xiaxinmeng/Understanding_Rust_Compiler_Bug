
let names = ["Alice", "Bob", "Catherine"];

for (i, &name) in names.iter().enumerate() {
    println!("{} {}", i, name);
}
