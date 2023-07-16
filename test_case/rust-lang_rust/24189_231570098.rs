 rust
macro_rules! serializable {
    (
        $(#[$struct_meta:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type_:ty
            ),* ,
        }
    ) => {
        $(#[$struct_meta])*
        pub struct $name {
            $(
                $(#[$field_meta])*
               $field: $type_
            ),* ,
        }
    }
}

serializable! {
    #[allow(dead_code)]
    /// This is a test
    pub struct Tester {
        #[allow(dead_code)]
        name: String,
    }
}
