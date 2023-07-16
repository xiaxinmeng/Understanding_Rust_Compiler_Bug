rust
#[derive(Subdiagnostic)]
pub enum OverflowingBinHexSub<'a> {
    #[suggestion(
        lint_suggestion,
        code = "{sans_suffix}{suggestion_ty}",
        applicability = "machine-applicable"
    )]
    Suggestion {
        #[primary_span]
        span: Span,
        suggestion_ty: &'a str,
        sans_suffix: &'a str,
    },
    #[help(lint_help)]
    Help { suggestion_ty: &'a str },
}
