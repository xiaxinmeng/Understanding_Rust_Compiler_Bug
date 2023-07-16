sh
#!/bin/bash
echo 'fn main() {}' > rust.rs

rustc                      foo.rs --target wasm32-unknown-unknown
`rustup ${TC} which rustc` foo.rs --target wasm32-unknown-unknown
