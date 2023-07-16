
[01:00:24]    Compiling error_index_generator v0.0.0 (file:///checkout/src/tools/error_index_generator)
[01:00:25] error[E0432]: unresolved import `rustdoc::html::markdown::RenderType`
[01:00:25]   --> tools/error_index_generator/main.rs:27:53
[01:00:25]    |
[01:00:25] 27 | use rustdoc::html::markdown::{Markdown, PLAYGROUND, RenderType};
[01:00:25]    |                                                     ^^^^^^^^^^ no `RenderType` in `html::markdown`
[01:00:25] 
[01:00:25] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:00:25]    --> tools/error_index_generator/main.rs:103:52
[01:00:25]     |
[01:00:25] 103 |             Some(ref desc) => write!(output, "{}", Markdown(desc, &[], RenderType::Hoedown))?,
[01:00:25]     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters
[01:00:25] 
[01:00:25] error: aborting due to 2 previous errors
[01:00:25] 
[01:00:25] error: Could not compile `error_index_generator`.
