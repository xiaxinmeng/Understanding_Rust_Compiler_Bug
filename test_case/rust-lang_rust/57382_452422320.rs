
error: expected item after doc comment
  --> $DIR/doc-before-extern-rbrace.rs:2:5
   |
LL |     /// hi
   |     ^^^^^^ this doc comment doesn't document anything

error: expected one of `crate`, `fn`, `pub`, `static`, or `type`, found `}`
  --> $DIR/doc-before-extern-rbrace.rs:4:1
   |
LL |     /// hi
   |           - expected one of `crate`, `fn`, `pub`, `static`, or `type` here
LL |     //~^ ERROR expected item after doc comment
LL | }
   | ^ unexpected token

error: aborting due to 2 previous errors
