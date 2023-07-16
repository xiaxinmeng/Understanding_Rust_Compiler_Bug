 rust
bjz.rs:10:51: 10:66 error: associated type `Resources` not found for type parameter `Self` [E0220]
bjz.rs:10     type CommandBuffer: CommandBuffer<Resources = Self::Resources>;
                                                            ^~~~~~~~~~~~~~~
bjz.rs:10:39: 10:67 error: no associated type `Resources` defined in `CommandBuffer` [E0218]
bjz.rs:10     type CommandBuffer: CommandBuffer<Resources = Self::Resources>;
                                                ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
