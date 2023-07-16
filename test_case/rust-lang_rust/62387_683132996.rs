`rust
use std::path::PathBuf;

struct Container {
    things: Vec<PathBuf>,
}

impl Container {
    fn things(&mut self) -> &[PathBuf] {
        &self.things
    }
}

// contains containers
struct ContainerContainer {
    contained: Vec<Container>
}

impl ContainerContainer {
    fn contained(&self ) -> &[Container] {
        &self.contained
    }

    fn all_the_things(&mut self) -> &[PathBuf]
    {
       let a = &self.contained().iter().flat_map(|container| container.things()).cloned().collect::<Vec<PathBuf>>();
        unimplemented!();
    }
}

pub fn main() {}
