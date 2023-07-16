Rust
switch command {
    Command::A => {
        storagelive(_name);
        _name = (command as Command::A).0;
    }
    Command::B = {}
}
drop(_name);
storagedead(_name);
