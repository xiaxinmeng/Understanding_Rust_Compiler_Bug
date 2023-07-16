
unreachable!();          // panic message: "internal error: entered unreachable code"
unreachable!(123);       // panic message: "internal error: entered unreachable code: 123"
unreachable!("{}");      // panic message: "internal error: entered unreachable code: {}"
unreachable!("{}", 123); // panic message: "internal error: entered unreachable code: 123"
