rust
let self1 = &mut self;             // self1: lifetime starts here
let self2 = &mut self;             // self2: lifetime starts here
let bar_result = Self::bar(self2); // self2: lifetime ends here
Self::foo(arg1, bar_result);       // self1: lifetime ends here
