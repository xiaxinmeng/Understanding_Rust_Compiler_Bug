
[00:40:50] failures:

[00:40:50] 

[00:40:50] ---- [ui] ui/issue-44406.rs stdout ----

[00:40:50] 	normalized stderr:

[00:40:50] error: expected identifier, found keyword `true`

[00:40:50]   --> $DIR/issue-44406.rs:18:10

[00:40:50]    |

[00:40:50] 18 |     foo!(true);

[00:40:50]    |          ^^^^

[00:40:50] 

[00:40:50] error: expected type, found keyword `true`

[00:40:50]   --> $DIR/issue-44406.rs:18:10

[00:40:50]    |

[00:40:50] 13 |         bar(baz: $rest)

[00:40:50]    |                - help: did you mean to use `;` here?

[00:40:50] ...

[00:40:50] 18 |     foo!(true);

[00:40:50]    |          ^^^^ expecting a type here because of type ascription

[00:40:50] 

[00:40:50] error: expected one of `!`, `&&`, `&`, `(`, `*`, `.`, `;`, `<`, `?`, `[`, `_`, `dyn`, `extern`, `fn`, `for`, `impl`, `unsafe`, `}`, an operator, or lifetime, found `true`

[00:40:50]   --> $DIR/issue-44406.rs:18:10

[00:40:50]    |

[00:40:50] 13 |         bar(baz: $rest)

[00:40:50]    |                 - expected one of 20 possible tokens here

[00:40:50] ...

[00:40:50] 18 |     foo!(true);

[00:40:50]    |          ^^^^ unexpected token

[00:40:50] 

[00:40:50] error: aborting due to 3 previous errors
