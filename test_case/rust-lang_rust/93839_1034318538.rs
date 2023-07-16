plain
Successfully built b94b4da0ef52
Successfully tagged rust-ci:latest
Built container sha256:b94b4da0ef5270c01f0ba10a2363b797eda9c22c2c3272d641f5b1d4b828e5a8
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 70 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....FF.............FF...FF............................................

---- [ui] ui-fulldeps/deriving-encodable-decodable-box.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-encodable-decodable-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-box/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-box/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find function `decode` in module `json`
   |
   |
LL |       let obj2: A = json::decode(&s);
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:284:1
   |
   |
LL | / pub fn encode<T: for<'r> crate::Encodable<Encoder<'r>>>(
LL | |     object: &T,
LL | | ) -> Result<string::String, EncoderError> {
   | |_________________________________________- similarly named function `encode` defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.


------------------------------------------


---- [ui] ui-fulldeps/deriving-encodable-decodable-cell-refcell.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deriving-encodable-decodable-cell-refcell/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find function `decode` in module `json`
   |
   |
LL |       let obj2: B = json::decode(&s);
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:284:1
   |
   |
LL | / pub fn encode<T: for<'r> crate::Encodable<Encoder<'r>>>(
LL | |     object: &T,
LL | | ) -> Result<string::String, EncoderError> {
   | |_________________________________________- similarly named function `encode` defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.


------------------------------------------


---- [ui] ui-fulldeps/issue-14021.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-14021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-14021/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-14021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: could not find `Decoder` in `json`
   |
   |
LL |     let mut decoder = json::Decoder::new(json_object.unwrap());
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL |     let mut decoder = json::Encoder::new(json_object.unwrap());
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
---
---- [ui] ui-fulldeps/issue-24972.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-24972.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-24972/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-24972/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `Decoder` in module `json`
   |
   |
LL | pub trait Entity: Decodable<json::Decoder> + for<'a> Encodable<json::Encoder<'a>> + Sized {
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL | pub trait Entity: Decodable<json::Encoder> + for<'a> Encodable<json::Encoder<'a>> + Sized {
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
   |
LL | use rustc_serialize::opaque::Decoder;
   |

error[E0412]: cannot find type `Decoder` in module `json`
   |
   |
LL |         + Decodable<json::Decoder>
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL |         + Decodable<json::Encoder>
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
---
---- [ui] ui-fulldeps/issue-4016.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-4016.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-4016/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-4016/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `Decoder` in module `json`
   |
   |
LL | trait JD: Decodable<json::Decoder> {}
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL | trait JD: Decodable<json::Encoder> {}
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
   |
LL | use rustc_serialize::opaque::Decoder;
   |

error[E0433]: failed to resolve: could not find `Decoder` in `json`
   |
   |
LL |     let mut decoder = json::Decoder::new(doc);
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL |     let mut decoder = json::Encoder::new(doc);
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
   |
LL | use rustc_serialize::opaque::Decoder;
   |

error[E0282]: type annotations needed
  --> /checkout/src/test/ui-fulldeps/issue-4016.rs:13:34
   |
LL |     let doc = json::from_str("").unwrap();
   |
   = note: type must be known at this point


error[E0599]: no method named `unwrap` found for enum `Result<Json, ParserError>` in the current scope
   |
   |
LL |     let doc = json::from_str("").unwrap();
   |                                  ^^^^^^ method not found in `Result<Json, ParserError>`
   = note: the method was found for
           - `Result<T, E>`
           - `Result<T, E>`
   = note: `json::from_str("")` is a function, perhaps you wish to call it
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0282, E0412, E0433, E0599.
For more information about an error, try `rustc --explain E0282`.
---
---- [ui] ui-fulldeps/issue-4036.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-4036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-4036/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-4036/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: could not find `Decoder` in `json`
   |
   |
LL |     let mut decoder = json::Decoder::new(json);
   |
  ::: /checkout/compiler/rustc_serialize/src/json.rs:419:1
   |
   |
LL | pub struct Encoder<'a> {
   | ---------------------- similarly named struct `Encoder` defined here
help: a struct with a similar name exists
   |
   |
LL |     let mut decoder = json::Encoder::new(json);
help: consider importing one of these items
   |
LL | use rustc_serialize::Decoder;
   |
