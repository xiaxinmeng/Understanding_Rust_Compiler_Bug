rust
{
    #[deny(unfulfilled_lint_expectations)]
    #[expect(unreachable_code)]
    {
        unreachable!()
    }
}; // < unreachable statement!
