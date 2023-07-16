rust
pub fn parse() -> Foo { <not_poisoned>
    let args: Vec<String> = env::args().collect();
    let text = args[1].clone();
    
    pub Foo { <poisoned> text </poisoned> }
</not_poisoned> }
