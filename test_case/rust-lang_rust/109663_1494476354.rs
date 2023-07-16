plain
---- [ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs stdout ----

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:84: unexpected error: '84:9: 84:12: invalid nested attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:94: unexpected error: '94:9: 94:14: unexpected literal in nested attribute, expected ident'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:104: unexpected error: '104:9: 104:13: invalid nested attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:114: unexpected error: '114:9: 114:13: invalid nested attribute'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:134: unexpected error: '134:9: 134:10: unexpected end of input, unexpected token in nested attribute, expected ident'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:143: unexpected error: '143:27: 143:31: invalid nested attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:152: unexpected error: '152:27: 152:40: invalid nested attribute'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:223: unexpected error: '223:13: 223:17: invalid nested attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:325: unexpected error: '325:44: 325:45: a diagnostic slug must be the first argument to the attribute'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:538: unexpected error: '538:42: 538:46: invalid nested attribute'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:556: unexpected error: '556:23: 556:24: unexpected end of input, unexpected token in nested attribute, expected ident'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:579: unexpected error: '579:23: 579:26: `code` is the only valid nested attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:576: unexpected error: '576:23: 576:24: unexpected end of input, unexpected token in nested attribute, expected ident'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:579: unexpected error: '579:27: 579:28: expected `,`'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:670: unexpected error: '670:28: 670:33: unexpected token'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:679: unexpected error: '679:28: 679:33: unexpected token'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:688: unexpected error: '688:28: 688:29: unexpected token'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:706: unexpected error: '706:30: 706:31: expected string literal'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:781: unexpected error: '781:49: 781:50: expected `= "xxx"`'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:789: unexpected error: '789:48: 789:49: a diagnostic slug must be the first argument to the attribute'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:797: unexpected error: '797:48: 797:49: expected `= "xxx"`'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:797: unexpected error: '797:48: 797:49: expected `,`'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:703: unexpected error: '703:10: 703:23: cannot find value `__code_29` in this scope [E0425]'

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:703: unexpected note: 'in this expansion of #[derive(Subdiagnostic)]'
error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:703: unexpected note: 'not found in this scope'


error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:84: expected error not found: `#[label(bug = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:94: expected error not found: `#[label("...")]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:94: expected error not found: diagnostic slug must be first argument

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:104: expected error not found: `#[label(slug = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:114: expected error not found: `#[label(slug(...))]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:134: expected error not found: diagnostic slug must be first argument of a `#[label(...)]` attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:143: expected error not found: `#[label(code = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:152: expected error not found: `#[label(applicability = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:223: expected error not found: `#[label(code = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:325: expected error not found: `#[label(no_crate::example)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:538: expected error not found: `#[multipart_suggestion(code = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:556: expected error not found: `#[suggestion_part(...)]` attribute without `code = "..."`

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:576: expected error not found: `#[suggestion_part(...)]` attribute without `code = "..."`

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:579: expected error not found: `#[suggestion_part(foo = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:706: expected error not found: `code = "..."`/`code(...)` must contain only string literals

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:781: expected error not found: `#[suggestion(style = ...)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:789: expected error not found: `#[suggestion(style)]` is not a valid attribute

error: /checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:797: expected error not found: `#[suggestion(style(...))]` is not a valid attribute
error: 25 unexpected errors found, 18 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive/auxiliary"
    Error {
        line_num: 84,
        kind: Some(
            Error,
---
        line_num: 94,
        kind: Some(
            Error,
        ),
        msg: "94:9: 94:14: unexpected literal in nested attribute, expected ident",
    Error {
        line_num: 104,
        kind: Some(
            Error,
---
        line_num: 134,
        kind: Some(
            Error,
        ),
        msg: "134:9: 134:10: unexpected end of input, unexpected token in nested attribute, expected ident",
    Error {
        line_num: 143,
        kind: Some(
            Error,
---
        line_num: 325,
        kind: Some(
            Error,
        ),
        msg: "325:44: 325:45: a diagnostic slug must be the first argument to the attribute",
    Error {
        line_num: 538,
        kind: Some(
            Error,
---
        line_num: 556,
        kind: Some(
            Error,
        ),
        msg: "556:23: 556:24: unexpected end of input, unexpected token in nested attribute, expected ident",
    Error {
        line_num: 579,
        kind: Some(
            Error,
            Error,
        ),
        msg: "579:23: 579:26: `code` is the only valid nested attribute",
    Error {
        line_num: 576,
        kind: Some(
            Error,
            Error,
        ),
        msg: "576:23: 576:24: unexpected end of input, unexpected token in nested attribute, expected ident",
    Error {
        line_num: 579,
        kind: Some(
            Error,
---
        line_num: 781,
        kind: Some(
            Error,
        ),
        msg: "781:49: 781:50: expected `= \"xxx\"`",
    Error {
        line_num: 789,
        kind: Some(
            Error,
            Error,
        ),
        msg: "789:48: 789:49: a diagnostic slug must be the first argument to the attribute",
    Error {
        line_num: 797,
        kind: Some(
            Error,
            Error,
        ),
        msg: "797:48: 797:49: expected `= \"xxx\"`",
    Error {
        line_num: 797,
        kind: Some(
            Error,
---
        line_num: 703,
        kind: Some(
            Error,
        ),
        msg: "703:10: 703:23: cannot find value `__code_29` in this scope [E0425]",
    Error {
        line_num: 703,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of #[derive(Subdiagnostic)]",
    Error {
        line_num: 703,
        kind: Some(
            Note,
---
        line_num: 84,
        kind: Some(
            Error,
        ),
        msg: "`#[label(bug = ...)]` is not a valid attribute",
    Error {
        line_num: 94,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(\"...\")]` is not a valid attribute",
    Error {
        line_num: 94,
        kind: Some(
            Error,
            Error,
        ),
        msg: "diagnostic slug must be first argument",
    Error {
        line_num: 104,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(slug = ...)]` is not a valid attribute",
    Error {
        line_num: 114,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(slug(...))]` is not a valid attribute",
    Error {
        line_num: 134,
        kind: Some(
            Error,
            Error,
        ),
        msg: "diagnostic slug must be first argument of a `#[label(...)]` attribute",
    Error {
        line_num: 143,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(code = ...)]` is not a valid attribute",
    Error {
        line_num: 152,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(applicability = ...)]` is not a valid attribute",
    Error {
        line_num: 223,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(code = ...)]` is not a valid attribute",
    Error {
        line_num: 325,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[label(no_crate::example)]` is not a valid attribute",
    Error {
        line_num: 538,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[multipart_suggestion(code = ...)]` is not a valid attribute",
    Error {
        line_num: 556,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion_part(...)]` attribute without `code = \"...\"`",
    Error {
        line_num: 576,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion_part(...)]` attribute without `code = \"...\"`",
    Error {
        line_num: 579,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion_part(foo = ...)]` is not a valid attribute",
    Error {
        line_num: 706,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`code = \"...\"`/`code(...)` must contain only string literals",
    Error {
        line_num: 781,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion(style = ...)]` is not a valid attribute",
    Error {
        line_num: 789,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion(style)]` is not a valid attribute",
    Error {
        line_num: 797,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`#[suggestion(style(...))]` is not a valid attribute",
]

thread '[ui] tests/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
