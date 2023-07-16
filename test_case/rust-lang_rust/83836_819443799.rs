sh
echo 'fn foo() -> std::iter::Map<Item = i32> { unimplemented!() }' | rustc - --crate-type=lib
