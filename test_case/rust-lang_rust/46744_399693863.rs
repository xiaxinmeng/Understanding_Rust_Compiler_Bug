rust
use std::io;

enum Tree {
    Leaf(
        Vec<u8>
    ),
    Branch(
        Vec<BlockRef>,
    ),
}

struct Block {
    tree: Tree
}

enum BlockRef {
    StoreRef(Vec<u8>),
    Memory(Block),
}


trait Blockstore {
    fn store(&mut self, block: Block) -> io::Result<Block>;
}

struct FilesystemStore {
}

impl FilesystemStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self{})
    }
}

impl Blockstore for FilesystemStore {
    fn store(&mut self, block: Block) -> io::Result<Block> {
    }
}

fn main() {
    let some_file = include_bytes!("main.rs");
}

