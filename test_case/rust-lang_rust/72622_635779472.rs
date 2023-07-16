rust
macro_rules! regurgitate_struct_def {
    (
        $( #[$struct_attr:meta] )*
        $item_vis:vis struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                $field_vis:vis $field:ident : $type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        $item_vis struct $name {
            $(
                $( #[$field_attr] )*
                $field_vis $field: $type,
            )*
        }
    )
}

regurgitate_struct_def! {
    #[derive(serde_derive::Deserialize)]
    pub struct Foo {
        _phantom: (),
    }
}
