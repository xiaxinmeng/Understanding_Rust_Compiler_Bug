rust
fn fails<'a, 'b: 'a, T: Trait>(entity: AssocAlias<'a, T>) -> &'a i32 {&5}
fn fails<'a, T: Trait + 'a>(entity: AssocAlias<'a, T>) -> &'a i32 {&5}
