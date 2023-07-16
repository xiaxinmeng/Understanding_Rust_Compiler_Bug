
error[E0271]: type mismatch resolving `<fn(Guild, Invoker, Option<Vec<HashMap<std::string::String, std::string::String>>>) -> impl futures_util::Future<Output = Vec<std::string::String>> {send_dog} as FnOnce<(Guild, Invoker, Option<Vec<HashMap<std::string::String, std::string::String>>>)>>::Output == Pin<Box<(dyn futures_util::Future<Output = Vec<std::string::String>> + std::marker::Send + 'static)>>`
  --> src/actions/commands.rs:60:97
   |
60 |                         .add("dog".to_string(), 1, "Sends a picture of a cute dog".to_string(), Box::new(send_dog));
   |                                                                                                 ^^^^^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
   |
note: while checking the return type of the `async fn`
  --> src/actions/commands.rs:51:98
   |
51 | async fn send_dog(guild: Guild, invoker: Invoker, args: Option<Vec<HashMap<String, String>>>) -> Vec<String> {
   |                                                                                                  ^^^^^^^^^^^ checked the `Output` of this `async fn`, found opaque type
   = note:   expected struct `Pin<Box<(dyn futures_util::Future<Output = Vec<std::string::String>> + std::marker::Send + 'static)>>`
           found opaque type `impl futures_util::Future<Output = Vec<std::string::String>>`
   = note: required for the cast from `fn(Guild, Invoker, Option<Vec<HashMap<std::string::String, std::string::String>>>) -> impl futures_util::Future<Output = Vec<std::string::String>> {send_dog}` to the object type `dyn Fn(Guild, Invoker, Option<Vec<HashMap<std::string::String, std::string::String>>>) -> Pin<Box<(dyn futures_util::Future<Output = Vec<std::string::String>> + std::marker::Send + 'static)>>
