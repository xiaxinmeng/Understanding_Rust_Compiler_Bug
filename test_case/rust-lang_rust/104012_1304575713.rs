console
    --> e3.rs:2605:3
     |
38   | impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
     |                                 - unclosed delimiter
...
110  |                                  sugg_span: Span| {
     |                                                   - this delimiter might not be properly closed...
...
240  |         };
     |         - ...as it matches this but it has different indentation
...
2605 | }
     |   ^

error: expected expression, found `let` statement
    --> e3.rs:1421:12
     |
1421 |         && let Some((fields, substs)) =
     |            ^^^

