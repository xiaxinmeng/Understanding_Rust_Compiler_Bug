`rust
struct FileSystem;

impl FileSystem {
    fn build<'a>(
        &mut self,
        commands: impl Iterator<Item = &'a str> + 'a,
    ) -> Option<impl Iterator<Item = &'a str> + 'a> {
        let further_commands = self.build(commands);
        self.build(further_commands?)
    }
}

fn main() {}