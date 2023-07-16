
warning: `[exhaustiveness]` cannot be resolved, ignoring it.
 --> src/librustc_mir_build/hair/pattern/_match.rs:7:69
  |
7 | //! (a) the patterns cover every possible constructor for the type [exhaustiveness]
  |                                                                     ^^^^^^^^^^^^^^ cannot be resolved, ignoring
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: `[usefulness]` cannot be resolved, ignoring it.
 --> src/librustc_mir_build/hair/pattern/_match.rs:8:36
  |
8 | //! (b) each pattern is necessary [usefulness]
  |                                    ^^^^^^^^^^ cannot be resolved, ignoring
  |
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: could not parse code block as Rust code
   --> src/librustc_mir_build/hair/pattern/_match.rs:113:9
    |
113 |   //!        For example, the first pattern above initially gives a stack `[(Some(true), _)]`. If we
    |  _________^
114 | | //!        pop the tuple constructor, we are left with `[Some(true), _]`, and if we then pop the
115 | | //!        `Some` constructor we get `[true, _]`. If we had popped `None` instead, we would get
116 | | //!        nothing back.
...   |
149 | | //!     Note that the OR-patterns are not always used directly in Rust, but are used to derive the
150 | | //!     exhaustive integer matching rules, so they're written here for posterity.
    | |_________________________________________________________________________________^
    |
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: character literal may only contain one codepoint
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: \u{2260}
    = note: error from rustc: unterminated character literal

warning: could not parse code block as Rust code
   --> src/librustc_mir_build/hair/pattern/_match.rs:179:9
    |
179 |   //!           For example, if `P` is:
    |  _________^
180 | | //!           [
181 | | //!               [Some(true), _],
182 | | //!               [None, 0],
...   |
229 | | //!           `U(P, p) := U(P, (r_1, p_2, .., p_n))
230 | | //!                    || U(P, (r_2, p_2, .., p_n))`
    | |____________________________________________________^
    |
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: \u{2203}
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `
    = note: error from rustc: unknown start of token: `

warning: 4 warnings emitted
