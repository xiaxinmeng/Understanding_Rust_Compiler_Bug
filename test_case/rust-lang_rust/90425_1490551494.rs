rust
let bar_result =
{
    let self2 = &mut self;             // self2: lifetime starts here
    Self::bar(self2)                   // self2: lifetime ends here
}
let foo_result = 
{
    let self1 = &mut self;             // self1: lifetime starts here
    Self::foo(self1, bar_result)       // self1: lifetime ends here
}
