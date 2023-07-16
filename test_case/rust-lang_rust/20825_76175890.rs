
foo.rs:5:40: 5:51 error: associated type `Input` not found for type parameter `Self` [E0220]
foo.rs:5 pub trait Processor : Subscriber<Input=Self::Input> {
