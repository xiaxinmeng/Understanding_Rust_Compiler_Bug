rust
extern crate serde_json; // 1.0.41

pub enum MyEnum {
    Variant,
}

impl Into<u16> for MyEnum {
    fn into(self) -> u16 {
        0
    }
}

fn blubb() {
    // for as long as serde_json is mentioned in this file:
    // type annotations required: cannot resolve `MyEnum: std::convert::Into<_>`
    assert_eq!(0u16, MyEnum::Variant.into());
    
    // fine
    // assert_eq!(0u16, <MyEnum as Into<u16>>::into(MyEnum::Variant));
    
    // in case of no "extern crate"
    // serde_json::to_string_pretty(&"");
}
