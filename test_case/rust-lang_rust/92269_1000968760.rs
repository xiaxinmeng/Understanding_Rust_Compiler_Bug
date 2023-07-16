plain
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0596]: cannot borrow `real_params` as mutable, as it is not declared as mutable
   --> src/librustdoc/html/format.rs:227:58
225 |             let real_params =
225 |             let real_params =
    |                 ----------- help: consider changing this to be mutable: `mut real_params`
226 |                 self.params.iter().filter(|p| !p.is_synthetic_type_param());
227 |             let first_param = if let Some(first_param) = real_params.next() {
    |                                                          ^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:26
