
error: expected one of `,`, `->`, `::`, or `>`, found `;`
 --> test.rs:6:31
  |
6 |     bar::<<T as Foo>::Output();
  |                               ^
  |                               |
  |                               expected one of `,`, `->`, `::`, or `>` here
  |                               help: likely there's a missing `>` here
