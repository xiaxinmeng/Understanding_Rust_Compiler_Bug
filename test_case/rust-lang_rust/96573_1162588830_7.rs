` always check that the code block compiles to align it to the `#[ignored] #[test]` semantics in a future edition, but this is a significantly larger change and *not* part of this PR, and is merely provided as an indication of the larger picture of making test annotations more consistent. See [this comment](https://github.com/rust-lang/rust/pull/96573#issuecomment-1115248925) for some comparison graphs.

So, the **actionable question to the reviewer**:

Does the above justify adding 