
if Example { // Example is some value. The parser doesn't have type information to know it isn't.
    a  // a block with a tail expression and no statements, `a` is an expression consisting of a single binding,
    :   // annotated with type ascription
    one() // ascribed to the "type" `one` which would *have* to be a trait with no type paremeters in parenthetical notation (like `FnMut()`)
}.is_pos() // a method call on whatever the result of the previous `if` is, which *has* to be `()` because it has no `else` arm
{ // oh no! a token that can follow an expression, you might have wanted to end a statement with `;`
