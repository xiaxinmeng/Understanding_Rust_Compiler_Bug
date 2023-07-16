
error: expected one of `,`, `::`, or `as`, found `-`
  --> test.rs:17:10
   |
17 | use x::{A- B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`
   |          ^ expected one of `,`, `::`, or `as` here

error: expected one of `*`, `::`, `;`, `as`, or `{`, found `-`
  --> test.rs:17:10
   |
17 | use x::{A- B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`
   |          ^ expected one of `*`, `::`, `;`, `as`, or `{` here

error: aborting due to 2 previous errors