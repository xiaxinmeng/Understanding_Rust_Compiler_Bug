
[nix-shell:/tmp/foo]$ wasmtime --mapdir .::. target/wasm32-wasi/release/foo.wasm --invoke foo
before read
read: Err(Custom { kind: Other, error: "failed to find a pre-opened file descriptor through which \"foo.txt\" could be opened" })
after read

[nix-shell:/tmp/foo]$ cat foo.txt
hello
