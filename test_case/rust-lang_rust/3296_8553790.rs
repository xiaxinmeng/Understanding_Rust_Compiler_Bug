
failstruct.rs:3:22: 3:25 error: obsolete syntax: class traits
failstruct.rs:3 struct Deserializer : std::serialization::deserializer{
                                      ^~~
note: implemented traits are specified on the impl, as in `impl foo : bar {`
failstruct.rs:5:4: 5:7 error: obsolete syntax: struct constructor
failstruct.rs:5     new() { self.x = (); }

...
