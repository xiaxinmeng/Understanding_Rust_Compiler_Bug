 rust
pub trait Resources {}

pub trait CommandBuffer {
    type Resources: Resources;
}

pub trait Device {
    type Resources: Resources;
    type CommandBuffer: CommandBuffer<Resources = Self::Resources>;
}
