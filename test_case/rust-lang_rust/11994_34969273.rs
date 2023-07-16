 rust
macro_rules! struct_field {
    ($Struct:ident $($short:ident),*; $($long:ident: $long_e: expr)*) => {
        $Struct { $($short: $short, )* $($long: $long_e),* }
    }
}

// ...
struct_fields!(
    Url scheme, query, fragment; scheme_data: OtherSchemeData(scheme_data)
)
