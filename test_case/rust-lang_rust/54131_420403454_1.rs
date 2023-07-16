rust
{ 
    let x = &foo(); // <-- temporary is created here
    ...
} // <-- and then freed here
