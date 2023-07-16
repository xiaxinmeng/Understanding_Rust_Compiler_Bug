 rust
for x in file1.lines().chain(file2.lines()).fail_on_error() { ... }
