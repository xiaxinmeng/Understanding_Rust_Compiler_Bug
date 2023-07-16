 rust
pub trait Resources {}

pub trait CommandBuffer {
    type Resouces: Resources;
}

pub trait Device {
    type CommandBuffer: CommandBuffer<Resources = Self::Resources>;
}
