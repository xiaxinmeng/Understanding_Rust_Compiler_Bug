rust
enum MyEnum {
    Variant,
}

let _ = matches!(MyEnum::Variant, MyEnum::Variant); // this should lint
