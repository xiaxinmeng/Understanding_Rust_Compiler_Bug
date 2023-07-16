rust

error: expected one of `,`, `.`, `?`, or an operator, found `;`
  --> $DIR/token-error-correct-3.rs:23:35
   |
23 |             callback(path.as_ref();  //~ NOTE: unclosed delimiter
   |                                   ^ expected one of `,`, `.`, `?`, or an operator here

