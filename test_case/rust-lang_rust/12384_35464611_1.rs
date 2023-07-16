 rust
println!(some_variable); // ok, omitted format string on purpose

println!(&mut writer, "a {} b", 1); // oops, was meant to be `writeln!`
