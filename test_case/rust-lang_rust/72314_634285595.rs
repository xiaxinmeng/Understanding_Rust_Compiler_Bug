rust
    let _ = Struct1 { field: 1 }; // ERROR use of unstable library feature 'unstable_default'
    let _: Struct1 = Struct1 { field: 1 }; // ERROR use of unstable library feature 'unstable_default'
    let _: usize = STRUCT1.field; // ERROR use of unstable library feature 'unstable_default'
    let _ = STRUCT1.field + 1; // ERROR use of unstable library feature 'unstable_default'
