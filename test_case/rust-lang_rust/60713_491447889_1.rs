rust
// src/lib.rs
#[macro_use]
extern crate juniper_codegen;
mod value {
    mod scalar {
        pub trait ScalarValue {}
        pub trait ScalarRefValue<'a> {}
    }
    pub use self::scalar::{
        DefaultScalarValue, ParseScalarResult, ParseScalarValue, ScalarRefValue, ScalarValue,
    };
}
mod ast {
    pub trait FromInputValue<S = DefaultScalarValue>: Sized {}
    pub trait ToInputValue<S = DefaultScalarValue>: Sized {}
}
mod executor {
    use schema::meta::{
        Argument, DeprecationStatus, EnumMeta, EnumValue, Field, InputObjectMeta, InterfaceMeta,
    };
    pub struct Registry<'r, DefaultScalarValue> {
        types: FnvHashMap,
    }
    impl<'r, S> Registry<'r, S>
    where
        S: 'r,
    {
        pub fn build_enum_type(&mut self) -> EnumMeta<'r, S> {
            unimplemented!()
        }
    }
}
mod schema {
    pub mod meta {
        pub struct EnumMeta<'a, S> {
            pub name: Cow,
            pub description: Option<String>,
            pub values: VecEnumValue,
            pub try_parse_fn: fn() -> bool,
        }
    }
}
mod types {
    pub mod base {
        use value::{DefaultScalarValue, Object, ScalarRefValue, ScalarValue, Value};
        #[doc = " of a type."]
        #[derive(Clone, Eq, PartialEq, Debug, GraphQLEnumInternal)]
        pub enum TypeKind {
            NonNull,
        }
        pub trait GraphQLType<S = DefaultScalarValue>: Sized
        where
            S: ScalarValue,
            for<'b> &'b S: ScalarRefValue<'b>,
        {
        }
    }
}
use ast::{FromInputValue, ToInputValue};
use executor::Registry;
use types::base::GraphQLType;
use value::{ScalarRefValue, ScalarValue};
