 rust
for x in file1.lines().fail_on_error().chain(file2.lines().fail_on_error()) { ... }
