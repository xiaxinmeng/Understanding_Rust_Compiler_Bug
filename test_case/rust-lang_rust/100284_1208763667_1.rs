text
error[E0046]: not all trait items implemented, missing: `deserialize_any`, `deserialize_bool`, `deserialize_i8`, `deserialize_i16`, `deserialize_i32`, `deserialize_i64`, `deserialize_u8`, `deserialize_u16`, `deserialize_u32`, `deserialize_u64`, `deserialize_f32`, `deserialize_f64`, `deserialize_char`, `deserialize_str`, `deserialize_string`, `deserialize_bytes`, `deserialize_byte_buf`, `deserialize_option`, `deserialize_unit`, `deserialize_unit_struct`, `deserialize_newtype_struct`, `deserialize_seq`, `deserialize_tuple`, `deserialize_tuple_struct`, `deserialize_map`, `deserialize_struct`, `deserialize_enum`, `deserialize_identifier`, `deserialize_ignored_any`
  --> src/vm/de.rs:16:1
   |
16 | / impl<'de, Der> serde::Deserializer<'de> for RootDeserializer<Der>
17 | | where
18 | |     Der: serde::Deserializer<'de>
19 | | {
20 | |     type Error = Der::Error;
21 | | }
   | |_^ missing `deserialize_any`, `deserialize_bool`, `deserialize_i8`, `deserialize_i16`, `deserialize_i32`, `deserialize_i64`, `deserialize_u8`, `deserialize_u16`, `deserialize_u32`, `deserialize_u64`, `deserialize_f32`, `deserialize_f64`, `deserialize_char`, `deserialize_str`, `deserialize_string`, `deserialize_bytes`, `deserialize_byte_buf`, `deserialize_option`, `deserialize_unit`, `deserialize_unit_struct`, `deserialize_newtype_struct`, `deserialize_seq`, `deserialize_tuple`, `deserialize_tuple_struct`, `deserialize_map`, `deserialize_struct`, `deserialize_enum`, `deserialize_identifier`, `deserialize_ignored_any` in implementation
   |
   = help: implement the missing item: `fn deserialize_any<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_bool<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_i8<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_i16<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_i32<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_i64<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_u8<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_u16<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_u32<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_u64<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_f32<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_f64<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_char<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_str<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_string<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_bytes<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_byte_buf<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_option<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_unit<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_seq<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_map<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_struct<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_identifier<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
   = help: implement the missing item: `fn deserialize_ignored_any<V>(self, _: V) -> Result<<V as Visitor<'de>>::Value, <Self as serde::Deserializer<'de>>::Error> where V: Visitor { todo!() }`
