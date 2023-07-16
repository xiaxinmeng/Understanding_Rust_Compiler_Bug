plain
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: `#[error(...)]` is not a valid attribute
   |
   |
10 | #[error(ast_passes::forbidden_let)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
10 | #[error(ast_passes::forbidden_let)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
34 | #[error(ast_passes::forbidden_assoc_constraint)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
34 | #[error(ast_passes::forbidden_assoc_constraint)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
41 | #[error(ast_passes::keyword_lifetime)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
41 | #[error(ast_passes::keyword_lifetime)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
48 | #[error(ast_passes::invalid_label)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
48 | #[error(ast_passes::invalid_label)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
56 | #[error(ast_passes::invalid_visibility, code = "E0449")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
56 | #[error(ast_passes::invalid_visibility, code = "E0449")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
75 | #[error(ast_passes::trait_fn_async, code = "E0706")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
75 | #[error(ast_passes::trait_fn_async, code = "E0706")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
86 | #[error(ast_passes::trait_fn_const, code = "E0379")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
86 | #[error(ast_passes::trait_fn_const, code = "E0379")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
94 | #[error(ast_passes::forbidden_lifetime_bound)]
   | ^
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
94 | #[error(ast_passes::forbidden_lifetime_bound)]
   | ^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
101 | #[error(ast_passes::forbidden_non_lifetime_param)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
101 | #[error(ast_passes::forbidden_non_lifetime_param)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[fatal(...)]` is not a valid attribute
    |
    |
108 | #[fatal(ast_passes::fn_param_too_many)]
    |
    |
    = help: only `diag`, `help`, `note` and `warn_` are valid attributes

error: diagnostic slug not specified
    |
    |
108 | #[fatal(ast_passes::fn_param_too_many)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
116 | #[error(ast_passes::fn_param_c_var_args_only)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
116 | #[error(ast_passes::fn_param_c_var_args_only)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
123 | #[error(ast_passes::fn_param_c_var_args_not_last)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
123 | #[error(ast_passes::fn_param_c_var_args_not_last)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
130 | #[error(ast_passes::fn_param_doc_comment)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
130 | #[error(ast_passes::fn_param_doc_comment)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
138 | #[error(ast_passes::fn_param_forbidden_attr)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
138 | #[error(ast_passes::fn_param_forbidden_attr)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
145 | #[error(ast_passes::fn_param_forbidden_self)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
145 | #[error(ast_passes::fn_param_forbidden_self)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
154 | #[error(ast_passes::forbidden_default)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
154 | #[error(ast_passes::forbidden_default)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
163 | #[error(ast_passes::assoc_const_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
163 | #[error(ast_passes::assoc_const_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
172 | #[error(ast_passes::assoc_fn_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
172 | #[error(ast_passes::assoc_fn_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
181 | #[error(ast_passes::assoc_type_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
181 | #[error(ast_passes::assoc_type_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
190 | #[error(ast_passes::const_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
190 | #[error(ast_passes::const_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
199 | #[error(ast_passes::static_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
199 | #[error(ast_passes::static_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
208 | #[error(ast_passes::ty_alias_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
208 | #[error(ast_passes::ty_alias_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
    |
    |
217 | #[error(ast_passes::fn_without_body)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
217 | #[error(ast_passes::fn_without_body)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:10:3
   |
   |
10 | #[error(ast_passes::forbidden_let)]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:34:3
   |
   |
34 | #[error(ast_passes::forbidden_assoc_constraint)]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:41:3
   |
   |
41 | #[error(ast_passes::keyword_lifetime)]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:48:3
   |
   |
48 | #[error(ast_passes::invalid_label)]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:56:3
   |
   |
56 | #[error(ast_passes::invalid_visibility, code = "E0449")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:75:3
   |
   |
75 | #[error(ast_passes::trait_fn_async, code = "E0706")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:86:3
   |
   |
86 | #[error(ast_passes::trait_fn_const, code = "E0379")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_ast_passes/src/errors.rs:94:3
   |
   |
94 | #[error(ast_passes::forbidden_lifetime_bound)]
   |   ^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:101:3
    |
101 | #[error(ast_passes::forbidden_non_lifetime_param)]


error: cannot find attribute `fatal` in this scope
    |
    |
108 | #[fatal(ast_passes::fn_param_too_many)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:116:3
    |
    |
116 | #[error(ast_passes::fn_param_c_var_args_only)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:123:3
    |
    |
123 | #[error(ast_passes::fn_param_c_var_args_not_last)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:130:3
    |
    |
130 | #[error(ast_passes::fn_param_doc_comment)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:138:3
    |
    |
138 | #[error(ast_passes::fn_param_forbidden_attr)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:145:3
    |
    |
145 | #[error(ast_passes::fn_param_forbidden_self)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:154:3
    |
    |
154 | #[error(ast_passes::forbidden_default)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:163:3
    |
    |
163 | #[error(ast_passes::assoc_const_without_body)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:172:3
    |
    |
172 | #[error(ast_passes::assoc_fn_without_body)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:181:3
    |
    |
181 | #[error(ast_passes::assoc_type_without_body)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:190:3
    |
    |
190 | #[error(ast_passes::const_without_body)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:199:3
    |
    |
199 | #[error(ast_passes::static_without_body)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_ast_passes/src/errors.rs:208:3
    |
