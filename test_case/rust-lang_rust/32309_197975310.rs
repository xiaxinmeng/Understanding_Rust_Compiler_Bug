
// define the lint and register it:
librustc/lint/builtin.rs:140:9:    pub MATCH_OF_UNIT_VARIANT_VIA_PAREN_DOTDOT,
librustc/lint/builtin.rs:184:13:            MATCH_OF_UNIT_VARIANT_VIA_PAREN_DOTDOT,

// define the forward compat details
librustc_lint/lib.rs:170:28:            id: LintId::of(MATCH_OF_UNIT_VARIANT_VIA_PAREN_DOTDOT),

// issue the lint
librustc_typeck/check/_match.rs:605:38:        sess.add_lint(lint::builtin::MATCH_OF_UNIT_VARIANT_VIA_PAREN_DOTDOT,
