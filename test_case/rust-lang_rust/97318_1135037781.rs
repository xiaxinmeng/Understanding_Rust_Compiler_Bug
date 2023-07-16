
cargo bisect-rustc -vv --regress=success --preserve --test-dir=./ffi/op-core-bindings --start=1.59.0

...

searched toolchains 02822334e9bd22df33c0692cbaade5a5b5449130 through ef9b49881ba99248b68dbdebbebd50155587c509

********************************************************************************
Regression in 18b53cefdf7456bf68937b08e377b7e622a115c2
********************************************************************************
