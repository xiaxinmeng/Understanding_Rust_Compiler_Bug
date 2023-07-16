plain
running 3 tests
FF.
failures:

---- src/json.rs - json (line 140) stdout ----
error[E0425]: cannot find function `decode` in module `json`
    |
    |
40  |   let decoded: TestStruct = json::decode(&json_str);
    |
   ::: /checkout/compiler/rustc_serialize/src/json.rs:284:1
    |
    |
284 | / pub fn encode<T: for<'r> crate::Encodable<Encoder<'r>>>(
285 | |     object: &T,
286 | | ) -> Result<string::String, EncoderError> {
    | |_________________________________________- similarly named function `encode` defined here
help: a function with a similar name exists
    |
    |
40  | let decoded: TestStruct = json::encode(&json_str);
help: consider importing this function
    |
5   | use core::num::flt2dec::decode;
    |
    |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/json.rs - json (line 69) stdout ----
error[E0425]: cannot find function `decode` in module `json`
    |
    |
27  |   let decoded: TestStruct = json::decode(&encoded[..]);
    |
   ::: /checkout/compiler/rustc_serialize/src/json.rs:284:1
    |
    |
284 | / pub fn encode<T: for<'r> crate::Encodable<Encoder<'r>>>(
285 | |     object: &T,
286 | | ) -> Result<string::String, EncoderError> {
    | |_________________________________________- similarly named function `encode` defined here
help: a function with a similar name exists
    |
    |
27  | let decoded: TestStruct = json::encode(&encoded[..]);
help: consider importing this function
    |
5   | use core::num::flt2dec::decode;
    |
