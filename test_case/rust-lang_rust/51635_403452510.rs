rust
#![feature(proc_macro, wasm_import_module, wasm_custom_section)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const MEM_SIZE: usize = 1024 * 1024;
const BLOCK_SIZE: usize = 1024;
const NUM_BLOCKS: usize = MEM_SIZE / BLOCK_SIZE;

struct FAT
{
    memory: [[u8; BLOCK_SIZE]; NUM_BLOCKS]
}

#[wasm_bindgen]
impl FAT
{
    pub fn new() -> FAT
    {
        FAT
        {
            memory: [[0; BLOCK_SIZE], NUM_BLOCKS]
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_FAT()
    {
        let newFS = FAT::new();
        for blk in 0..NUM_BLOCKS
        {
            for b in 0..BLOCK_SIZE
            {
                assert_eq!(newFS.memory[blk][b], 0);
            }
        }
    }
}
