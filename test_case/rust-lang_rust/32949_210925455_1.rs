 rust
{ // "lifetime scope"
    let x;
    {  // "initializer scope"
        tmp1 = 3 + 4;
        x = tmp1;
    }
    {  // scope within which x is visible, i.e. "remainder scope" of the let binding
        <subsequent statements>
    }
}
