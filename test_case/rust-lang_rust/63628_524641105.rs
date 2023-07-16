
[INFO] [stderr]    Compiling custom-slice-macros v0.1.1 (/opt/crater/workdir)
[INFO] [stderr] error: proc macro panicked
[INFO] [stderr]    --> tests/derives.rs:48:13
[INFO] [stderr]     |
[INFO] [stderr] 48  | /             custom_slice_macros::define_slice_types_pair! {
[INFO] [stderr] 49  | |                 #[custom_slice(owned)]
[INFO] [stderr] 50  | |                 $(#[$meta_owned])*
[INFO] [stderr] 51  | |                 struct Owned($owned_inner);
[INFO] [stderr] ...   |
[INFO] [stderr] 59  | |                 fn validate(_: &$slice_inner) -> Result<(), $ty_error> $body
[INFO] [stderr] 60  | |             }
[INFO] [stderr]     | |_____________^
[INFO] [stderr] ...
[INFO] [stderr] 227 | /     gen_test! {
[INFO] [stderr] 228 | |         name: try_from_inner,
[INFO] [stderr] 229 | |         #[custom_slice(derive(TryFromInner))]
[INFO] [stderr] 230 | |         #[custom_slice(error(r#type = "()"))]
[INFO] [stderr] ...   |
[INFO] [stderr] 236 | |         validator: () { Ok(()) },
[INFO] [stderr] 237 | |     }
[INFO] [stderr]     | |_____- in this macro invocation
[INFO] [stderr]     |
[INFO] [stderr]     = help: message: `#[custom_slice(error(type = "..."))]` should be specified
