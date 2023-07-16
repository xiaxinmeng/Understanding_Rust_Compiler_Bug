
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/codeAction CodeActionParams {
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    },
    context: CodeActionContext {
        diagnostics: [],
        only: None,
    },
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::assists_with_fixes::{{closure}}
  29: std::panicking::try
  30: rust_analyzer::handlers::handle_code_action
  31: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/codeAction CodeActionParams {
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 20,
            character: 8,
        },
        end: Position {
            line: 20,
            character: 8,
        },
    },
    context: CodeActionContext {
        diagnostics: [],
        only: None,
    },
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::assists_with_fixes::{{closure}}
  29: std::panicking::try
  30: rust_analyzer::handlers::handle_code_action
  31: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/semanticTokens/range SemanticTokensRangeParams {
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 111,
            character: 0,
        },
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Local::ty
  26: ide::syntax_highlighting::highlight::highlight_def
  27: ide::syntax_highlighting::highlight::element
  28: ide::syntax_highlighting::traverse
  29: ide::syntax_highlighting::highlight
  30: std::panicking::try
  31: rust_analyzer::handlers::handle_semantic_tokens_range
  32: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  33: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[Error - 3:56:00 PM] Request textDocument/semanticTokens/range failed.
  Message: server panicked: index out of bounds: the len is 1 but the index is 1
  Code: -32603 
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/codeAction CodeActionParams {
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 35,
            character: 69,
        },
        end: Position {
            line: 35,
            character: 69,
        },
    },
    context: CodeActionContext {
        diagnostics: [],
        only: None,
    },
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::assists_with_fixes::{{closure}}
  29: std::panicking::try
  30: rust_analyzer::handlers::handle_code_action
  31: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/codeAction CodeActionParams {
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 33,
            character: 51,
        },
        end: Position {
            line: 33,
            character: 51,
        },
    },
    context: CodeActionContext {
        diagnostics: [],
        only: None,
    },
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::assists_with_fixes::{{closure}}
  29: std::panicking::try
  30: rust_analyzer::handlers::handle_code_action
  31: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/codeAction CodeActionParams {
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
    range: Range {
        start: Position {
            line: 44,
            character: 28,
        },
        end: Position {
            line: 44,
            character: 28,
        },
    },
    context: CodeActionContext {
        diagnostics: [],
        only: None,
    },
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::assists_with_fixes::{{closure}}
  29: std::panicking::try
  30: rust_analyzer::handlers::handle_code_action
  31: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::diagnostics
  29: rust_analyzer::handlers::publish_diagnostics
  30: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  31: alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Function::diagnostics
  26: hir::Module::diagnostics
  27: ide_diagnostics::diagnostics
  28: ide::Analysis::diagnostics
  29: rust_analyzer::handlers::publish_diagnostics
  30: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  31: alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
  32: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Panic context:
> 
version: 996300f4a 2021-08-23 stable
request: textDocument/semanticTokens/full SemanticTokensParams {
    work_done_progress_params: WorkDoneProgressParams {
        work_done_token: None,
    },
    partial_result_params: PartialResultParams {
        partial_result_token: None,
    },
    text_document: TextDocumentIdentifier {
        uri: Url {
            scheme: "file",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: None,
            port: None,
            path: "/home/paulg/Repositories/cachou/server/src/core/auth.rs",
            query: None,
            fragment: None,
        },
    },
}

thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.70.0/src/lib.rs:2752:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
   3: <&chalk_ir::SubstFolder<I,A> as chalk_ir::fold::Folder<I>>::fold_free_var_ty
   4: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   5: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::GenericArg<I>>::fold_with
   6: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
   7: chalk_ir::fold::boring_impls::<impl chalk_ir::fold::Fold<I> for chalk_ir::Substitution<I>>::fold_with
   8: <chalk_ir::Ty<I> as chalk_ir::fold::SuperFold<I>>::super_fold_with
   9: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce_inner
  10: hir_ty::infer::coerce::<impl hir_ty::infer::InferenceContext>::coerce
  11: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::check_call_arguments
  12: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  13: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  14: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  15: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_block
  16: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_inner
  17: hir_ty::infer::expr::<impl hir_ty::infer::InferenceContext>::infer_expr_coerce
  18: hir_ty::infer::infer_query
  19: salsa::runtime::Runtime::execute_query_implementation
  20: salsa::derived::slot::Slot<Q,MP>::read_upgrade
  21: salsa::derived::slot::Slot<Q,MP>::read
  22: <salsa::derived::DerivedStorage<Q,MP> as salsa::plumbing::QueryStorageOps<Q>>::try_fetch
  23: <DB as hir_ty::db::HirDatabase>::infer_query
  24: hir_ty::db::infer_wait
  25: hir::Local::ty
  26: ide::syntax_highlighting::highlight::highlight_def
  27: ide::syntax_highlighting::highlight::element
  28: ide::syntax_highlighting::traverse
  29: ide::syntax_highlighting::highlight
  30: std::panicking::try
  31: rust_analyzer::handlers::handle_semantic_tokens_full
  32: rust_analyzer::dispatch::RequestDispatcher::on::{{closure}}::{{closure}}
  33: <F as threadpool::FnBox>::call_box
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
[Error - 3:57:00 PM] Request textDocument/semanticTokens/full failed.
  Message: server panicked: index out of bounds: the len is 1 but the index is 1
  Code: -32603
