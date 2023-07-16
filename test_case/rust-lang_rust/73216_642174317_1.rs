rust
    to.extend(from.as_ref().iter().map(|value| { 
        let value: &dyn Debug = value;
        value
    }));
