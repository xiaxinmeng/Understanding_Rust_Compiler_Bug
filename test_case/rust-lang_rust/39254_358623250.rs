rust
let machine = true;
// span_to_snippet sets `machine` to false if it ends up using the fallback
let snippet1 = span_to_snippet(e.span, "<type>", &mut machine);
...
span_lint_with_suggestion_approximate(cx, ....., machine)
