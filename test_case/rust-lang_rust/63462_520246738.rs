plain
2019-08-11T17:10:42.1227826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T17:10:42.1406749Z ##[command]git config gc.auto 0
2019-08-11T17:10:43.1011215Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T17:10:43.1016054Z ##[command]git config --get-all http.proxy
2019-08-11T17:10:43.1021687Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63462/merge:refs/remotes/pull/63462/merge
---
2019-08-11T17:11:14.8932824Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T17:11:14.8932878Z 
2019-08-11T17:11:14.8933114Z   git checkout -b <new-branch-name>
2019-08-11T17:11:14.8933149Z 
2019-08-11T17:11:14.8933204Z HEAD is now at 2591dcdb7 Merge 1c8d77cefefefbdc1d8c37cd0e5fc623bd8e09cd into 2b78e10ac1454d2d4190c575f6ece03f484ac398
2019-08-11T17:11:14.9083080Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T17:11:14.9085952Z ==============================================================================
2019-08-11T17:11:14.9086007Z Task         : Bash
2019-08-11T17:11:14.9086069Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T17:42:02.8139034Z    Compiling rustc_version v0.2.3
2019-08-11T17:42:03.5808473Z    Compiling lock_api v0.1.3
2019-08-11T17:42:04.4142154Z    Compiling polonius-engine v0.9.0
2019-08-11T17:42:05.8494374Z    Compiling chalk-engine v0.9.0
2019-08-11T17:42:10.0880307Z error[E0403]: the name `E` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.0880853Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/de/impls.rs:2456:34
2019-08-11T17:42:10.0881127Z      |
2019-08-11T17:42:10.0881475Z 2423 | impl<'de, T, E> Deserialize<'de> for Result<T, E>
2019-08-11T17:42:10.0881822Z      |              - first use of `E`
2019-08-11T17:42:10.0882062Z ...
2019-08-11T17:42:10.0882433Z 2456 |                     fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
2019-08-11T17:42:10.0882944Z      |                                  ^ already used
2019-08-11T17:42:10.0883000Z 
2019-08-11T17:42:10.0890557Z error[E0403]: the name `E` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.0891414Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/de/impls.rs:2470:34
2019-08-11T17:42:10.0891902Z      |
2019-08-11T17:42:10.0892385Z 2423 | impl<'de, T, E> Deserialize<'de> for Result<T, E>
2019-08-11T17:42:10.0892901Z      |              - first use of `E`
2019-08-11T17:42:10.0893447Z ...
2019-08-11T17:42:10.0893922Z 2470 |                     fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
2019-08-11T17:42:10.0894654Z      |                                  ^ already used
2019-08-11T17:42:10.0895173Z 
2019-08-11T17:42:10.0896166Z error[E0403]: the name `E` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.0896808Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/de/impls.rs:2481:36
2019-08-11T17:42:10.0897173Z      |
2019-08-11T17:42:10.0897829Z 2423 | impl<'de, T, E> Deserialize<'de> for Result<T, E>
2019-08-11T17:42:10.0898288Z      |              - first use of `E`
2019-08-11T17:42:10.0898671Z ...
2019-08-11T17:42:10.0904428Z 2481 |                     fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
2019-08-11T17:42:10.0904835Z      |                                    ^ already used
2019-08-11T17:42:10.0904905Z 
2019-08-11T17:42:10.1034887Z error[E0403]: the name `V` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.1035241Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/private/de.rs:30:28
2019-08-11T17:42:10.1035484Z    |
2019-08-11T17:42:10.1035767Z 17 | pub fn missing_field<'de, V, E>(field: &'static str) -> Result<V, E>
2019-08-11T17:42:10.1036062Z    |                           - first use of `V`
2019-08-11T17:42:10.1036546Z ...
2019-08-11T17:42:10.1036848Z 30 |         fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, E>
2019-08-11T17:42:10.1037140Z    |                            ^ already used
2019-08-11T17:42:10.1037197Z 
2019-08-11T17:42:10.1044251Z error[E0403]: the name `V` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.1044967Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/private/de.rs:37:31
2019-08-11T17:42:10.1045392Z    |
2019-08-11T17:42:10.1045694Z 17 | pub fn missing_field<'de, V, E>(field: &'static str) -> Result<V, E>
2019-08-11T17:42:10.1046029Z    |                           - first use of `V`
2019-08-11T17:42:10.1046272Z ...
2019-08-11T17:42:10.1046585Z 37 |         fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, E>
2019-08-11T17:42:10.1047295Z    |                               ^ already used
2019-08-11T17:42:10.1047337Z 
2019-08-11T17:42:10.1047800Z error[E0403]: the name `V` is already used for a generic parameter in this item's generic parameters
2019-08-11T17:42:10.1056032Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/macros.rs:118:57
2019-08-11T17:42:10.1056291Z     |
2019-08-11T17:42:10.1056593Z 112 | / macro_rules! forward_to_deserialize_any {
2019-08-11T17:42:10.1057092Z 113 | |     (<$visitor:ident: Visitor<$lifetime:tt>> $($func:ident)*) => {
2019-08-11T17:42:10.1057493Z 114 | |         $(forward_to_deserialize_any_helper!{$func<$lifetime, $visitor>})*
2019-08-11T17:42:10.1058047Z ...   |
2019-08-11T17:42:10.1058047Z ...   |
2019-08-11T17:42:10.1058367Z 118 | |         $(forward_to_deserialize_any_helper!{$func<'de, V>})*
2019-08-11T17:42:10.1058743Z     | |                                                         ^ already used
2019-08-11T17:42:10.1059351Z 120 | | }
2019-08-11T17:42:10.1059351Z 120 | | }
2019-08-11T17:42:10.1060188Z     | |_- in this expansion of `forward_to_deserialize_any!`
2019-08-11T17:42:10.1060452Z     | 
2019-08-11T17:42:10.1060826Z    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.92/src/private/de.rs:17:27
2019-08-11T17:42:10.1061077Z     |
2019-08-11T17:42:10.1061422Z 17  |   pub fn missing_field<'de, V, E>(field: &'static str) -> Result<V, E>
2019-08-11T17:42:10.1061803Z     |                             - first use of `V`
2019-08-11T17:42:10.1062217Z ...
2019-08-11T17:42:10.1062592Z 44  | /         forward_to_deserialize_any! {
2019-08-11T17:42:10.1063247Z 45  | |             bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
2019-08-11T17:42:10.1063594Z 46  | |             bytes byte_buf unit unit_struct newtype_struct seq tuple
2019-08-11T17:42:10.1063955Z 47  | |             tuple_struct map struct enum identifier ignored_any
2019-08-11T17:42:10.1064583Z     | |_________- in this macro invocation
2019-08-11T17:42:10.1064634Z 
2019-08-11T17:42:10.6517053Z    Compiling num_cpus v1.8.0
2019-08-11T17:42:12.2405527Z    Compiling jobserver v0.1.16
---
2019-08-11T17:42:15.6817089Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-11T17:42:15.6817637Z expected success, got: exit code: 101
2019-08-11T17:42:15.6828957Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-11T17:42:15.6829171Z Build completed unsuccessfully in 0:24:49
2019-08-11T17:42:17.0384582Z ##[error]Bash exited with code '1'.
2019-08-11T17:42:17.0425723Z ##[section]Starting: Checkout
2019-08-11T17:42:17.0427720Z ==============================================================================
2019-08-11T17:42:17.0427775Z Task         : Get sources
2019-08-11T17:42:17.0427823Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
