rust
    true && { 1 } > 0; // OK, { 1 } is an intermediate expression somewhere
    { 1 } > 0; // FAIL, expression { 1 } is a full statement, we expect a next statement after it
