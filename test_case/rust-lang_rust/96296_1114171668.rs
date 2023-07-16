rust
    'a: loop { break };
    'a: loop { break }; // warning: label name `'a` shadows a label name that is already in scope
