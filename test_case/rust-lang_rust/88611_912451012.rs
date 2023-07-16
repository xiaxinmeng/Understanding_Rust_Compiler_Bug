plain
...
1310 |     }
     |     ^ mismatched closing delimiter

error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
     |
1303 |         return find_matches_sugg(
     |                                 - unclosed delimiter
...
...
1309 |         );
     |          ^ help: `)` may belong here
error: expected expression, found `)`
    --> src/tools/clippy/clippy_lints/src/matches.rs:1310:5
     |
1310 |     }
---
   |
16 | use core::array;
   |     ^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unnecessary parentheses around function argument
    --> src/tools/clippy/clippy_lints/src/matches.rs:1306:37
     |
     |
1306 |             IntoIterator::into_iter(([(&[][..], Some(let_pat), if_then, None), (&[][..], None, if_else, None)]),
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
     |
     = note: `-D unused-parens` implied by `-D warnings`
error[E0061]: this function takes 5 arguments but 3 arguments were supplied
    --> src/tools/clippy/clippy_lints/src/matches.rs:1303:16
     |
1303 |           return find_matches_sugg(
1303 |           return find_matches_sugg(
     |                  ^^^^^^^^^^^^^^^^^ expected 5 arguments
1304 |               cx,
     |               --
1305 |               let_expr,
     |               --------
1306 | /             IntoIterator::into_iter(([(&[][..], Some(let_pat), if_then, None), (&[][..], None, if_else, None)]),
1308 | |             true,
1309 | |         );
     | |_________- supplied 3 arguments
     |
     |
note: function defined here
    --> src/tools/clippy/clippy_lints/src/matches.rs:1333:4
     |
1333 | fn find_matches_sugg<'a, 'b, I>(
     |    ^^^^^^^^^^^^^^^^^
1334 |     cx: &LateContext<'_>,
     |     --------------------
1335 |     ex: &Expr<'_>,
1336 |     mut iter: I,
     |     -----------
     |     -----------
1337 |     expr: &Expr<'_>,
1338 |     is_if_let: bool,
     |     ---------------

error[E0061]: this function takes 1 argument but 3 arguments were supplied
error[E0061]: this function takes 1 argument but 3 arguments were supplied
    --> src/tools/clippy/clippy_lints/src/matches.rs:1306:13
     |
1306 |             IntoIterator::into_iter(([(&[][..], Some(let_pat), if_then, None), (&[][..], None, if_else, None)]),
     |             |
     |             expected 1 argument
1307 |             expr,
     |             ----
---
     |
1303 | /         return find_matches_sugg(
1304 | |             cx,
1305 | |             let_expr,
1306 | |             IntoIterator::into_iter(([(&[][..], Some(let_pat), if_then, None), (&[][..], None, if_else, None)]),
1308 | |             true,
1309 | |         );
     | |_________- any code following this expression is unreachable
1310 |       }
1310 |       }
     |       ^ unreachable expression
     |
     = note: `-D unreachable-code` implied by `-D warnings`
For more information about this error, try `rustc --explain E0061`.
error: could not compile `clippy_lints` due to 8 previous errors
Build completed unsuccessfully in 0:04:17
