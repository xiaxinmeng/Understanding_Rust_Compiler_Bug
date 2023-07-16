rust
enum Enum { Variant }

fn f() -> Enum::Variant // successfully resolved to variant, but enum is expected
{ ... }
