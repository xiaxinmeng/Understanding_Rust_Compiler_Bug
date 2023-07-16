plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0423]: expected value, found built-in attribute `link`
     |
     |
1213 |                 let start = link.as_ptr() as usize - md.as_ptr() as usize;
     |
help: a local variable with a similar name exists
     |
     |
1213 |                 let start = links.as_ptr() as usize - md.as_ptr() as usize;
help: consider importing one of these items instead
     |
     |
20   | use crate::clean::auto_trait::auto_trait::error_reporting::infer::outlives::obligations::infer::mir::interpret::error::tls::query::job::thread::panicking::backtrace::backtrace_rs::symbolize::gimli::mystd::sys::fs::link;
     |
20   | use crate::core::source_map::symbol::sym_generated::link;
20   | use rustc_span::sym::link;
     |


error[E0423]: expected value, found built-in attribute `link`
     |
     |
1214 |                 start..start + link.len()
     |
help: a local variable with a similar name exists
     |
     |
1214 |                 start..start + links.len()
help: consider importing one of these items instead
     |
     |
20   | use crate::clean::auto_trait::auto_trait::error_reporting::infer::outlives::obligations::infer::mir::interpret::error::tls::query::job::thread::panicking::backtrace::backtrace_rs::symbolize::gimli::mystd::sys::fs::link;
     |
20   | use crate::core::source_map::symbol::sym_generated::link;
20   | use rustc_span::sym::link;
     |

error[E0057]: this function takes 2 arguments but 3 arguments were supplied
error[E0057]: this function takes 2 arguments but 3 arguments were supplied
    --> src/librustdoc/html/markdown.rs:1216:17
     |
1216 |                 span_for_link(kind, &dest, ev.1)
     |                 |
     |                 expected 2 arguments

error: aborting due to 3 previous errors
